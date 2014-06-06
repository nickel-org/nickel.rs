Floor
=======

This may or may not become and expressjs inspired web framework for Rust. At it's current stage it's actually just a very tiny experiment and far from being a full framework.

##How to build Floor

```shell
make all
```

##How to run the example

```shell
make run
```

Then try `localhost:6767/foo` and `localhost:6767/bar` 


##Write your server
Here is how sample server in `example.rs` looks like:
```rust
extern crate http;
extern crate floor;

use floor::{ Floor };
use http::server::{ Request, ResponseWriter };

fn main() {

    let mut server = Floor::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    fn fooHandler (request: &Request, response: &mut ResponseWriter) -> () {
        response.write("hello from foo".as_bytes()); 
    };

    fn barHandler (request: &Request, response: &mut ResponseWriter) -> () { 
        response.write("hello from bar".as_bytes()); 
    };

    server.get("/foo", fooHandler);
    server.get("/bar", barHandler);

    server.listen(6767);
}
```

##Contributing

I would love to find a helping hand. Especially if you know Rust, because I don't :)
