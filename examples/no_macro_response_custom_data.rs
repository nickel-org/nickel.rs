extern crate nickel;
extern crate plugin;
extern crate typemap;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use plugin::Extensible;
use typemap::Key;

/**
 * This example shows how to manipulate data in a middleware stack.
 * 
 * This is quite different from the no_macro_custom_data server example, as here the structure will be uniq to each request
 * processed by the server.
 */

/**
 * Struct holding data being passed along middleware
 * 
 * This need to implement the Key trait to be inserted into the TypeMap
 */
struct MyData {
    name: String,
}

impl Key for MyData {
    type Value = MyData;
}

/**
 * First middleware: Load the structure in the response
 * Side note: req can also have a map field you can use to store custom information
 */
fn init_data<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let data = MyData {
        name: "".to_string()
    };

    res.extensions_mut().insert::<MyData>(data);
    res.next_middleware()
}

/**
 * Second: The handler will get the structure and set some stuff in it
 */
fn handler<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // Get a mutable reference to the data so it can be updated
    let data = res.extensions_mut().get_mut::<MyData>().unwrap();

    data.name = req.param("some_stuff").unwrap().to_string();
    res.next_middleware()
}

/**
 * Last middleware block: Load the structure and send the response
 * note: You have to use an intermediate get_data function so you can borrow the Response more easily
 */
fn get_data(res: &Response) -> String {
    let data = res.extensions().get::<MyData>().unwrap();

    data.name.clone()
}

fn end_data<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let name = get_data(&res);
    // let data = res.extensions().get::<MyData>().unwrap(); // Would crash build since this borrow res and prevent its reuse

    res.send(format!("Hello {}", name))
}

/**
 * Note that the order you declare the middleware and handler are important here
 */
fn main() {
    let mut server = Nickel::new();

    server.utilize(init_data);
    server.get("/data/:some_stuff", handler);
    server.utilize(end_data);
    server.listen("127.0.0.1:6767").unwrap();
}
