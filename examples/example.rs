extern crate http;
extern crate floor;

use floor::{ Floor, Request };
use http::server::{ ResponseWriter };

fn main() {

    let mut server = Floor::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    fn fooHandler (request: Request, response: &mut ResponseWriter) -> () {

        let text = String::new()
                    .append("This is user: ")
                    .append(request.params.get(&"userid".to_string()).as_slice());

        response.write(text.as_bytes()); 
    };

    fn barHandler (request: Request, response: &mut ResponseWriter) -> () { 
        response.write("This is the /bar handler".as_bytes()); 
    };

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", fooHandler);

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", barHandler);

    server.listen(6767);
}