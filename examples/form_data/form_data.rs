#[macro_use] extern crate nickel;
use async_trait::async_trait;
use nickel::{Nickel, HttpRouter, Request, Response, Middleware, MiddlewareResult};
use std::collections::HashMap;

struct Root;

#[async_trait]
impl Middleware<()> for Root {
    async fn invoke(&self, _req: &mut Request, res: Response) -> MiddlewareResult {

        let mut data = HashMap::new();
        data.insert("title","Contact");

        return res.render("examples/form_data/views/contact.html", &data).await
    }
}

struct Confirmation;

#[async_trait]
impl Middleware<()> for Confirmation {
    async fn invoke(&self, req: &mut Request, res: Response) -> MiddlewareResult {

        let form_data = try_with!(res, req.form_body().await);

        println!("{:?}", form_data);

        let mut data = HashMap::new();
        data.insert("title", "Confirmation");
        data.insert("firstname", form_data.get("firstname").unwrap_or("First name?"));
        data.insert("lastname", form_data.get("lastname").unwrap_or("Last name?"));
        data.insert("phone", form_data.get("phone").unwrap_or("Phone?"));
        data.insert("email", form_data.get("email").unwrap_or("Email?"));
        return res.render("examples/form_data/views/confirmation.html", &data).await
    }
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    server.get("/", Root);

    server.post("/confirmation", Confirmation);

    server.listen("0.0.0.0:8080").await.unwrap();
}
