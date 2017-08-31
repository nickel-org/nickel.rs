use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std;
use std::thread;

use hyper::server::{Http, Service, NewService, Request, Response};

use middleware::MiddlewareStack;
use request;
use response;

use futures;
use futures::Future;

use std::time::Duration;

use hyper;
use futures::sync::oneshot;

pub struct Server<D> {
    middleware_stack: MiddlewareStack<D>,
    templates: response::TemplateCache,
    shared_data: D
}


// FIXME: Any better coherence solutions?
pub struct ArcServer<D>(Arc<Server<D>>);

impl<D: Sync + Send + 'static> Service for ArcServer<D> {


    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=hyper::Response, Error=hyper::Error>>;

    fn call<'a, 'k>(&'a self, req: Request) -> Self::Future {
        
        let i = self.0.clone();

        Box::new(request::RequestOrigin::from_internal(req).and_then(move |req_origin| {

            let mut res = hyper::Response::new();
            
            {
                let nickel_req = request::Request::from_internal(&req_origin,
                                                                &i.shared_data);
            

                let nickel_res = response::Response::from_internal(&mut res,
                                                                &i.templates,
                                                                &i.shared_data);

                i.middleware_stack.invoke(nickel_req, nickel_res);
            }

            futures::future::ok(res)

        }))
    }
}

impl<D: Sync + Send + 'static> NewService for ArcServer<D> {

    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = ArcServer<D>;
    
    fn new_service(&self) -> Result<Self::Instance, std::io::Error> {
        
        let cl: ArcServer<D> = ArcServer(self.0.clone());
        Ok(cl)
    }
}

impl<D: Sync + Send + 'static> Server<D> {
    pub fn new(middleware_stack: MiddlewareStack<D>, data: D) -> Server<D> {
        Server {
            middleware_stack: middleware_stack,
            templates: RwLock::new(HashMap::new()),
            shared_data: data
        }
    }

    pub fn serve<A: ToSocketAddrs>(self,
                                   addr: A,
                                   keep_alive: bool,
                                   shutdown_timeout: Option<Duration>)
                                    -> Result<ListeningServer, hyper::Error> {

        let arc = ArcServer(Arc::new(self));


            
        let addr2 = addr.to_socket_addrs().unwrap().next().unwrap();


        let (tx, rx): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
        let (socketaddr_tx, socketaddr_rx): (oneshot::Sender<SocketAddr>, oneshot::Receiver<SocketAddr>) = oneshot::channel();

        let child_handle = thread::spawn(move || {
            
            let mut http = Http::new();

            http.keep_alive(keep_alive);

            let mut server = http.bind(&addr2, arc)?;

            match server.local_addr() {
                Ok(local_addr) => {
                    socketaddr_tx.send(local_addr).unwrap();
                    if shutdown_timeout.is_some() {
                        server.shutdown_timeout(shutdown_timeout.unwrap());
                    }
                    server.run_until(rx.map_err(|err| { () }))
                }
                Err(err) => panic!("cannot get address {:?}", err)
            }

        });

        let local_addr = socketaddr_rx.wait().unwrap();

        let listening_server = ListeningServer {
            local_addr: local_addr,
            handle: child_handle,
            stopper: tx
        };
        
        Ok(listening_server)
    }

    //TODO: SSL
    // pub fn serve_https<A,S>(self,
    //                         addr: A,
    //                         keep_alive_timeout: Option<Duration>,
    //                         thread_count: Option<usize>,
    //                         ssl: S)
    //                         -> HttpResult<ListeningServer>
    //     where A: ToSocketAddrs,
    //           S: SslServer + Clone + Send + 'static {
    //     let arc = ArcServer(Arc::new(self));
    //     let mut server = try!(HyperServer::https(addr, ssl));

    //     server.keep_alive(keep_alive_timeout);

    //     let listening = match thread_count {
    //         Some(threads) => server.handle_threads(arc, threads),
    //         None => server.handle(arc),
    //     };

    //     listening.map(ListeningServer)
    // }
}

/// A server listening on a socket
pub struct ListeningServer {
    pub local_addr: SocketAddr,
    pub handle: thread::JoinHandle<Result<(), hyper::Error>>,
    pub stopper: oneshot::Sender<()>
}

impl ListeningServer {
    pub fn socket(&self) -> SocketAddr {
        self.local_addr.clone()
    }
    pub fn detach(self) {
        self.stopper.send(()).unwrap();
        self.handle.join().unwrap().unwrap();
    }
    pub fn wait(self) {
        self.handle.join().unwrap().unwrap();
    }
}