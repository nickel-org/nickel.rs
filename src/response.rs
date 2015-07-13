use std::mem;
use std::borrow::Cow;
use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
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
use mustache;
use mustache::Template;
use std::io::{self, Read, Write, copy};
use std::fs::File;
use std::any::Any;
use {NickelError, Halt, MiddlewareResult, Responder, Request};
use router::RouteResult;
use modifier::Modifier;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;

pub type TemplateCache = RwLock<HashMap<String, Template>>;

///A container for the response
pub struct Response<'a, 'k: 'a, D: 'a, T: 'static + Any = Fresh> {
    ///the original `hyper::server::Response`
    origin: HyperResponse<'a, T>,
    pub request: Request<'a, 'k>,
    pub route_result: Option<RouteResult<'a, D>>,
    templates: &'a TemplateCache,
    data: &'a D,
    map: TypeMap,
    // This should be FnBox, but that's currently unstable
    on_send: Vec<Box<FnMut(&mut Response<'a, 'k, D, Fresh>)>>
}

impl<'a, 'k, D> Response<'a, 'k, D, Fresh> {
    pub fn from_internal(response: HyperResponse<'a, Fresh>,
                         request: Request<'a, 'k>,
                         templates: &'a TemplateCache,
                         data: &'a D)
                                -> Response<'a, 'k, D, Fresh> {
        Response {
            origin: response,
            route_result: None,
            request: request,
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
    ///     server.get("/a", middleware! { |mut res|
    ///             // set the Status
    ///         res.set(StatusCode::PermanentRedirect)
    ///             // update a Header value
    ///            .set(Location("http://nickel.rs".into()));
    ///
    ///         ""
    ///     });
    ///
    ///     server.get("/b", middleware! { |mut res|
    ///             // setting the content type
    ///         res.set(MediaType::Json);
    ///
    ///         "{'foo': 'bar'}"
    ///     });
    ///
    ///     // ...
    /// }
    /// ```
    pub fn set<T: Modifier<Response<'a, 'k, D>>>(&mut self, attribute: T) -> &mut Response<'a, 'k, D> {
        attribute.modify(self);
        self
    }

    /// Writes a response
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Response, MiddlewareResult};
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, 'k, D>(res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
    ///     res.send("hello world")
    /// }
    /// ```
    #[inline]
    pub fn send<T: Responder<D>>(self, data: T) -> MiddlewareResult<'a, 'k, D> {
        data.respond(self)
    }

    /// Writes a file to the output.
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Response, MiddlewareResult};
    /// use std::path::Path;
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, 'k, D>(res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     res.send_file(favicon)
    /// }
    /// ```
    pub fn send_file(mut self, path: &Path) -> MiddlewareResult<'a, 'k, D> {
        // Chunk the response
        self.origin.headers_mut().remove::<ContentLength>();
        // Determine content type by file extension or default to binary
        let mime = mime_from_filename(path).unwrap_or(MediaType::Bin);
        self.set(mime);

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
    pub fn error<T>(self, status: StatusCode, message: T) -> MiddlewareResult<'a, 'k, D>
            where T: Into<Cow<'static, str>> {
        Err(NickelError::new(self, message, status))
    }

    /// Sets the header if not already set.
    ///
    /// If the header is not set then `f` will be called.
    /// Renders the given template bound with the given data.
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
    ///     server.get("/", middleware! { |mut res|
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
    /// use nickel::{Response, MiddlewareResult};
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, 'k, D>(res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     res.render("examples/assets/template.tpl", &data)
    /// }
    /// ```
    pub fn render<T, P>(self, path: P, data: &T) -> MiddlewareResult<'a, 'k, D>
            where T: Encodable, P: AsRef<str> + Into<String> {
        fn render<'a, 'k, D, T>(res: Response<'a, 'k, D>, template: &Template, data: &T)
                -> MiddlewareResult<'a, 'k, D> where T: Encodable {
            let mut stream = try!(res.start());
            match template.render(&mut stream, data) {
                Ok(()) => Ok(Halt(stream)),
                Err(e) => stream.bail(format!("Problem rendering template: {:?}", e))
            }
        }

        // Fast path doesn't need writer lock
        if let Some(t) = self.templates.read().unwrap().get(path.as_ref()) {
            return render(self, t, data);
        }

        // We didn't find the template, get writers lock
        let mut templates = self.templates.write().unwrap();

        // Additional clone required for now as the entry api doesn't give us a key ref
        let path = path.into();

        // Search again incase there was a race to compile the template
        let template = match templates.entry(path.clone()) {
            Vacant(entry) => {
                let template = try_with!(self, {
                    mustache::compile_path(&path)
                             .map_err(|e| format!("Failed to compile template '{}': {:?}",
                                            path, e))
                });
                entry.insert(template)
            },
            Occupied(entry) => entry.into_mut()
        };

        render(self, template, data)
    }

    pub fn start(mut self) -> Result<Response<'a, 'k, D, Streaming>, NickelError<'a, 'k, D>> {
        let on_send = mem::replace(&mut self.on_send, vec![]);
        for mut f in on_send.into_iter().rev() {
            // TODO: Ensure `f` doesn't call on_send again
            f(&mut self)
        }

        // Set fallback headers last after everything runs, if we did this before as an
        // on_send then it would possibly set redundant things.
        self.set_fallback_headers();

        let Response { origin, request, route_result, templates, data, map, on_send } = self;
        match origin.start() {
            Ok(origin) => {
                Ok(Response {
                    origin: origin,
                    route_result: route_result,
                    request: request,
                    templates: templates,
                    data: data,
                    map: map,
                    on_send: on_send
                })
            },
            Err(e) =>
                unsafe {
                    Err(NickelError::without_response(format!("Failed to start response: {}", e),
                                                      request))
                }
        }
    }

    pub fn on_send<F>(&mut self, f: F)
            where F: FnMut(&mut Response<'a, 'k, D, Fresh>) + 'static {
        self.on_send.push(Box::new(f))
    }
}

impl<'a, 'k, D> Write for Response<'a, 'k, D, Streaming> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.origin.write(buf)
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        self.origin.flush()
    }
}

impl<'a, 'k, D> Response<'a, 'k, D, Streaming> {
    /// In the case of an unrecoverable error while a stream is already in
    /// progress, there is no standard way to signal to the client that an
    /// error has occurred. `bail` will drop the connection and log an error
    /// message.
    pub fn bail<T>(self, message: T) -> MiddlewareResult<'a, 'k, D>
            where T: Into<Cow<'static, str>> {
        let Response { origin, request, .. } = self;
        let _ = origin.end();
        unsafe { Err(NickelError::without_response(message, request)) }
    }

    /// Flushes all writing of a response to the client.
    pub fn end(self) -> io::Result<()> {
        self.origin.end()
    }
}

impl<'a, 'k, D, T: 'static + Any> Response<'a, 'k, D, T> {
    /// The status of this response.
    pub fn status(&self) -> StatusCode {
        self.origin.status()
    }

    /// The headers of this response.
    pub fn headers(&self) -> &Headers {
        self.origin.headers()
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.route_result.as_ref().and_then(|r| r.param(key))
    }
}

impl<'a, 'k, D, T: 'static + Any> Extensible for Response<'a, 'k, D, T> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'a, 'k, D, T: 'static + Any> Pluggable for Response<'a, 'k, D, T> {}

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

    impl<'a, 'k, D> Modifier<Response<'a, 'k, D>> for StatusCode {
        fn modify(self, res: &mut Response<'a, 'k, D>) {
            *res.status_mut() = self
        }
    }

    impl<'a, 'k, D> Modifier<Response<'a, 'k, D>> for MediaType {
        fn modify(self, res: &mut Response<'a, 'k, D>) {
            ContentType(self.into()).modify(res)
        }
    }

    macro_rules! header_modifiers {
        ($($t:ty),+) => (
            $(
                impl<'a, 'k, D> Modifier<Response<'a, 'k, D>> for $t {
                    fn modify(self, res: &mut Response<'a, 'k, D>) {
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
