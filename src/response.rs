use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::old_io::{IoResult, File, IoError, IoErrorKind};
use std::old_io::util::copy;
use std::old_path::BytesContainer;
use std::fmt::Debug;
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
pub struct Response<'a, T=Fresh> {
    ///the original `hyper::server::Response`
    pub origin: HyperResponse<'a, T>,
    templates: &'a TemplateCache
}

impl<'a> Response<'a, Fresh> {
    pub fn from_internal<'c, 'd>(response: HyperResponse<'c, Fresh>,
                                 templates: &'c TemplateCache)
                                -> Response<'c, Fresh> {
        Response {
            origin: response,
            templates: templates
        }
    }

    /// Sets the content type by it's short form.
    /// Returns the response for chaining.
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult, Continue};
    /// use nickel::mimes::MediaType;
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     res.content_type(MediaType::Html);
    ///     Ok(Continue(res))
    /// }
    /// ```
    pub fn content_type(&mut self, mt: MediaType) -> &mut Response<'a> {
        self.origin.headers_mut().set(header::ContentType(get_media_type(mt)));
        self
    }

    /// Sets the status code and returns the response for chaining
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult, Continue};
    /// use nickel::status::StatusCode;
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     res.status_code(StatusCode::NotFound);
    ///     Ok(Continue(res))
    /// }
    /// ```
    pub fn status_code(&mut self, status: StatusCode) -> &mut Response<'a> {
        *self.origin.status_mut() = status;
        self
    }

    /// Writes a response
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult, Halt};
    ///
    /// fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    ///     Ok(Halt(try!(res.send("hello world"))))
    /// }
    /// ```
    pub fn send<T: BytesContainer> (mut self, text: T) -> IoResult<Response<'a Streaming>> {
        self.set_common_headers();

        let mut stream = try!(self.start());
        try!(stream.write_all(text.container_as_bytes()));
        Ok(stream)
    }

    /// Writes a file to the output.
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult, Halt};
    /// use nickel::status::StatusCode;
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     Ok(Halt(try!(res.send_file(&favicon))))
    /// }
    /// ```
    pub fn send_file(mut self, path: &Path) -> IoResult<Response<'a, Streaming>> {
        // Chunk the response
        self.origin.headers_mut().remove::<header::ContentLength>();
        // Determine content type by file extension or default to binary
        self.content_type(path.extension_str()
                              .and_then(|s| s.parse().ok())
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
    /// # Examples
    /// ```{rust}
    /// use std::collections::HashMap;
    /// use nickel::{Request, Response, MiddlewareResult, Halt};
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     let stream = try!(res.render("examples/assets/template.tpl", &data));
    ///     Ok(Halt(stream))
    /// }
    /// ```
    pub fn render<T>(self, path: &'static str, data: &T)
            -> IoResult<Response<'a, Streaming>>
            where T: Encodable {
        fn to_ioerr<U: Debug>(r: Result<(), U>) -> IoResult<()> {
            r.map_err(|e| IoError {
                kind: IoErrorKind::OtherIoError,
                desc: "Problem rendering template",
                detail: Some(format!("{:?}", e))
            })
        }

        // Fast path doesn't need writer lock
        if let Some(t) = self.templates.read().unwrap().get(&path) {
            let mut stream = try!(self.start());
            try!(to_ioerr(t.render(&mut stream, data)));
            return Ok(stream);
        }

        // We didn't find the template, get writers lock
        let mut templates = self.templates.write().unwrap();
        // Search again incase there was a race to compile the template
        let template = match templates.entry(path) {
            Vacant(entry) => {
                let mut file = File::open(&Path::new(path));
                let raw_template =
                    file.read_to_string()
                        .ok()
                        .expect(format!("Couldn't open the template file: {}",
                                        path).as_slice());

                entry.insert(mustache::compile_str(raw_template.as_slice()))
            },
            Occupied(entry) => entry.into_mut()
        };

        let mut stream = try!(self.start());
        try!(to_ioerr(template.render(&mut stream, data)));
        Ok(stream)
    }

    pub fn start(mut self) -> IoResult<Response<'a, Streaming>> {
        self.set_common_headers();

        let Response { origin, templates } = self;
        let origin = try!(origin.start());

        Ok(Response { origin: origin, templates: templates })
    }
}

impl<'a, 'b> Writer for Response<'a, Streaming> {
    #[inline(always)]
    fn write_all(&mut self, msg: &[u8]) -> IoResult<()> {
        self.origin.write_all(msg)
    }
    #[inline(always)]
    fn flush(&mut self) -> IoResult<()> {
        self.origin.flush()
    }
}

impl<'a, 'b> Response<'a, Streaming> {
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
