use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::old_path::BytesContainer;
use std::path::Path;
use serialize::Encodable;
use hyper::status::StatusCode::{self, InternalServerError};
use hyper::server::Response as HyperResponse;
use hyper::header::{Date, Server, ContentType, ContentLength, Header, HeaderFormat};
use hyper::net::{Fresh, Streaming};
use time;
use mimes::{get_media_type, MediaType};
use mustache;
use mustache::Template;
use std::io;
use std::io::{Read, Write, copy};
use std::fs::File;
use {NickelError, Halt, MiddlewareResult};
use NickelErrorKind::ErrorWithStatusCode;

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
        self.origin.headers_mut().set(ContentType(get_media_type(mt)));
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
    ///     res.send("hello world")
    /// }
    /// ```
    pub fn send<T: BytesContainer> (self, text: T) -> MiddlewareResult<'a> {
        let mut stream = try!(self.start());
        match stream.write_all(text.container_as_bytes()) {
            Ok(()) => Ok(Halt(stream)),
            Err(e) => Err(NickelError::new(stream,
                                           format!("Failed to send: {}", e),
                                           ErrorWithStatusCode(InternalServerError)))
        }
    }

    /// Writes a file to the output.
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult, Halt};
    /// use nickel::status::StatusCode;
    /// use std::path::Path;
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     res.send_file(favicon)
    /// }
    /// ```
    pub fn send_file(mut self, path: &Path) -> MiddlewareResult<'a> {
        // Chunk the response
        self.origin.headers_mut().remove::<ContentLength>();
        // Determine content type by file extension or default to binary
        self.content_type(path.extension()
                              .and_then(|os| os.to_str())
                              .and_then(|s| s.parse().ok())
                              .unwrap_or(MediaType::Bin));

        match File::open(path) {
            Ok(mut file) => {
                let mut stream = try!(self.start());
                match copy(&mut file, &mut stream) {
                    Ok(_) => Ok(Halt(stream)),
                    Err(e) => Err(NickelError::new(stream,
                                                   format!("Failed to send file: {}", e),
                                                   ErrorWithStatusCode(InternalServerError)))
                }
            }
            Err(e) => {
                self.status_code(InternalServerError);
                let stream = try!(self.start());
                Err(NickelError::new(stream,
                                     format!("Failed to send file '{:?}': {}", path, e),
                                     ErrorWithStatusCode(InternalServerError)))
            }
        }
    }

    // TODO: This needs to be more sophisticated to return the correct headers
    // not just "some headers" :)
    //
    // Also, it should only set them if not already set.
    fn set_fallback_headers(&mut self) {
        self.set_header_fallback(|| Date(time::now_utc()));
        self.set_header_fallback(|| Server("Nickel".to_string()));
        self.set_header_fallback(|| ContentType(get_media_type(MediaType::Html)));
    }

    /// Sets the header if not already set.
    ///
    /// If the header is not set then `f` will be called.
    /// Renders the given template bound with the given data.
    ///
    /// # Examples
    /// ```{rust}
    /// # extern crate nickel;
    /// # extern crate hyper;
    ///
    /// # fn main() {
    /// use nickel::{Request, Response, MiddlewareResult, Halt, MediaType, get_media_type};
    /// use hyper::header::ContentType;
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     res.content_type(MediaType::Html);
    ///     res.set_header_fallback(|| {
    ///         panic!("Should not get called");
    ///         ContentType(get_media_type(MediaType::Txt))
    ///     });
    ///     res.send("<h1>Hello World</h1>")
    /// }
    /// # }
    /// ```
    pub fn set_header_fallback<F, H>(&mut self, f: F)
            where H: Header + HeaderFormat, F: FnOnce() -> H {
        let headers = self.origin.headers_mut();
        if !headers.has::<H>() { headers.set(f()) }
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
    ///     res.render("examples/assets/template.tpl", &data)
    /// }
    /// ```
    pub fn render<T>(self, path: &'static str, data: &T) -> MiddlewareResult<'a>
            where T: Encodable {
        fn render<'a, T>(res: Response<'a>, template: &Template, data: &T)
                -> MiddlewareResult<'a> where T: Encodable {
            let mut stream = try!(res.start());
            match template.render(&mut stream, data) {
                Ok(()) => Ok(Halt(stream)),
                Err(e) => {
                    Err(NickelError::new(stream,
                                         format!("Problem rendering template: {:?}", e),
                                         ErrorWithStatusCode(InternalServerError)))
                }
            }
        }

        // Fast path doesn't need writer lock
        if let Some(t) = self.templates.read().unwrap().get(&path) {
            return render(self, t, data);
        }

        // We didn't find the template, get writers lock
        let mut templates = self.templates.write().unwrap();
        // Search again incase there was a race to compile the template
        let template = match templates.entry(path) {
            Vacant(entry) => {
                let mut file = File::open(&Path::new(path))
                                     .ok().expect(&*format!("Couldn't open the template file: {}", path));
                let mut raw_template = String::new();

                file.read_to_string(&mut raw_template)
                    .ok().expect(&*format!("Couldn't open the template file: {}", path));

                entry.insert(mustache::compile_str(&*raw_template))
            },
            Occupied(entry) => entry.into_mut()
        };

        render(self, template, data)
    }

    pub fn start(mut self) -> Result<Response<'a, Streaming>, NickelError<'a>> {
        self.set_fallback_headers();

        let Response { origin, templates } = self;
        match origin.start() {
            Ok(origin) => Ok(Response { origin: origin, templates: templates }),
            Err(e) => {
                unsafe {
                    Err(NickelError::without_response(format!("Failed to start response: {}", e),
                                                      ErrorWithStatusCode(InternalServerError)))
                }
            }
        }
    }
}

impl<'a, 'b> Write for Response<'a, Streaming> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.origin.write(buf)
    }
    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        self.origin.flush()
    }
}

impl<'a, 'b> Response<'a, Streaming> {
    /// Flushes all writing of a response to the client.
    pub fn end(self) -> io::Result<()> {
        self.origin.end()
    }
}

#[test]
fn matches_content_type () {
    use hyper::mime::{Mime, TopLevel, SubLevel};
    let path = &Path::new("test.txt");
    let content_type = path.extension()
                           .and_then(|os| os.to_str())
                           .and_then(|s| s.parse().ok());

    assert_eq!(content_type, Some(MediaType::Txt));
    let content_type = content_type.map(get_media_type).unwrap();

    match content_type {
        Mime(TopLevel::Text, SubLevel::Plain, _) => {}, // OK
        wrong => panic!("Wrong mime: {}", wrong)
    }
}
