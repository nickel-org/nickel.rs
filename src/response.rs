use http;
use http::headers::content_type::MediaType;
use time;

///A container for the response
pub struct Response<'a, 'b> {
    ///the original `http::server::ResponseWriter`
    pub origin: &'a mut http::server::ResponseWriter<'b>,
}

impl<'a, 'b> Response<'a, 'b> {
    pub fn write (&mut self, text: &str) {
        // TODO: This needs to be more sophisticated to return the correct headers
        // not just "some headers" :)
        Response::set_headers(self.origin);
        self.origin.write(text.as_bytes());
    }

    fn set_headers(response_writer: &mut http::server::ResponseWriter) {
        response_writer.headers.date = Some(time::now_utc());

        // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
        response_writer.headers.content_length = None;
        response_writer.headers.content_type = Some(MediaType {
            type_: String::from_str("text"),
            subtype: String::from_str("plain"),
            parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
        });
        response_writer.headers.server = Some(String::from_str("Example"));
    }
}