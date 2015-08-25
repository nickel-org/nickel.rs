use response::Response;
use request::Request;
use hyper::header::SetCookie as HyperSetCookie;
use hyper::header::Cookie;
use cookie::Cookie as CookiePair;

pub trait SetCookie {
    fn set_cookie_string_pair(&mut self, String, String);
    fn set_cookies(&mut self, Vec<CookiePair>);
}

pub trait GetCookies {
    fn get_cookies(&self) -> Vec<&CookiePair>;
}

impl<'a, 'b, 'c> GetCookies for Request<'a, 'b, 'c> {
    fn get_cookies(&self) -> Vec<&CookiePair> {
        match self.origin.headers.get::<Cookie>() {
            Some(c) => c.iter().collect(),
            None => vec![]
        }
    }
}

impl<'a> SetCookie for Response<'a> {
    fn set_cookie_string_pair(&mut self, key: String, val: String) {
        let cookie = CookiePair::new(key, val);
        self.headers_mut().set(HyperSetCookie(vec![cookie]));
    }

    fn set_cookies(&mut self, cookies: Vec<CookiePair>) {
        self.headers_mut().set(HyperSetCookie(cookies));
    }
}

