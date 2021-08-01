use crate::util::*;

use reqwest::blocking::Response;
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

