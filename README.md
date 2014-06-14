Floor
=======

Floor is supposed to be a simple and lightweight foundation for web applications written in Rust. It's API is inspired by the popular express framework for JavaScript.

Some of the features are:

* Easy handlers: A handler is just a function that takes a `Request` and `ResponseWriter`
* Variables in routes. Just write `my/route/:someid`
* Easy parameter access: `request.params.get(&"someid")`

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

    fn fooHandler (request: Request, response: &mut ResponseWriter) -> () {

        let text = String::new()
                    .append("This is user: ")
                    .append(request.params.get(&"userid".to_string()).as_slice());

        response.write(text.as_bytes()); 
    };

    fn barHandler (request: Request, response: &mut ResponseWriter) -> () { 
        response.write("This is the /bar handler".as_bytes()); 
    };

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", fooHandler);

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", barHandler);

    server.listen(6767);
}
```

##License

Floor is open source and licensed with the [MIT license](https://github.com/cburgdorf/Floor/blob/master/LICENSE)


##Contributing

I would love to find a helping hand. Especially if you know Rust, because I don't :)
There is list of [open issues](https://github.com/cburgdorf/Floor/issues?state=open) right here on github.
And hey, did you know you can also contribute by just starring the project here on github :)
