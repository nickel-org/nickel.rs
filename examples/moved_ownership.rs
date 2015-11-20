#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    // For the purpose of this example, this represents any type which is not `Copy`.
    let x = "Expensive computation".to_string();

    // The middleware! macro expands into a closure which moves variables
    // when it captures them. This acts as a handler for all get requests,
    // which means it needs to be executed many times.
    //
    // If we only did `middleware!(x)` then we would get the error:
    //
    // `cannot move out of captured outer variable in an `Fn` closure [E0507]`
    //
    // This is because the body of the closure would expand to something similar to:
    //
    // `move |request, response| { response.send(x) }`
    //
    // The code fails to compile as the ownership of `x` has been moved, which
    // would leave it in an unusable state for future requests. Therefore, we use
    // the value by 'ref' by doing `&*x` (the additional `*` in this case is to
    // deref from `String` to `str`).
    server.get("**", middleware!(&*x));
    server.listen("127.0.0.1:6767");
}
