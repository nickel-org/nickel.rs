use std::borrow::Cow;
use std::ops::Deref;
use regex::Regex;

pub struct Matcher {
    pub path: Cow<'static, str>,
    pub regex: Regex
}

impl Matcher {
    pub fn new<P: Into<Cow<'static, str>>>(path: P, regex: Regex) -> Matcher {
        Matcher {
            path: path.into(),
            regex: regex
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Deref for Matcher {
    type Target = Regex;

    fn deref(&self) -> &Regex {
        &self.regex
    }
}
