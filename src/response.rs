use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::old_io::{IoResult, File};
use std::old_io::util::copy;
use std::old_path::BytesContainer;
use serialize::Encodable;
use hyper::status::StatusCode;
use hyper::server::Response as HyperResponse;
use hyper::header;
use hyper::net::{Fresh, Streaming};
use time;
use mimes::{get_media_type, MediaType};
use mustache;
use mustache::Template;

pub type TemplateCache = RwLock<HashMap<&'static str, Template>>;

///A container for the response
pub struct Response<'a, 'b: 'a, T=Fresh> {
    ///the original `hyper::server::Response`
    pub origin: HyperResponse<'a, T>,
    templates: &'b TemplateCache
}

impl<'a, 'b> Response<'a, 'b, Fresh> {
    pub fn from_internal<'c, 'd>(response: HyperResponse<'c, Fresh>,
                                 templates: &'d TemplateCache)
                                -> Response<'c, 'd, Fresh> {
        Response {
            origin: response,
            templates: templates
        }
    }

    /// Sets the content type by it's short form.
    /// Returns the response for chaining.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// use nickel::mimes::MediaType;
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.content_type(MediaType::Html);
    /// }
    /// ```
    pub fn content_type(&mut self, mt: MediaType) -> &mut Response<'a,'b> {
        self.origin.headers_mut().set(header::ContentType(get_media_type(mt)));
        self
    }

    /// Sets the status code and returns the response for chaining
    ///
    /// # Example
    /// ```{rust}
    /// # extern crate hyper;
    /// # extern crate nickel;
    /// # use nickel::{Request, Response};
    /// # fn main() {
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.status_code(hyper::status::StatusCode::NotFound);
    /// }
    /// # }
    /// ```
    pub fn status_code(&mut self, status: StatusCode) -> &mut Response<'a,'b> {
        *self.origin.status_mut() = status;
        self
    }

    /// Writes a response
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("hello world");
    /// }
    /// ```
    pub fn send<T: BytesContainer> (self, text: T) -> IoResult<Response<'a, 'b, Streaming>> {
        self.set_common_headers();

        let mut stream = try!(self.start());
        try!(stream.write_all(text.container_as_bytes()));
        Ok(stream)
    }

    /// Writes a file to the output.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     response.send_file(&favicon).ok().expect("Failed to send favicon");
    /// }
    /// ```
    pub fn send_file(mut self, path: &Path) -> IoResult<Response<'a, 'b, Streaming>> {
        // Chunk the response
        self.origin.headers_mut().remove::<header::ContentLength>();
        // Determine content type by file extension or default to binary
        self.content_type(path.extension_str()
                              .and_then(from_str)
                              .unwrap_or(MediaType::Bin));

        let mut stream = try!(self.start());
        let mut file = try!(File::open(path));
        try!(copy(&mut file, &mut stream));
        Ok(stream)
    }

    // TODO: This needs to be more sophisticated to return the correct headers
    // not just "some headers" :)
    //
    // Also, it should only set them if not already set.
    fn set_common_headers(&mut self) {
        let ref mut headers = self.origin.headers_mut();
        headers.set(header::Date(time::now_utc()));
        headers.set(header::Server(String::from_str("Nickel")));
    }

    /// Renders the given template bound with the given data.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// # use std::collections::HashMap;
    /// fn handler(request: &Request, response: &mut Response) {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     response.render("examples/assets/template.tpl", &data);
    /// }
    /// ```
    pub fn render<T>(self, path: &'static str, data: &T)
            -> Result<Response<'a, 'b, Streaming>, Error>
            where T: Encodable<Encoder<'a>, Error> {
        // Fast path doesn't need writer lock
        if let Some(t) = self.templates.read().get(&path) {
            let mut stream = try!(self.start());
            try!(t.render(&mut stream, data));
            return Ok(stream);
        }

        // We didn't find the template, get writers lock
        let mut templates = self.templates.write();
        // Search again incase there was a race to compile the template
        let template = match templates.entry(path) {
            Vacant(entry) => {
                let mut file = File::open(&Path::new(path));
                let raw_template =
                    file.read_to_string()
                        .ok()
                        .expect(format!("Couldn't open the template file: {}",
                                        path).as_slice());

                entry.set(mustache::compile_str(raw_template.as_slice()))
            },
            Occupied(entry) => entry.into_mut()
        };

        let mut stream = try!(self.start());
        try!(template.render(&mut stream, data));
        Ok(stream)
    }

    pub fn start(mut self) -> IoResult<Response<'a, 'b, Streaming>> {
        self.set_common_headers();

        let Response { origin, templates } = self;
        let origin = try!(origin.start());

        Ok(Response { origin: origin, templates: templates })
    }
}

impl<'a, 'b> Writer for Response<'a, 'b, Streaming> {
    #[inline(always)]
    fn write(&mut self, msg: &[u8]) -> IoResult<()> {
        self.origin.write(msg)
    }
    #[inline(always)]
    fn flush(&mut self) -> IoResult<()> {
        self.origin.flush()
    }
}

impl<'a, 'b> Response<'a, 'b, Streaming> {
    /// Flushes all writing of a response to the client.
    pub fn end(self) -> IoResult<()> {
        self.origin.end()
    }
}

#[test]
fn matches_content_type () {
    use hyper::mime::{Mime, TopLevel, SubLevel};
    let path = &Path::new("test.txt");
    let content_type = path.extension_str().and_then(|s| s.parse().ok());

    assert_eq!(content_type, Some(MediaType::Txt));
    let content_type = content_type.map(get_media_type).unwrap();

    match content_type {
        Mime(TopLevel::Text, SubLevel::Plain, _) => {}, // OK
        wrong => panic!("Wrong mime: {}", wrong)
    }
}
