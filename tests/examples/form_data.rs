use hyper::client::Client;
use hyper::header::ContentType;
use hyper::status::StatusCode;
use util::{read_body_to_string, read_url, run_example};

#[test]
fn display_form() {
    run_example("form_data", |port| {
        let url = format!("http://localhost:{}/", port);
        let s = read_url(&url);
        assert!(s.contains(r#"<form>"#), "response didn't have a form");
    })
}

#[test]
fn post_with_data() {
    run_example("form_data", |port| {
        let url = format!("http://localhost:{}/confirmation", port);
        let ref mut res = Client::new()
            .post(&url)
            .header(ContentType::form_url_encoded())
            .body(r#"firstname=John&lastname=Doe&phone=911&email=john@doe.com"#)
            .send()
            .unwrap();
        let s = read_body_to_string(res);
        assert!(s.contains(r#"John Doe 911 john@doe.com"#), "response didn't have an expected data");
    })
}

#[test]
fn post_without_data() {
    run_example("form_data", |port| {
        let url = format!("http://localhost:{}/confirmation", port);
        let ref mut res = Client::new()
            .post(&url)
            .header(ContentType::form_url_encoded())
            .send()
            .unwrap();
        let s = read_body_to_string(res);
        assert!(s.contains(r#"First name? Last name? Phone? Email?"#), "response didn't have an expected data");
    })
}

#[test]
fn post_without_content_type() {
    run_example("form_data", |port| {
        let url = format!("http://localhost:{}/confirmation", port);
        let res = Client::new().post(&url).send().unwrap();
        assert_eq!(res.status, StatusCode::BadRequest);
    })
}
