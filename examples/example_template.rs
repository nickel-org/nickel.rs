extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{ Nickel, Request, Response }; 
use std::io::net::ip::Ipv4Addr;
use std::collections::HashMap;

fn main() {

    let mut server = Nickel::new();

    fn root_handler (_request: &Request, response: &mut Response) {
        let mut data = HashMap::<&'static str, &'static str>::new();
        data.insert("name", "user");
        response.render("examples/assets/template.tpl", &data);
    }

    server.get("/", root_handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
