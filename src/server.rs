use std::clone::Clone;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::Duration;
use futures;
use futures::future::Future;
use futures_fs::FsPool;
use hyper::Result as HttpResult;
use hyper::{Error, Request, Response};
use hyper::server::{Http, Service};
// use hyper::net::SslServer; not supported in hyper 0.11

use middleware::MiddlewareStack;
use request;
use response;

pub struct Server<D> {
    middleware_stack: MiddlewareStack<D>,
    fspool: FsPool,
    templates: response::TemplateCache,
    shared_data: D,
}

// FIXME: Any better coherence solutions?
struct ArcServer<D>(Arc<Server<D>>);

impl<D> Clone for ArcServer<D> {
    fn clone(&self) -> ArcServer<D> {
        ArcServer(self.0.clone())
    }
}

impl<D: Sync + Send + 'static> Service for ArcServer<D> {
    type Request = Request;
    type Response = Response<response::ResponseStream>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let nickel_req = request::Request::from_internal(req, &self.0.shared_data);
        let nickel_res = response::Response::new(self.0.fspool.clone(), &self.0.templates, &self.0.shared_data);

        let final_res = self.0.middleware_stack.invoke(nickel_req, nickel_res);
        Box::new(futures::future::ok(final_res.origin))
    }
}

impl<D: Sync + Send + 'static> Server<D> {
    pub fn new(middleware_stack: MiddlewareStack<D>, data: D) -> Server<D> {
        Server {
            middleware_stack: middleware_stack,
            fspool: FsPool::new(40),
            templates: RwLock::new(HashMap::new()),
            shared_data: data
        }
    }

    pub fn serve(mut self,
                 addr: &SocketAddr,
                 keep_alive_timeout: Option<Duration>,
                 thread_count: Option<usize>,
                 verbose: bool)
                 -> HttpResult<()> {
        let arc = ArcServer(Arc::new(self));
        let mut http = Http::new();

        if let Some(threads) = thread_count {
            // override the default set in Server::new
            self.fspool = FsPool::new(threads);
        }
        http.keep_alive(keep_alive_timeout.is_some());
        let server = http.bind(addr, move || Ok(arc.clone()))?;

        if verbose {
            match server.local_addr() {
                Ok(a) => { println!("Listening on http://{}", server.local_addr().unwrap()); },
                Err(e) => { println!("Error getting socket: {:?}", e); }
            };
            println!("Ctrl-C to shutdown server");
        }

        server.run()
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
