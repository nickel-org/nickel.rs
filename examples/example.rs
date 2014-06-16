extern crate http;
extern crate floor;

use floor::{ Floor, Request };
use http::server::{ ResponseWriter };

fn main() {

    let mut server = Floor::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    fn user_handler (request: Request, response: &mut ResponseWriter) {
        let _ = write!(response, "This is user: {}", request.params.get(&"userid".to_string()));
    };

    fn bar_handler (request: Request, response: &mut ResponseWriter) { 
        response.write("This is the /bar handler".as_bytes()); 
    };

    fn simple_wildcard (request: Request, response: &mut ResponseWriter) { 
        response.write("This matches /some/crazy/route but not /some/super/crazy/route".as_bytes()); 
    };

    fn double_wildcard (request: Request, response: &mut ResponseWriter) { 
        response.write("This matches /a/crazy/route and also /a/super/crazy/route".as_bytes()); 
    };

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", user_handler);

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    // go to http://localhost:6767/some/crazy/route to see this route in action
    server.get("/some/*/route", simple_wildcard);

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    server.get("/a/**/route", double_wildcard);

    server.listen(6767);
}
