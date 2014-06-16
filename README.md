Floor
=======

Floor is supposed to be a simple and lightweight foundation for web applications written in Rust. It's API is inspired by the popular express framework for JavaScript.

Some of the features are:

* Easy handlers: A handler is just a function that takes a `Request` and `ResponseWriter`
* Variables in routes. Just write `my/route/:someid`
* Easy parameter access: `request.params.get(&"someid")`
* simple wildcard routes: `/some/*/route`
* double wildcard routes: `/a/**/route`

##[Jump to the Full Documentation](http://cburgdorf.github.io/Floor/doc/floor/index.html)

#Getting started
The easiest way to get started is to get the example running and play around with it. Let's do that real quick!

##Clone the repository

```shell
git clone --recursive https://github.com/cburgdorf/Floor.git
```

##Build Floor

```shell
make all
```

##Run the example

```shell
make run
```

Then try `localhost:6767/user/4711` and `localhost:6767/bar` 


##Take a look at the example code
Here is how sample server in `example.rs` looks like:
```rust
extern crate http;
extern crate floor;

use floor::{ Floor, Request };
use http::server::{ ResponseWriter };

fn main() {

    let mut server = Floor::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    fn user_handler (request: Request, response: &mut ResponseWriter) {

        let text = String::new()
                    .append("This is user: ")
                    .append(request.params.get(&"userid".to_string()).as_slice());

        response.write(text.as_bytes()); 
    };

    fn bar_handler (request: Request, response: &mut ResponseWriter) { 
        response.write("This is the /bar handler".as_bytes()); 
    };

    fn simple_wildcard (request: Request, response: &mut ResponseWriter) { 
        response.write("This matches /some/crazy/route but not /some/super/crazy/route".as_bytes()); 
    };

    fn double_wildcard (request: Request, response: &mut ResponseWriter) { 
        response.write("This matches /a/crazy/route and also /a/super/crazy/route".as_bytes()); 
    };

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", user_handler);

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    // go to http://localhost:6767/some/crazy/route to see this route in action
    server.get("/some/*/route", simple_wildcard);

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    server.get("/a/**/route", double_wildcard);

    server.listen(6767);
}
```

##[Jump to the Full Documentation](http://cburgdorf.github.io/Floor/doc/floor/index.html)

##License

Floor is open source and licensed with the [MIT license](https://github.com/cburgdorf/Floor/blob/master/LICENSE)


##Contributing

I would love to find a helping hand. Especially if you know Rust, because I don't :)
There is list of [open issues](https://github.com/cburgdorf/Floor/issues?state=open) right here on github.
And hey, did you know you can also contribute by just starring the project here on github :)
