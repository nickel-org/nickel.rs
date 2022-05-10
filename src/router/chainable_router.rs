use hyper::method::Method;
use middleware::Middleware;
use nickel::Nickel;
use router::http_router::{HttpRouter};

pub struct ChainableRoute<'a, D: Sync + Send + 'static = ()> {
    path: String,
    nickel: &'a mut Nickel<D>
}

pub trait ChainablePath<'a, D> {
    fn route(&'a mut self, path: &str) -> ChainableRoute<'a, D>;
}

impl<'a, D: Sync + Send + 'static> ChainablePath<'a, D> for Nickel<D> {
    fn route(&'a mut self, path: &str) -> ChainableRoute<'a, D> {
        ChainableRoute {
            path: path.to_string(),
            nickel: self
        }
    }
}

pub trait ChainableHandler<'a, D> {
    fn route(&'a mut self, path: &str) -> ChainableRoute<'a, D>;
    fn get<H: Middleware<D>>(&mut self, handler: H) -> &mut Self;
    fn post<H: Middleware<D>>(&mut self, handler: H) -> &mut Self;
    fn delete<H: Middleware<D>>(&mut self, handler: H) -> &mut Self;
    fn options<H: Middleware<D>>(&mut self, handler: H) -> &mut Self;
    fn patch<H: Middleware<D>>(&mut self, handler: H) -> &mut Self;
    fn put<H: Middleware<D>>(&mut self, handler: H) -> &mut Self;
}

impl<'a, D: Sync + Send + 'static> ChainableHandler<'a, D> for ChainableRoute<'a, D> {
    fn route(&'a mut self, path: &str) -> ChainableRoute<'a, D> {
        ChainableRoute {
            path: path.to_string(),
            nickel: self.nickel
        }
    }
    fn get<H: Middleware<D>>(&mut self, handler: H) -> &mut Self {
        let path = self.path.clone();
        self.nickel.add_route(Method::Get, &path[..], handler);
        self
    }
    fn post<H: Middleware<D>>(&mut self, handler: H) -> &mut Self {
        let path = self.path.clone();
        self.nickel.add_route(Method::Post, &path[..], handler);
        self
    }
    fn delete<H: Middleware<D>>(&mut self, handler: H) -> &mut Self {
        let path = self.path.clone();
        self.nickel.add_route(Method::Delete, &path[..], handler);
        self
    }
    fn options<H: Middleware<D>>(&mut self, handler: H) -> &mut Self {
        let path = self.path.clone();
        self.nickel.add_route(Method::Options, &path[..], handler);
        self
    }
    fn patch<H: Middleware<D>>(&mut self, handler: H) -> &mut Self {
        let path = self.path.clone();
        self.nickel.add_route(Method::Patch, &path[..], handler);
        self
    }
    fn put<H: Middleware<D>>(&mut self, handler: H) -> &mut Self {
        let path = self.path.clone();
        self.nickel.add_route(Method::Put, &path[..], handler);
        self
    }
}
