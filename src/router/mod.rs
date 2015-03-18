//!Router asigns handlers to paths and resolves them per request
pub use self::http_router::HttpRouter;
pub use self::router::{Router, Route, RouteResult};

pub mod http_router;
pub mod router;

/// The path_utils collects some small helper methods that operate on the path
mod path_utils {
    use regex::{Regex, Captures};

    // matches named variables (e.g. :userid)
    lazy_static! {
        static ref REGEX_VAR_SEQ: Regex = Regex::new(r":([,a-zA-Z0-9_-]*)").unwrap();
    }
    static VAR_SEQ:               &'static str = "[,a-zA-Z0-9_-]*";
    static VAR_SEQ_WITH_SLASH:    &'static str = "[,/a-zA-Z0-9_-]*";

    // matches request params (e.g. ?foo=true&bar=false)
    static REGEX_PARAM_SEQ: &'static str = "(\\?[a-zA-Z0-9%_=&-]*)?";

    pub fn create_regex(route_path: &str) -> Regex {
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

        // Add a named capture for each :(variable) symbol
        let named_captures = REGEX_VAR_SEQ.replace_all(&updated_path, |captures: &Captures| {
            // There should only ever be one match (after subgroup 0)
            let c = captures.iter().skip(1).next().unwrap();
            format!("(?P<{}>[,a-zA-Z0-9%_-]*)", c.unwrap())
        });

        let line_regex = format!("^{}{}$", named_captures, REGEX_PARAM_SEQ);

        Regex::new(&line_regex).unwrap()
    }
}
