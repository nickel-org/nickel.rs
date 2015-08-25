#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, res|
        let mut data = HashMap::new();
        data.insert("title","Contact");

        return res.render("examples/form_data/views/contact.html", &data)
    });

    server.post("/confirmation", middleware!{ |req, res|
        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data).unwrap();

        println!("{}", form_data);

        let mut data = HashMap::new();
        data.insert("title", "Confirmation");
        data.insert("formData", &form_data);

        return res.render("examples/form_data/views/confirmation.html", &data)
    });

    server.listen("0.0.0.0:8080");
}
