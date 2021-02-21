#[cfg(feature = "ssl")]
fn main() {
    use hyper::net::Openssl;

    use nickel::{Nickel, HttpRouter};

    let ssl = Openssl::with_cert_and_key("examples/assets/self_signed.crt", "examples/assets/key.pem").unwrap();
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World from HTTPS"));
    server.listen_https("127.0.0.1:6767", ssl).unwrap();
}

#[cfg(not(feature = "ssl"))]
fn main() {
    println!("Please run this example with the feature \"ssl\" enabled\n\
             $ cargo run --example https --features ssl");
}
