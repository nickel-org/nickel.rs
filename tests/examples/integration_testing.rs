use crate::util::*;

use reqwest::blocking::Response;
use reqwest::{header, StatusCode};
use serde_json::Value;
use std::{thread, time};

fn get_hits_after_delay(port: u16) -> u32 {
    // let other tests hit the server
    thread::sleep(time::Duration::from_secs(1));

    let url = format!{"http://127.0.0.1:{}/hits", port};
    println!{"Url: {}", url};
    let response = read_url(&url);
    response.parse().unwrap()
}

/// This test has a Server instance all to itself.
#[test]
fn non_shared_server() {
    run_example("integration_testing", |port| {
        assert_eq!(get_hits_after_delay(port), 1);
    })
}

// start of the sequence of tests
#[test]
fn test_sequence() {
    run_example("integration_testing", run_sequence)
    //run_example("integration_testing", root_responds_with_hello_world)
}

fn get(port: u16, path: &str) -> Response {
    let url = format!{"http://127.0.0.1:{}{}", port, path};
    println!{"Url: {}", url};
    response_for(&url)
}

fn post(port: u16, path: &str, body: &str) -> Response {
    let url = format!{"http://127.0.0.1:{}{}", port, path};
    println!{"Url: {}", url};
    response_for_post(&url, body)
}

fn run_sequence(port: u16) {
    root_responds_with_hello_world(port);
    root_responds_with_modified_json(port);
    accepts_json_with_missing_fields(port);
    doesnt_accept_bad_inputs(port);
    has_no_users_by_default(port);

    server_is_shared_with_other_tests(port);
}

fn root_responds_with_hello_world(port: u16) {
    let mut response = get(port, "/");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().unwrap(), "Hello World");
}

// FIXME: This will probably fail if tests are run without parallelism
fn server_is_shared_with_other_tests(port: u16) {
    assert!(get_hits_after_delay(port) > 1);
}

fn root_responds_with_modified_json(port: u16) {
    let mut response = post(port, "/", r#"{ "name": "Rust", "age": 1 }"#);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE),
        Some(&header::HeaderValue::from_static("application/json"))
    );

    let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();

    assert_eq!(json["message"].as_str(), Some("Hello Rust, your age is 1"));
}

fn accepts_json_with_missing_fields(port: u16) {
    let mut response = post(port, "/", r#"{ "name": "Rust" }"#);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE),
        Some(&header::HeaderValue::from_static("application/json"))
    );
    let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();

    assert_eq!(json["message"].as_str(), Some("Hello Rust, I don't know your age"));
}

fn doesnt_accept_bad_inputs(port: u16) {
    let response = post(port, "/", r#"{ }"#);
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

fn has_no_users_by_default(port: u16) {
    let mut response = get(port, "/users");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE),
        Some(&header::HeaderValue::from_static("application/json"))
    );
    let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();

    assert_eq!(json["users"].as_array().unwrap().len(), 0);
}

#[test]
fn non_shared_server_with_different_database() {
    run_example_with_env("integration_testing", "ALT_USERS", "1", |port| {
        let mut response = get(port, "/users");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&header::HeaderValue::from_static("application/json"))
        );
        let json: Value = serde_json::from_str(&response.text().unwrap()).unwrap();

        assert_eq!(json["users"].as_array().unwrap().len(), 3);
    })
}
