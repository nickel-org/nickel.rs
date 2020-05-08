use std::mem;
use std::borrow::Cow;
use std::path::Path;
use serde::Serialize;
use hyper::StatusCode;
use hyper::Response as HyperResponse;
use hyper::header::{self, HeaderMap, HeaderName, HeaderValue};
use time;
use crate::mimes::MediaType;
use std::io::{self, Write, copy};
use std::fs::File;
use std::any::Any;
use crate::{NickelError, Halt, MiddlewareResult, Responder, Action};
use crate::template_cache::TemplateCache;
use modifier::Modifier;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;

///A container for the response
pub struct Response<'a, B, D: 'a = ()> {
    ///the original `hyper::server::Response`
    origin: HyperResponse<B>,
    templates: &'a TemplateCache,
    data: &'a D,
    map: TypeMap,
    // This should be FnBox, but that's currently unstable
    on_send: Vec<Box<dyn FnMut(&mut Response<'a, B, D>)>>
}

impl<'a, B, D> Response<'a, B, D> {
    pub fn from_internal<'c, 'd>(response: HyperResponse<B>,
                                 templates: &'c TemplateCache,
                                 data: &'c D)
                                -> Response<'c, B, D> {
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

    /// Get a mutable reference to the HeaderMap.
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
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
    // TODO: replace with set_status, set_content_type, and set_header methods
    pub fn set<T: Modifier<Response<'a, B, D>>>(&mut self, attribute: T) -> &mut Response<'a, B, D> {
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
    pub fn send<T: Responder<B, D>>(self, data: T) -> MiddlewareResult<'a, B, D> {
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
    pub fn send_file<P:AsRef<Path>>(mut self, path: P) -> MiddlewareResult<'a, B, D> {
        let path = path.as_ref();
        // Chunk the response
        self.origin.headers_mut().remove(header::CONTENT_LENGTH);
        // Determine content type by file extension or default to binary
        let mime = mime_from_filename(path).unwrap_or(MediaType::Bin);
        self.set_header_fallback(header::CONTENT_TYPE, &mime.into());

        let mut file = try_with!(self, {
            File::open(path).map_err(|e| format!("Failed to send file '{:?}': {}",
                                                 path, e))
        });

        let mut stream = self.start()?;
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
        let now = HeaderValue::from_str(&time::now_utc().rfc822().to_string()).unwrap(); // rfc822 should always be valid
        self.set_header_fallback(&header::DATE, &now);
        self.set_header_fallback(&header::SERVER, &HeaderValue::from_static("Nickel"));
        self.set_header_fallback(&header::CONTENT_TYPE, &MediaType::Html.into());
    }

    /// Return an error with the appropriate status code for error handlers to
    /// provide output for.
    pub fn error<T>(self, status: StatusCode, message: T) -> MiddlewareResult<'a, B, D>
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
    pub fn set_header_fallback(&mut self, name: &HeaderName, value: &HeaderValue) {
        self.origin.headers_mut().entry(name).or_insert(*value);
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
    pub fn render<T, P>(self, path: P, data: &T) -> MiddlewareResult<'a, B, D>
        where T: Serialize, P: AsRef<Path> + Into<String> {

        let mut self_started = self.start()?;
        match self_started.templates.render(path, &mut self_started, data) {
            Ok(()) => Ok(Halt(self_started)),
            Err(e) => self_started.bail(format!("Problem rendering template: {:?}", e))
        }
    }

    pub fn start(mut self) -> Result<Response<'a, B, D>, NickelError<'a, B, D>> {
        let on_send = mem::replace(&mut self.on_send, vec![]);
        for mut f in on_send.into_iter().rev() {
            // TODO: Ensure `f` doesn't call on_send again
            f(&mut self)
        }

        // Set fallback headers last after everything runs, if we did this before as an
        // on_send then it would possibly set redundant things.
        self.set_fallback_headers();

        // Todo: migration cleanup
        //
        // Should be easy, may not even be needed
        // let Response { origin, templates, data, map, on_send } = self;
        // match origin.start() {
        //     Ok(origin) => {
        //         Ok(Response {
        //             origin: origin,
        //             templates: templates,
        //             data: data,
        //             map: map,
        //             on_send: on_send
        //         })
        //     },
        //     Err(e) =>
        //         unsafe {
        //             Err(NickelError::without_response(format!("Failed to start response: {}", e)))
        //         }
        // }

        Ok(self)
    }

    pub fn server_data(&self) -> &'a D {
        &self.data
    }

    pub fn on_send<F>(&mut self, f: F)
            where F: FnMut(&mut Response<'a, B, D>) + 'static {
        self.on_send.push(Box::new(f))
    }

    /// Pass execution off to another Middleware
    ///
    /// When returned from a Middleware, it allows computation to continue
    /// in any Middleware queued after the active one.
    pub fn next_middleware(self) -> MiddlewareResult<'a, B, D> {
        Ok(Action::Continue(self))
    }
}

impl<'a, B, D> Write for Response<'a, B, D> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Todo: migration cleanup
        // self.origin.write(buf)
        unimplemented!();
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        // Todo: migration cleanup
        // self.origin.flush()
        unimplemented!();
    }
}

impl<'a, B, D> Response<'a, B, D> {
    /// In the case of an unrecoverable error while a stream is already in
    /// progress, there is no standard way to signal to the client that an
    /// error has occurred. `bail` will drop the connection and log an error
    /// message.
    pub fn bail<T>(self, message: T) -> MiddlewareResult<'a, B, D>
            where T: Into<Cow<'static, str>> {
        let _ = self.end();
        unsafe { Err(NickelError::without_response(message)) }
    }

    /// Flushes all writing of a response to the client.
    // Todo: migration cleanup
    //
    // Should be easy, may not even be needed
    pub fn end(self) -> io::Result<()> {
        // self.origin.end()
        Ok(())
    }
}

impl <'a, B, D> Response<'a, B, D> {
    /// The status of this response.
    pub fn status(&self) -> StatusCode {
        self.origin.status()
    }

    /// The headers of this response.
    pub fn headers(&self) -> &HeaderMap {
        self.origin.headers()
    }

    pub fn data(&self) -> &'a D {
        &self.data
    }
}

impl<'a, B, D> Extensible for Response<'a, B, D> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'a, B, D> Pluggable for Response<'a, B, D> {}

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
    use hyper::StatusCode;
    use modifier::Modifier;
    use crate::{Response, MediaType};

    impl<'a, B, D> Modifier<Response<'a, B, D>> for StatusCode {
        fn modify(self, res: &mut Response<'a, B, D>) {
            *res.status_mut() = self
        }
    }

    // impl<'a, D> Modifier<Response<'a, D>> for MediaType {
    //     fn modify(self, res: &mut Response<'a, D>) {
    //         ContentType(self.into()).modify(res)
    //     }
    // }

//     macro_rules! header_modifiers {
//         ($($t:ty),+) => (
//             $(
//                 impl<'a, D> Modifier<Response<'a, D>> for $t {
//                     fn modify(self, res: &mut Response<'a, D>) {
//                         res.headers_mut().set(self)
//                     }
//                 }
//             )+
//         )
//     }

//     header_modifiers! {
//         Accept,
//         AccessControlAllowHeaders,
//         AccessControlAllowMethods,
//         AccessControlAllowOrigin,
//         AccessControlMaxAge,
//         AccessControlRequestHeaders,
//         AccessControlRequestMethod,
//         AcceptCharset,
//         AcceptEncoding,
//         AcceptLanguage,
//         AcceptRanges,
//         Allow,
//         Authorization<Basic>,
//         Authorization<Bearer>,
//         Authorization<String>,
//         CacheControl,
//         Cookie,
//         Connection,
//         ContentEncoding,
//         ContentLanguage,
//         ContentLength,
//         ContentType,
//         Date,
//         ETag,
//         Expect,
//         Expires,
//         From,
//         Host,
//         IfMatch,
//         IfModifiedSince,
//         IfNoneMatch,
//         IfRange,
//         IfUnmodifiedSince,
//         LastModified,
//         Location,
//         Pragma,
//         Referer,
//         Server,
//         SetCookie,
//         TransferEncoding,
//         Upgrade,
//         UserAgent,
//         Vary
//     }
}
