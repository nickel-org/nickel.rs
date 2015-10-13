extern crate mustache;
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Halt};
use mustache::MapBuilder;

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, res|
        let data = MapBuilder::new()
                              .insert_str("name", "user")
                              .insert_fn("helper", |text| {
                                  format!("<b>{}</b>", text.trim())
                              }).build();

        let template = try_with!(res, mustache::compile_path("examples/assets/template.tpl")
                                               .map_err(|e| format!("{:?}", e)));

        let mut stream = try!(res.start());
        template.render_data(&mut stream, &data);
        return Ok(Halt(stream))
    });

    server.listen("127.0.0.1:6767");
}
