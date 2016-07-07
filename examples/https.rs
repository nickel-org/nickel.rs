#[macro_use] extern crate nickel;
extern crate hyper;

#[cfg(feature = "ssl")]
fn main() {
    use hyper::net::Openssl;

    use nickel::{Nickel, HttpRouter};

    let ssl = Openssl::with_cert_and_key("examples/assets/self_signed.crt", "examples/assets/key.pem").unwrap();
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World from HTTPS"));
    
    /*
     * Hint: If you want to use the `listen_https` function in your own crate / module / project, you need to
     * add `nickel/ssl` to the features section of your crate configuration, to get nickel compiled
     * with ssl feature activated:
     * 
     * Cargo.toml >
     *      [features]
     *      ssl = ["nickel/ssl"]
     *
     * As you can see, to append `hyper/ssl` in the features section is obsolete, as hyper will get compiled with
     * ssl feature activated via `nickel/ssl`.
    */
    server.listen_https("127.0.0.1:6767", ssl);
}

#[cfg(not(feature = "ssl"))]
fn main() {
    println!("Please run this example with the feature \"ssl\" enabled\n\
             $ cargo run --example https --features ssl");
}
