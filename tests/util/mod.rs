use hyper::client::{Client, Response};
use hyper::method::Method;

use std::process::{Child, Command, Stdio};
use std::thread;
use std::io::Read;
use std::sync::Mutex;
use std::env;

struct Bomb(Child);

// Don't leak child processes!
impl Drop for Bomb {
    fn drop(&mut self) {
        match self.0.kill() {
            Ok(()) => {},
            Err(e) => panic!("Leaking child process: {:?}", e)
        }

        if thread::panicking() {
            let mut s = String::new();
            let stdout = self.0.stdout.as_mut().unwrap();
            stdout.read_to_string(&mut s).unwrap();

            println!("Unparsed Stdout:\n{}", s);
        }
    }
}

pub fn response_for_post(url: &str, body: &str) -> Response {
    Client::new()
           .post(url)
           .body(body)
           .send()
           .unwrap()
}

pub fn response_for_method(method: Method, url: &str) -> Response {
    Client::new()
           .request(method, url)
           .send()
           .unwrap()
}

pub fn response_for(url: &str) -> Response {
    response_for_method(Method::Get, url)
}

pub fn read_body_to_string(res: &mut Response) -> String {
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    s
}

pub fn read_url(url: &str) -> String {
    let mut res = response_for(url);
    read_body_to_string(&mut res)
}

pub fn run_example<F>(name: &str, f: F)
where F: FnOnce(u16) {
    cargo_build(name);

    let command = format!("target/debug/examples/{}", name);
    let child = Command::new(&command)
                        .env("NICKEL_TEST_HARNESS", "1")
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

    let mut bomb = Bomb(child);
    let port = parse_port(&mut bomb);

    f(port);
}

// We cannot use `cargo run --example foo` as when a test fails
// we can only send SIGKILL, which cargo doesn't propogate to the
// child process. Rust currently doesn't seem to give us a way to
// use SIGTERM.
//
// We do a full build call rather than just checking if the executable
// exists, as the dependancies may have changed and then a user running
// `cargo test --test foo` to run the integration tests only will not
// pick up the changes.
fn cargo_build(name: &str) {
    // Don't let cargo build in parallel, it can cause unnecessary test failures
    // https://github.com/rust-lang/cargo/issues/354
    //
    // NOTE: important to assign to variable or it'll drop instantly.
    let _lock = BUILD_LOCK.lock();


    let mut command = Command::new("cargo");

    command.env("NICKEL_TEST_HARNESS", "1")
           .stdout(Stdio::piped())
           .arg("build")
           .arg("--example")
           .arg(name);

    // support for features passed in the env (as we do on travis)
    if let Some(arg) = env::var("FEATURES").ok() {
        for feature_arg in arg.split_whitespace() {
            command.arg(feature_arg);
        }
    }

    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}

lazy_static! {
    static ref BUILD_LOCK : Mutex<()> = Mutex::new(());
}

fn parse_port(&mut Bomb(ref mut process): &mut Bomb) -> u16 {
    // stdout doesn't implement BufRead... *shrug*
    let stdout = process.stdout.as_mut().unwrap();

    let mut line = String::new();

    for c in stdout.bytes().map(|b| b.unwrap() as char) {
        if c == '\n' {
            // Check if it's the line we want
            if line.starts_with("Listening") {
                break
            } else {
                println!("Skipping Stdout line: {:?}", line);
                line.clear();
            }
        } else {
            line.push(c)
        }
    }

    let port = {
        let s = line.rsplitn(2, ':').next().unwrap();
        match s.parse() {
            Ok(port) => port,
            Err(e) => panic!("Failed to parse port from: {:?} : {:?}", line, e)
        }
    };

    println!("Parsed: port={} from {:?}", port, line);
    port
}
