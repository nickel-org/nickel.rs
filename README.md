# [nickel.rs](http://nickel-org.github.io) [![Build Status](https://travis-ci.org/nickel-org/nickel.rs.svg?branch=master)](https://travis-ci.org/nickel-org/nickel.rs) [![](http://meritbadge.herokuapp.com/nickel)](https://crates.io/crates/nickel) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/nickel-org/nickel.rs/master/LICENSE) [![Join the chat at https://gitter.im/nickel-org/nickel.rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/nickel-org/nickel.rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[nickel.rs](http://nickel-org.github.io) is a simple and lightweight foundation for web applications written in Rust. Its API is inspired by the popular express framework for JavaScript.

## Hello world

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

```toml
[package]

name = "my-nickel-app"
version = "0.0.1"
authors = ["yourname"]

[dependencies.nickel]
version = "*"
# If you are using the 'nightly' rust channel you can uncomment
# the line below to activate unstable features
# features = ["unstable"]

# Some examples require the `rustc_serialize` crate, which will
# require uncommenting the lines below
# [dependencies]
# rustc-serialize = "*"
```

You can then compile this using *Cargo build* and run it using *Cargo run*. After it's running you should visit http://localhost:6767 to see your hello world!

## More examples

More examples can be found [in the examples directory](/examples/) and the full documentation can be [found here](https://docs.rs/nickel/).

## Contributing

[nickel.rs](http://nickel-org.github.io) is a community effort. We welcome new contributors with open arms. Please read the [contributing guide here](/contributing.md) first.

If you're looking for inspiration, there's list of [open issues](https://github.com/nickel-org/nickel/issues?state=open) right here on github.

If you need a helping hand reach out to [@jolhoeft](https://github.com/jolhoeft), [@cburgdorf](https://github.com/cburgdorf), [@Ryman](https://github.com/Ryman) or [@SimonPersson](https://github.com/SimonPersson).

And hey, did you know you can also contribute by just starring the project here on github :)

### Development Plan

| Version | Branch       | Description                                                            |
| ------- | ------------ | -------------------------------------------------- |
| 0.11.x  | maint-0.11.x | hyper-0.10.x (synchronous version), bug fixes only |
| 0.12.x  | master       | hyper-0.14.x (asynchronous version)                |
| 0.13.x  |              | new features, possibly will be 1.0 instead         |
