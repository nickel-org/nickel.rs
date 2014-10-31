extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{Nickel, Request, Response, HttpRouter};
use std::io::net::ip::Ipv4Addr;
use std::sync::{Mutex, Arc};

//a single instance of this struct will be shares
//among requests.
struct Logger
{
    visits: u64
}

struct RouteData
{
    logger: Arc<Mutex<Logger>>
}

fn main() {
    let mut server = Nickel::new();

    fn root_handler (_request: &Request, response: &mut Response, route_data: &RouteData) {
	let mut logger = route_data.logger.lock();
	logger.visits += 1;
        response.send(format!("{}", logger.visits));
    }

    server.get_with_data("/", root_handler, RouteData{logger: Arc::new(Mutex::new(Logger{visits: 0}))});

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
