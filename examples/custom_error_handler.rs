#[macro_use] extern crate nickel;

use nickel::{Nickel, NickelError, Request, HttpRouter, Action};
use nickel::status::StatusCode;

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", middleware! { |request|
        if let Some("42") = request.param("userid") {
            (StatusCode::OK, "User 42 was found!")
        } else {
            (StatusCode::IM_A_TEAPOT, "Teapot activated!")
        }
    });

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_handler<D>(err: &mut NickelError<'_, D>, req: &mut Request<'_, D>) -> Action {
        // Print the internal error message and path to the console
        println!("[{}] ERROR: {}",
                 req.path_without_query(),
                 err.message);

        if let Some(ref mut res) = err.stream {
            match res.status() {
                StatusCode::IM_A_TEAPOT => {
                    // Pass the internal message to the client
                    let _ = res.set_body(err.message.clone());
                    return Action::Halt(())
                }
                StatusCode::NOT_FOUND => {
                    let _ = res.set_body("<h1>404 - Not Found</h1>");
                    return Action::Halt(())
                }
                _ => {}
            }
        }

        // Fall through to next error handler
        Action::Continue(())
    }

    // issue #20178
    let custom_handler: fn(&mut NickelError<'_, ()>, &mut Request<'_, ()>) -> Action = custom_handler;

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767").await.unwrap();
}
