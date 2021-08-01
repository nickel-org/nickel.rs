use std::borrow::Cow;
use chrono::prelude::Utc;
use std::path::Path;
use serde::Serialize;
use hyper::{Body, Response as HyperResponse, StatusCode};
use hyper::header::{self, HeaderMap, HeaderName, HeaderValue};
use crate::mimes::MediaType;
use std::io;
use crate::{NickelError, Halt, MiddlewareResult, Responder, Action};
use crate::template_cache::TemplateCache;
use modifier::Modifier;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use typemap::{ShareMap, TypeMap};

///A container for the response
pub struct Response<D: Send + 'static + Sync = ()> {
    ///the original `hyper::server::Response`
    pub origin: HyperResponse<Body>,
    templates: Arc<TemplateCache>,
    data: Arc<D>,
    map: ShareMap,
    // This should be FnBox, but that's currently unstable
    //on_send: Vec<Box<dyn FnMut(&mut Response<'a, D>)>>
}

impl<D: Send + 'static + Sync> Response<D> {
    pub fn from_internal(response: HyperResponse<Body>,
                         templates: Arc<TemplateCache>,
                         data: Arc<D>)
                         -> Response<D> {
        Response {
            origin: response,
            templates: templates,
            data: data,
            map: TypeMap::custom(),
            //on_send: vec![]
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
    /// use hyper::header::{self, HeaderValue};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", middleware! { |_, mut res|
    ///             // set the Status
    ///         res.set(StatusCode::PERMANENT_REDIRECT)
    ///             // update a Header value
    ///            .set_header(header::LOCATION,
    ///                        HeaderValue::from_static("http://nickel.rs".into()));
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
    pub fn set<T: Modifier<Response<D>>>(&mut self, attribute: T) -> &mut Response<D> {
        attribute.modify(self);
        self
    }

    /// Set a header value, return the od value if present.
    pub fn set_header<N: Into<HeaderName>, V: Into<HeaderValue>>(&mut self, name: N, value: V) -> Option<HeaderValue> {
        self.origin.headers_mut().insert(name.into(), value.into())
    }

    /// Set the body of the hyper response, discarding any already set
    pub fn set_body<T: Into<Body>>(&mut self, body: T) {
        *self.origin.body_mut() = body.into();
    }

    /// Writes a response
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Request, Response, MiddlewareResult};
    ///
    /// # #[allow(dead_code)]
    /// fn handler<D: Send + 'static + Sync>(_: &mut Request<D>, res: Response<D>) -> MiddlewareResult<D> {
    ///     res.send("hello world")
    /// }
    /// ```
    #[inline]
    pub fn send<T: Responder<D>>(self, data: T) -> MiddlewareResult<D> {
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
    /// async fn handler<D: Send + 'static + Sync>(_: &mut Request<D>, res: Response<D>) -> MiddlewareResult<D> {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     res.send_file(favicon).await
    /// }
    /// ```
    pub async fn send_file<P:AsRef<Path>>(mut self, path: P) -> MiddlewareResult<D> {
        let path = path.as_ref();
        // Chunk the response
        self.origin.headers_mut().remove(header::CONTENT_LENGTH);
        // Determine content type by file extension or default to binary
        let mime = mime_from_filename(path).unwrap_or(MediaType::Bin);
        self.set_header_fallback(&header::CONTENT_TYPE, &mime.into());

        self.start();
        match File::open(path).await {
            Ok(file) => {
                let stream = FramedRead::new(file, BytesCodec::new());
                let body = Body::wrap_stream(stream);
                self.set(StatusCode::OK).set_body(body);
                Ok(Halt(self))
            },
            Err(e) => {
                self.error(StatusCode::NOT_FOUND,
                           format!("Failed to send file '{:?}': {}", path, e))
            }
        }
    }

    // TODO: This needs to be more sophisticated to return the correct headers
    // not just "some headers" :)
    //
    // Also, it should only set them if not already set.
    fn set_fallback_headers(&mut self) {
        let now = HeaderValue::from_str(&Utc::now().to_rfc2822()).unwrap(); // rfc2822 should always be valid
        self.set_header_fallback(&header::DATE, &now);
        self.set_header_fallback(&header::SERVER, &HeaderValue::from_static("Nickel"));
        self.set_header_fallback(&header::CONTENT_TYPE, &MediaType::Html.into());
    }

    /// Return an error with the appropriate status code for error handlers to
    /// provide output for.
    pub fn error<T>(self, status: StatusCode, message: T) -> MiddlewareResult<D>
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
    /// use hyper::header;
    ///
    /// # #[allow(unreachable_code)]
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/", middleware! { |_, mut res|
    ///         res.set(MediaType::Html);
    ///         res.set_header_fallback(
    ///             &header::CONTENT_TYPE,
    ///             &MediaType::Txt.into());
    ///
    ///         "<h1>Hello World</h1>"
    ///     });
    ///
    ///     // ...
    /// }
    /// ```
    pub fn set_header_fallback(&mut self, name: &HeaderName, value: &HeaderValue) {
        self.origin.headers_mut().entry(name).or_insert(value.clone());
    }

    /// Renders the given template bound with the given data.
    ///
    /// # Examples
    /// ```{rust}
    /// use std::collections::HashMap;
    /// use nickel::{Request, Response, MiddlewareResult};
    ///
    /// # #[allow(dead_code)]
    /// async fn handler<D: Send + 'static + Sync>(_: &mut Request<D>, res: Response<D>) -> MiddlewareResult<D> {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     res.render("examples/assets/template.tpl", &data).await
    /// }
    /// ```
    pub async fn render<T, P>(mut self, path: P, data: &T) -> MiddlewareResult<D>
        where T: Serialize, P: AsRef<Path> + Into<String> {

        self.start();
        match self.templates.render(path, data).await {
            Ok(r) => self.send(r),
            Err(e) => {
                let msg = format!("Problem rendering template: {:?}", e);
                println!("{}", msg);
                self.error(StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        }
    }

    // Todo: migration cleanup
    //
    // hyper::Response no longer has a start() method. The api has
    // changed a lot, so this may not longer be necessary.
    //
    // What we are still doing is running the on_send mthods, and
    // setting fallback headers. Do we need this dedicated method in
    // the workflow to make sure that happens?
    pub fn start(&mut self) {
        // let on_send = mem::replace(&mut self.on_send, vec![]);
        // for mut f in on_send.into_iter().rev() {
        //     // TODO: Ensure `f` doesn't call on_send again
        //     f(self)
        // }

        // Set fallback headers last after everything runs, if we did this before as an
        // on_send then it would possibly set redundant things.
        self.set_fallback_headers();
    }

    pub fn server_data(&self) -> Arc<D> {
        self.data.clone()
    }

    // pub fn on_send<F>(&mut self, f: F)
    //         where F: FnMut(&mut Response<D>) + 'static {
    //     self.on_send.push(Box::new(f))
    // }

    /// Pass execution off to another Middleware
    ///
    /// When returned from a Middleware, it allows computation to continue
    /// in any Middleware queued after the active one.
    pub fn next_middleware(self) -> MiddlewareResult<D> {
        Ok(Action::Continue(self))
    }
}

// TODO: migration cleanup - delete this
// impl<D> Write for Response<D> {
//     #[inline(always)]
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//         // TODO: migration cleanup
//         // self.origin.write(buf)
//         unimplemented!();
//     }
//
//     #[inline(always)]
//     fn flush(&mut self) -> io::Result<()> {
//         // TODO: migration cleanup
//         // self.origin.flush()
//         unimplemented!();
//     }
// }

impl<D: Send + 'static + Sync> Response<D> {
    /// In the case of an unrecoverable error while a stream is already in
    /// progress, there is no standard way to signal to the client that an
    /// error has occurred. `bail` will drop the connection and log an error
    /// message.
    pub fn bail<T>(self, message: T) -> MiddlewareResult<D>
            where T: Into<Cow<'static, str>> {
        let _ = self.end();
        unsafe { Err(NickelError::without_response(message)) }
    }

    /// Flushes all writing of a response to the client.
    // TODO: migration cleanup
    //
    // Should be easy, may not even be needed
    pub fn end(self) -> io::Result<()> {
        // self.origin.end()
        Ok(())
    }
}

impl <D: Send + 'static + Sync> Response<D> {
    /// The status of this response.
    pub fn status(&self) -> StatusCode {
        self.origin.status()
    }

    /// The headers of this response.
    pub fn headers(&self) -> &HeaderMap {
        self.origin.headers()
    }

    pub fn data(&self) -> Arc<D> {
        self.data.clone()
    }

    // (Hopefully) temporary replacements for the Extensible trait. We can't
    // support plugins without Extensible, but access to the ShareMap is used by
    // itself.
    pub fn extensions(&self) -> &ShareMap {
        &self.map
    }

    pub fn extensions_mut(&mut self) -> &mut ShareMap {
        &mut self.map
    }
}

// TODO: migration cleanup - Extensible does not support ShareMap, but TypeMap is not Sync+Send
// impl<D: Send + 'static + Sync> Extensible for Response<D> {
//     fn extensions(&self) -> &ShareMap {
//         &self.map
//     }

//     fn extensions_mut(&mut self) -> &mut ShareMap {
//         &mut self.map
//     }
// }

// impl<D: Send + 'static + Sync> Pluggable for Response<D> {}

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
    use hyper::header;
    use modifier::Modifier;
    use crate::{Response, MediaType};

    impl<D: Send + 'static + Sync> Modifier<Response<D>> for StatusCode {
        fn modify(self, res: &mut Response<D>) {
            *res.status_mut() = self
        }
    }

    impl<D: Send + 'static + Sync> Modifier<Response<D>> for MediaType {
        fn modify(self, res: &mut Response<D>) {
            res.set_header(header::CONTENT_TYPE, self);
        }
    }
}
