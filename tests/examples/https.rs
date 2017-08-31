use std::sync::Arc;

use hyper::net::{HttpsConnector,Openssl};
use hyper::client::Client;

use util::{run_example, read_body_to_string};

#[test]
fn https() {
    run_example("https", |port| {
        let paths = ["", "foo", "bar.html", "foo-barrrr/baz"];

        let context = create_context();
        for path in &paths {
            let url = format!("https://localhost:{}/{}", port, path);
            // Cannot use read_url() with SSL
            let s = read_url_https(&url, context.clone());

            assert_eq!(s, "Hello World from HTTPS");
        }
    })
}

fn create_context() -> Openssl {
    let mut openssl = Openssl::default();

    // Trick so we don't depend on the crate openssl directly
    Arc::get_mut(&mut openssl.context)
        .unwrap()
        .set_CA_file("examples/assets/self_signed.crt")
        .unwrap();
    openssl
}

fn read_url_https(url: &str, ssl: Openssl) -> String {
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let res = client.get(url)
                        .send()
                        .unwrap();
    read_body_to_string(&mut res)
}
