#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter, FormBody, Request, Response, MiddlewareResult};
use std::collections::HashMap;

fn root<'mw>(_req: &mut Request<'mw>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::new();
    data.insert("title","Contact");

    return res.render("examples/form_data/views/contact.html", &data)
}

fn confirmation<'mw>(req: &mut Request<'mw>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let form_data = try_with!(res, req.form_body());

    println!("{:?}", form_data);

    let mut data = HashMap::new();
    data.insert("title", "Confirmation");
    data.insert("firstname", form_data.get("firstname").unwrap_or("First name?"));
    data.insert("lastname", form_data.get("lastname").unwrap_or("Last name?"));
    data.insert("phone", form_data.get("phone").unwrap_or("Phone?"));
    data.insert("email", form_data.get("email").unwrap_or("Email?"));
    return res.render("examples/form_data/views/confirmation.html", &data)
}

fn main() {
    let mut server = Nickel::new();

    server.get("/", root);

    server.post("/confirmation", confirmation);

    server.listen("0.0.0.0:8080").unwrap();
}
