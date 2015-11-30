#[macro_use] extern crate nickel;

use nickel::{Nickel, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    // For more complicated uses of `StaticFilesHandler`, you probably want to
    // use `Mount`. See the mount example for usage.

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    server.listen("127.0.0.1:6767");
}
