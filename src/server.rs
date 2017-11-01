use std::clone::Clone;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::Duration;
use futures;
use futures::future::Future;
use hyper::Result as HttpResult;
use hyper::{Error, Request, Response};
use hyper::header::ContentLength;
use hyper::server::{Http, Service};
// use hyper::net::SslServer; not supported in hyper 0.11

use middleware::MiddlewareStack;
use request;
use response;

pub struct Server<B, D> {
    middleware_stack: MiddlewareStack<B, D>,
    templates: response::TemplateCache,
    shared_data: D,
}

// FIXME: Any better coherence solutions?
struct ArcServer<B, D>(Arc<Server<B, D>>);

// impl<D: Sync + Send + 'static> Handler for ArcServer<D> {
//     fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, res: Response<'a>) {
//         let nickel_req = request::Request::from_internal(req,
//                                                          &self.0.shared_data);

//         let nickel_res = response::Response::from_internal(res,
//                                                            &self.0.templates,
//                                                            &self.0.shared_data);

//         self.0.middleware_stack.invoke(nickel_req, nickel_res);
//     }
// }

impl<B, D> Clone for ArcServer<B, D> {
    fn clone(&self) -> ArcServer<B, D> {
        ArcServer(self.0.clone())
    }
}

const PHRASE: &'static str = "Hello, World!";

impl<B, D: Sync + Send + 'static> Service for ArcServer<B, D> {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Error>>;

    fn call(&self, req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}

impl<B: 'static, D: Sync + Send + 'static> Server<B, D> {
    pub fn new(middleware_stack: MiddlewareStack<B, D>, data: D) -> Server<B, D> {
        Server {
            middleware_stack: middleware_stack,
            templates: RwLock::new(HashMap::new()),
            shared_data: data
        }
    }

    pub fn serve(self,
                 addr: &SocketAddr,
                 keep_alive_timeout: Option<Duration>,
                 thread_count: Option<usize>)
                 -> HttpResult<()> {
        let arc = ArcServer(Arc::new(self));
        // let mut server = try!(HyperServer::http(addr));
        let mut http = Http::new();

        http.keep_alive(keep_alive_timeout.is_some());
        let server = http.bind(addr, || Ok(arc.clone()))?;
        server.run()
        // let listening = match thread_count {
        //     Some(threads) => server.handle_threads(arc, threads),
        //     None => server.handle(arc),
        // };

        // listening.map(ListeningServer)
        // Ok(())
    }

    /* Ssl support changed in hyper 0.11
    pub fn serve_https<A,S>(self,
                            addr: A,
                            keep_alive_timeout: Option<Duration>,
                            thread_count: Option<usize>,
                            ssl: S)
                            -> HttpResult<ListeningServer>
        where A: ToSocketAddrs,
              S: SslServer + Clone + Send + 'static {
        let arc = ArcServer(Arc::new(self));
        let mut server = try!(HyperServer::https(addr, ssl));

        server.keep_alive(keep_alive_timeout);

        let listening = match thread_count {
            Some(threads) => server.handle_threads(arc, threads),
            None => server.handle(arc),
        };

        listening.map(ListeningServer)
    }
    */
}

// /// A server listeing on a socket
// pub struct ListeningServer(Listening);

// impl ListeningServer {
//     /// Gets the `SocketAddr` which the server is currently listening on.
//     pub fn socket(&self) -> SocketAddr {
//         self.0.socket
//     }

//     /// Detaches the server thread.
//     ///
//     /// This doesn't actually kill the server, it just stops the current thread from
//     /// blocking due to the server running. In the case where `main` returns due to
//     /// this unblocking, then the server will be killed due to process death.
//     ///
//     /// The required use of this is when writing unit tests which spawn servers and do
//     /// not want to block the test-runner by waiting on the server to stop because
//     /// it probably never will.
//     ///
//     /// See [this hyper issue](https://github.com/hyperium/hyper/issues/338) for more
//     /// information.
//     pub fn detach(self) {
//         // We want this handle to be dropped without joining.
//         let _ = ::std::thread::spawn(move || {
//             // This will hang the spawned thread.
//             // See: https://github.com/hyperium/hyper/issues/338
//             let _ = self.0;
//         });
//     }
// }
