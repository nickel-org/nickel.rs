use super::Matcher;
use regex::{Regex, Captures};

pub trait IntoMatcher {
    fn into_matcher(self) -> Matcher;
}

impl IntoMatcher for Regex {
    fn into_matcher(self) -> Matcher {
        let path = self.as_str().to_string();
        Matcher::new(path, self)
    }
}

impl<'a> IntoMatcher for &'a str {
    fn into_matcher(self) -> Matcher {
        self.to_string().into_matcher()
    }
}

lazy_static! {
    static ref REGEX_VAR_SEQ: Regex = Regex::new(r":([,a-zA-Z0-9_-]*)").unwrap();
}

pub static FORMAT_PARAM:      &'static str = "format";
// FIXME: Once const fn lands this could be defined in terms of the above
static FORMAT_VAR:            &'static str = ":format";
static VAR_SEQ:               &'static str = "[,a-zA-Z0-9_-]*";
static VAR_SEQ_WITH_SLASH:    &'static str = "[,/a-zA-Z0-9_-]*";
// matches request params (e.g. ?foo=true&bar=false)
static REGEX_PARAM_SEQ:       &'static str = "(\\?[a-zA-Z0-9%_=&-]*)?";

impl IntoMatcher for String {
    fn into_matcher(self) -> Matcher {
        let with_format = if self.contains(FORMAT_VAR) {
            self
        } else {
            format!("{}(\\.{})?", self, FORMAT_VAR)
        };

        // First mark all double wildcards for replacement. We can't directly
        // replace them since the replacement does contain the * symbol as well,
        // which would get overwritten with the next replace call
        let with_placeholder = with_format.replace("**", "___DOUBLE_WILDCARD___");

        // Then replace the regular wildcard symbols (*) with the appropriate regex
        let star_replaced = with_placeholder.replace("*", VAR_SEQ);

        // Now replace the previously marked double wild cards (**)
        let wildcarded = star_replaced.replace("___DOUBLE_WILDCARD___", VAR_SEQ_WITH_SLASH);

        // Add a named capture for each :(variable) symbol
        let named_captures = REGEX_VAR_SEQ.replace_all(&wildcarded, |captures: &Captures| {
            // There should only ever be one match (after subgroup 0)
            let c = captures.iter().skip(1).next().unwrap();
            format!("(?P<{}>[,a-zA-Z0-9%_-]*)", c.unwrap())
        });

        let line_regex = format!("^{}{}$", named_captures, REGEX_PARAM_SEQ);
        let regex = Regex::new(&line_regex).unwrap();
        Matcher::new(with_format, regex)
    }
}
