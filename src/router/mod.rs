//!Router asigns handlers to paths and resolves them per request
pub use self::http_router::HttpRouter;
pub use self::request_handler::{RequestHandler, ResponseFinalizer};
pub use self::router::{Router, Route, RouteResult};
mod http_router;
mod request_handler;

mod router;

/// The path_utils collects some small helper methods that operate on the path
mod path_utils {
    use regex::Regex;
    use std::collections::hashmap::HashMap;

    // matches named variables (e.g. :userid)
    static REGEX_VAR_SEQ: Regex                 = regex!(r":([,a-zA-Z0-9_-]*)");
    static VAR_SEQ:&'static str                 = "[,a-zA-Z0-9_-]*";
    static VAR_SEQ_WITH_SLASH:&'static str      = "[,/a-zA-Z0-9_-]*";
    static VAR_SEQ_WITH_CAPTURE:&'static str    = "([,a-zA-Z0-9%_-]*)";
    // matches request params (e.g. ?foo=true&bar=false)
    static REGEX_PARAM_SEQ:&'static str         = "(\\?[a-zA-Z0-9%_=&-]*)?";
    static REGEX_START:&'static str             = "^";
    static REGEX_END:&'static str               = "$";
    pub fn create_regex (route_path: &str) -> Regex {
        let updated_path =
            route_path.to_string()
                      // first mark all double wildcards for replacement.
                      // We can't directly replace them since the replacement
                      // does contain the * symbol as well, which would get
                      // overwritten with the next replace call
                      .replace("**", "___DOUBLE_WILDCARD___")
                      // then replace the regular wildcard symbols (*) with the
                      // appropriate regex
                      .replace("*", VAR_SEQ)
                      // now replace the previously marked double wild cards (**)
                      .replace("___DOUBLE_WILDCARD___", VAR_SEQ_WITH_SLASH);

        // then replace the variable symbols (:variable) with the appropriate regex
        let result = [REGEX_START,
                      REGEX_VAR_SEQ.replace_all(updated_path.as_slice(),
                                                VAR_SEQ_WITH_CAPTURE)
                                   .as_slice(),
                      REGEX_PARAM_SEQ,
                      REGEX_END].concat();

        Regex::new(result.as_slice()).ok().unwrap()
    }

    pub fn get_variable_info (route_path: &str) -> HashMap<String, uint> {
        REGEX_VAR_SEQ.captures_iter(route_path)
             .enumerate()
             .map(|(i, matched)| (matched.at(1).to_string(), i))
             .collect()
    }
}
