use http;

///A container for the response
pub struct Response<'a, 'b> {
    ///the original `http::server::ResponseWriter`
    pub origin: &'a mut http::server::ResponseWriter<'b>,
}
