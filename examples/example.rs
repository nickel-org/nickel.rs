extern crate http;
extern crate floor;

use floor::{ Floor };
use http::server::{ Request, ResponseWriter };

fn main() {

    let mut server = Floor::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    fn fooHandler (request: &Request, response: &mut ResponseWriter) -> () {
        response.write("hello from foo".as_bytes()); 
    };

    fn barHandler (request: &Request, response: &mut ResponseWriter) -> () { 
        response.write("hello from bar".as_bytes()); 
    };

    server.get("/foo", fooHandler);
    server.get("/bar", barHandler);

    server.listen(6767);
}