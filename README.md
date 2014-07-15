nickel.rs
=======

nickel is supposed to be a simple and lightweight foundation for web applications written in Rust. It's API is inspired by the popular express framework for JavaScript.

Some of the features are:

* Easy handlers: A handler is just a function that takes a `Request` and `ResponseWriter`
* Variables in routes. Just write `my/route/:someid`
* Easy parameter access: `request.params.get(&"someid")`
* simple wildcard routes: `/some/*/route`
* double wildcard routes: `/a/**/route`
* middleware
    * static file support 

##[Jump to the Full Documentation](http://nickel-org.github.io/nickel/)

#Getting started
The easiest way to get started is to get the example running and play around with it. Let's do that real quick!

##Clone the repository

```shell
git clone --recursive https://github.com/nickel-org/nickel.git
```

##Build nickel

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
extern crate serialize;
extern crate nickel;

use nickel::{ Nickel, Request, Response, FromFn };

#[deriving(Decodable, Encodable)]
pub struct Person {
    pub firstname: String,
    pub lastname:  String,
}

fn main() {

    let mut server = Nickel::new();
    
    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    //this is an example middleware function that just logs each request
    fn logger (request: &Request, _response: &mut Response) -> bool{
        println!("logging request: {}", request.origin.request_uri);
        
        // a request is supposed to return a `bool` to indicate whether additional
        // middleware should continue executing or should be stopped.
        true
    }

    // middleware is optional and can be registered with `utilize`
    server.utilize(FromFn::new(logger));

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(Nickel::static_files("examples/assets/"));

    fn user_handler (request: &Request, response: &mut Response) {
        let text = format!("This is user: {}", request.params.get(&"userid".to_string()));
        response.send(text.as_slice());
    }

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", user_handler);

    fn bar_handler (_request: &Request, response: &mut Response) { 
        response.send("This is the /bar handler"); 
    }

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    fn simple_wildcard (_request: &Request, response: &mut Response) { 
        response.send("This matches /some/crazy/route but not /some/super/crazy/route"); 
    }

    // go to http://localhost:6767/some/crazy/route to see this route in action
    server.get("/some/*/route", simple_wildcard);

    fn double_wildcard (_request: &Request, response: &mut Response) { 
        response.send("This matches /a/crazy/route and also /a/super/crazy/route"); 
    }

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    server.get("/a/**/route", double_wildcard);

    // this will cause json bodies automatically being parsed
    server.utilize(Nickel::json_body_parser());

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    fn post_handler (request: &Request, response: &mut Response) { 

        let person = request.json_as::<Person>().unwrap();
        let text = format!("Hello {} {}", person.firstname, person.lastname);
        response.send(text.as_slice()); 
    }

    // go to http://localhost:6767/a/post/request to see this route in action
    server.post("/a/post/request", post_handler);

    server.listen(6767);
}
```

##[Jump to the Full Documentation](http://nickel-org.github.io/nickel/)

##License

Nickel is open source and licensed with the [MIT license](https://github.com/nickel-org/nickel/blob/master/LICENSE)


##Contributing

I would love to find a helping hand. Especially if you know Rust, because I don't :)
There is list of [open issues](https://github.com/nickel-org/nickel/issues?state=open) right here on github.
And hey, did you know you can also contribute by just starring the project here on github :)
