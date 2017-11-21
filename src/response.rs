use std::mem;
use std::borrow::Cow;
use std::path::Path;
use serialize::Encodable;
use hyper::status::StatusCode;
use hyper::server::Response as HyperResponse;
use hyper::header::{
    Headers, Date, HttpDate, Server, ContentType, ContentLength, Header, HeaderFormat
};
use hyper::net::{Fresh, Streaming};
use time;
use mimes::MediaType;
use std::io::{self, Write, copy};
use std::fs::File;
use std::any::Any;
use {NickelError, Halt, MiddlewareResult, Responder, Action};
use template_cache::TemplateCache;
use modifier::Modifier;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;

///A container for the response
pub struct Response<'a, D: 'a = (), T: 'static + Any = Fresh> {
    ///the original `hyper::server::Response`
    origin: HyperResponse<'a, T>,
    templates: &'a TemplateCache,
    data: &'a D,
    map: TypeMap,
    // This should be FnBox, but that's currently unstable
    on_send: Vec<Box<FnMut(&mut Response<'a, D, Fresh>)>>
}

impl<'a, D> Response<'a, D, Fresh> {
    pub fn from_internal<'c, 'd>(response: HyperResponse<'c, Fresh>,
                                 templates: &'c TemplateCache,
                                 data: &'c D)
                                -> Response<'c, D, Fresh> {
        Response {
            origin: response,
            templates: templates,
            data: data,
            map: TypeMap::new(),
            on_send: vec![]
        }
    }

    /// Get a mutable reference to the status.
    pub fn status_mut(&mut self) -> &mut StatusCode {
        self.origin.status_mut()
    }

    /// Get a mutable reference to the Headers.
    pub fn headers_mut(&mut self) -> &mut Headers {
        self.origin.headers_mut()
    }

    /// Modify the response with the provided data.
    ///
    /// # Examples
    /// ```{rust}
    /// extern crate hyper;
    /// #[macro_use] extern crate nickel;
    ///
    /// use nickel::{Nickel, HttpRouter, MediaType};
    /// use nickel::status::StatusCode;
    /// use hyper::header::Location;
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", middleware! { |_, mut res|
    ///             // set the Status
    ///         res.set(StatusCode::PermanentRedirect)
    ///             // update a Header value
    ///            .set(Location("http://nickel.rs".into()));
    ///
    ///         ""
    ///     });
    ///
    ///     server.get("/b", middleware! { |_, mut res|
    ///             // setting the content type
    ///         res.set(MediaType::Json);
    ///
    ///         "{'foo': 'bar'}"
    ///     });
    ///
    ///     // ...
    /// }
    /// ```
    pub fn set<T: Modifier<Response<'a, D>>>(&mut self, attribute: T) -> &mut Response<'a, D> {
        attribute.modify(self);
        self
    }

    /// Writes a response
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult};
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    ///     res.send("hello world")
    /// }
    /// ```
    #[inline]
    pub fn send<T: Responder<D>>(self, data: T) -> MiddlewareResult<'a, D> {
        data.respond(self)
    }

    /// Writes a file to the output.
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult};
    /// use std::path::Path;
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     res.send_file(favicon)
    /// }
    /// ```
    pub fn send_file<P:AsRef<Path>>(mut self, path: P) -> MiddlewareResult<'a, D> {
        let path = path.as_ref();
        // Chunk the response
        self.origin.headers_mut().remove::<ContentLength>();
        // Determine content type by file extension or default to binary
        let mime = mime_from_filename(path).unwrap_or(MediaType::Bin);
        self.set_header_fallback(|| ContentType(mime.into()));

        let mut file = try_with!(self, {
            File::open(path).map_err(|e| format!("Failed to send file '{:?}': {}",
                                                 path, e))
        });

        let mut stream = try!(self.start());
        match copy(&mut file, &mut stream) {
            Ok(_) => Ok(Halt(stream)),
            Err(e) => stream.bail(format!("Failed to send file: {}", e))
        }
    }

    // TODO: This needs to be more sophisticated to return the correct headers
    // not just "some headers" :)
    //
    // Also, it should only set them if not already set.
    fn set_fallback_headers(&mut self) {
        self.set_header_fallback(|| Date(HttpDate(time::now_utc())));
        self.set_header_fallback(|| Server("Nickel".to_string()));
        self.set_header_fallback(|| ContentType(MediaType::Html.into()));
    }

    /// Return an error with the appropriate status code for error handlers to
    /// provide output for.
    pub fn error<T>(self, status: StatusCode, message: T) -> MiddlewareResult<'a, D>
            where T: Into<Cow<'static, str>> {
        Err(NickelError::new(self, message, status))
    }

    /// Sets the header if not already set.
    ///
    /// If the header is not set then `f` will be called.
    ///
    /// # Examples
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// extern crate hyper;
    ///
    /// use nickel::{Nickel, HttpRouter, MediaType};
    /// use hyper::header::ContentType;
    ///
    /// # #[allow(unreachable_code)]
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/", middleware! { |_, mut res|
    ///         res.set(MediaType::Html);
    ///         res.set_header_fallback(|| {
    ///             panic!("Should not get called");
    ///             ContentType(MediaType::Txt.into())
    ///         });
    ///
    ///         "<h1>Hello World</h1>"
    ///     });
    ///
    ///     // ...
    /// }
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
    /// use nickel::{Request, Response, MiddlewareResult};
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     res.render("examples/assets/template.tpl", &data)
    /// }
    /// ```
    pub fn render<T, P>(self, path: P, data: &T) -> MiddlewareResult<'a, D>
        where T: Encodable, P: AsRef<Path> + Into<String> {

        let mut self_started = self.start()?;
        match self_started.templates.render(path, &mut self_started, data) {
            Ok(()) => Ok(Halt(self_started)),
            Err(e) => self_started.bail(format!("Problem rendering template: {:?}", e))
        }
    }

    pub fn start(mut self) -> Result<Response<'a, D, Streaming>, NickelError<'a, D>> {
        let on_send = mem::replace(&mut self.on_send, vec![]);
        for mut f in on_send.into_iter().rev() {
            // TODO: Ensure `f` doesn't call on_send again
            f(&mut self)
        }

        // Set fallback headers last after everything runs, if we did this before as an
        // on_send then it would possibly set redundant things.
        self.set_fallback_headers();

        let Response { origin, templates, data, map, on_send } = self;
        match origin.start() {
            Ok(origin) => {
                Ok(Response {
                    origin: origin,
                    templates: templates,
                    data: data,
                    map: map,
                    on_send: on_send
                })
            },
            Err(e) =>
                unsafe {
                    Err(NickelError::without_response(format!("Failed to start response: {}", e)))
                }
        }
    }

    pub fn server_data(&self) -> &'a D {
        &self.data
    }

    pub fn on_send<F>(&mut self, f: F)
            where F: FnMut(&mut Response<'a, D, Fresh>) + 'static {
        self.on_send.push(Box::new(f))
    }

    /// Pass execution off to another Middleware
    ///
    /// When returned from a Middleware, it allows computation to continue
    /// in any Middleware queued after the active one.
    pub fn next_middleware(self) -> MiddlewareResult<'a, D> {
        Ok(Action::Continue(self))
    }
}

impl<'a, 'b, D> Write for Response<'a, D, Streaming> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.origin.write(buf)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        self.origin.flush()
    }
}

impl<'a, 'b, D> Response<'a, D, Streaming> {
    /// In the case of an unrecoverable error while a stream is already in
    /// progress, there is no standard way to signal to the client that an
    /// error has occurred. `bail` will drop the connection and log an error
    /// message.
    pub fn bail<T>(self, message: T) -> MiddlewareResult<'a, D>
            where T: Into<Cow<'static, str>> {
        let _ = self.end();
        unsafe { Err(NickelError::without_response(message)) }
    }

    /// Flushes all writing of a response to the client.
    pub fn end(self) -> io::Result<()> {
        self.origin.end()
    }
}

impl <'a, D, T: 'static + Any> Response<'a, D, T> {
    /// The status of this response.
    pub fn status(&self) -> StatusCode {
        self.origin.status()
    }

    /// The headers of this response.
    pub fn headers(&self) -> &Headers {
        self.origin.headers()
    }

    pub fn data(&self) -> &'a D {
        &self.data
    }
}

impl<'a, D, T: 'static + Any> Extensible for Response<'a, D, T> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'a, D, T: 'static + Any> Pluggable for Response<'a, D, T> {}

fn mime_from_filename<P: AsRef<Path>>(path: P) -> Option<MediaType> {
    path.as_ref()
        .extension()
        .and_then(|os| os.to_str())
        // Lookup mime from file extension
        .and_then(|s| s.parse().ok())
}

#[test]
fn matches_content_type () {
    assert_eq!(Some(MediaType::Txt), mime_from_filename("test.txt"));
    assert_eq!(Some(MediaType::Json), mime_from_filename("test.json"));
    assert_eq!(Some(MediaType::Bin), mime_from_filename("test.bin"));
}

mod modifier_impls {
    use hyper::header::*;
    use hyper::status::StatusCode;
    use modifier::Modifier;
    use {Response, MediaType};

    impl<'a, D> Modifier<Response<'a, D>> for StatusCode {
        fn modify(self, res: &mut Response<'a, D>) {
            *res.status_mut() = self
        }
    }

    impl<'a, D> Modifier<Response<'a, D>> for MediaType {
        fn modify(self, res: &mut Response<'a, D>) {
            ContentType(self.into()).modify(res)
        }
    }

    macro_rules! header_modifiers {
        ($($t:ty),+) => (
            $(
                impl<'a, D> Modifier<Response<'a, D>> for $t {
                    fn modify(self, res: &mut Response<'a, D>) {
                        res.headers_mut().set(self)
                    }
                }
            )+
        )
    }

    header_modifiers! {
        Accept,
        AccessControlAllowHeaders,
        AccessControlAllowMethods,
        AccessControlAllowOrigin,
        AccessControlMaxAge,
        AccessControlRequestHeaders,
        AccessControlRequestMethod,
        AcceptCharset,
        AcceptEncoding,
        AcceptLanguage,
        AcceptRanges,
        Allow,
        Authorization<Basic>,
        Authorization<Bearer>,
        Authorization<String>,
        CacheControl,
        Cookie,
        Connection,
        ContentEncoding,
        ContentLanguage,
        ContentLength,
        ContentType,
        Date,
        ETag,
        Expect,
        Expires,
        From,
        Host,
        IfMatch,
        IfModifiedSince,
        IfNoneMatch,
        IfRange,
        IfUnmodifiedSince,
        LastModified,
        Location,
        Pragma,
        Referer,
        Server,
        SetCookie,
        TransferEncoding,
        Upgrade,
        UserAgent,
        Vary
    }
}
