use std::io::{IoResult, File};
use std::io::util::copy;
use http;
use http::headers::content_type::MediaType;
use time;
use mimes::get_media_type;

///A container for the response
pub struct Response<'a, 'b> {
    ///the original `http::server::ResponseWriter`
    pub origin: &'a mut http::server::ResponseWriter<'b>,
}

impl<'a, 'b> Response<'a, 'b> {

    pub fn from_internal<'c, 'd>(response: &'c mut http::server::ResponseWriter<'d>) -> Response<'c, 'd> {
        Response {
            origin: response
        }
    }

    /// Writes a response
    ///
    /// # Example
    /// ```rust
    /// response.send("hello world");
    /// ```
    pub fn send (&mut self, text: &str) {
        // TODO: This needs to be more sophisticated to return the correct headers
        // not just "some headers" :)
        Response::set_headers(self.origin);
        let _ = self.origin.write(text.as_bytes());
    }

    fn set_headers(response_writer: &mut http::server::ResponseWriter) {
        response_writer.headers.date = Some(time::now_utc());

        // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
        response_writer.headers.content_length = None;
        let has_set_content_type: bool = match response.origin.headers.content_type {
            None => false,
            Some(_) => true,
        };
        if !has_set_content_type {
            response_writer.headers.content_type = Some(MediaType {
                type_: String::from_str("text"),
                subtype: String::from_str("plain"),
                parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
            });
        }
        response_writer.headers.server = Some(String::from_str("Nickel"));
    }

    pub fn send_file(&mut self, path: &Path) -> IoResult<()> {
        let mut file = try!(File::open(path));
        self.origin.headers.content_length = None;

        self.origin.headers.content_type = path.extension_str().and_then(get_media_type);
        self.origin.headers.server = Some(String::from_str("Nickel"));
        copy(&mut file, self.origin)
    }
}

#[test]
fn matches_content_type () {
    let path = &Path::new("test.txt");
    let content_type = path.extension_str().and_then(get_media_type).unwrap();

    assert_eq!(content_type.type_.as_slice(), "text");
    assert_eq!(content_type.subtype.as_slice(), "plain");
}
