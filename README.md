# [nickel.rs](http://nickel.rs) [![Build Status](https://travis-ci.org/nickel-org/nickel.rs.png?branch=master)](https://travis-ci.org/nickel-org/nickel.rs) [![](http://meritbadge.herokuapp.com/nickel)](https://crates.io/crates/nickel) [![Join the chat at https://gitter.im/nickel-org/nickel.rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/nickel-org/nickel.rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[nickel.rs](http://nickel.rs) is a simple and lightweight foundation for web applications written in Rust. Its API is inspired by the popular express framework for JavaScript.

##Hello world

```rust,no_run
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:6767");
}
```

### Dependencies

You'll need to create a *Cargo.toml* that looks like this;

```not_rust
[package]

name = "my-nickel-app"
version = "0.0.1"
authors = ["yourname"]

[dependencies.nickel]
version = "*"
# If you are using the 'nightly' rust channel you can uncomment
# the line below to activate unstable features
# features = ["unstable"]
```

You can then compile this using *Cargo build* and run it using *Cargo run*. After it's running you should visit http://localhost:6767 to see your hello world!

## More examples

More examples can be found [in the examples directory](/examples/) and the full documentation can be [found here](http://nickel-org.github.io/nickel/).

##License

[MIT license](https://github.com/nickel-org/nickel/blob/master/LICENSE)

##Contributing

[nickel.rs](http://nickel.rs) is a community effort. We welcome new contributors with open arms. Please read the [contributing guide here](/contributing.md) first.

If you're looking for inspiration, there's list of [open issues](https://github.com/nickel-org/nickel/issues?state=open) right here on github.

If you need a helping hand reach out to [@cburgdorf](https://github.com/cburgdorf), [@Ryman](https://github.com/Ryman) or [@SimonPersson](https://github.com/SimonPersson).

And hey, did you know you can also contribute by just starring the project here on github :)
