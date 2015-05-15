nickel.rs
=======
[![Build Status](https://travis-ci.org/nickel-org/nickel.rs.png?branch=master)](https://travis-ci.org/nickel-org/nickel.rs)
[![](http://meritbadge.herokuapp.com/nickel)](https://crates.io/crates/nickel)
[![Join the chat at https://gitter.im/nickel-org/nickel.rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/nickel-org/nickel.rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

nickel is supposed to be a simple and lightweight foundation for web applications written in Rust. It's API is inspired by the popular express framework for JavaScript.

Some of the features are:

* Easy handlers: A handler is just a function that takes a `&Request` and `&mut Response`
* Variables in routes. Just write `my/route/:someid`
* Easy parameter access: `request.param("someid")`
* simple wildcard routes: `/some/*/route`
* double wildcard routes: `/a/**/route`
* middleware
    * static file support

##[Jump to the nickel.rs website](http://nickel.rs)

#Getting started
The easiest way to get started is to get the example running and play around with it. Let's do that real quick!

##Clone the repository

```shell
git clone --recursive https://github.com/nickel-org/nickel.git
```

##Build nickel

```shell
cargo build --release
```

##Run the tests

```shell
cargo test
```

##Run the example

```shell
cargo run --example example
```

Then try `localhost:6767/user/4711` and `localhost:6767/bar`

### Note about nightly

To build on nightly, you will need to add `--features nightly` to the end of the above commands. Alternatively, if depending on the library you will need to add the following to your `Cargo.toml`.

```not_rust
[dependencies.nickel]
version = "*"
features = ["nightly"]
```

##Hello World!
Here's a simple server, for a longer example check out the examples folder.

```rust,no_run
#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen("127.0.0.1:6767");
}
```

##[Jump to the Full Documentation](http://nickel-org.github.io/nickel/)

##License

Nickel is open source and licensed with the [MIT license](https://github.com/nickel-org/nickel/blob/master/LICENSE)


##Contributing

Nickel.rs is a community effort. We welcome new contributors with open arms.
There is list of [open issues](https://github.com/nickel-org/nickel/issues?state=open) right here on github.

If you need a helping hand reach out to [@cburgdorf](https://github.com/cburgdorf), [@Ryman](https://github.com/Ryman) or [@SimonPersson](https://github.com/SimonPersson).

Make sure to follow this [commit message convention](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) because we will auto generate a changelog with [clog](https://github.com/thoughtram/clog) in the future.

And hey, did you know you can also contribute by just starring the project here on github :)
