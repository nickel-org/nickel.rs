use futures::future::{ok, Future};
use futures::stream::Stream;
use hyper::{Chunk, Client, Method, Request, Response};
use tokio_core;

use std::collections::HashSet;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::io::{BufReader, BufRead, Read};
use std::str::from_utf8;
use std::sync::Mutex;
use std::env;

struct Bomb(Child);

// Don't leak child processes!
impl Drop for Bomb {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.0);
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

pub fn response_for_post<F>(url: &str, body: &str, handler: F) where F: FnOnce(Response) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = Client::configure().build(&core.handle());
    let mut req = Request::new(Method::Post, url.parse().unwrap());
    req.set_body(body.to_string());
    let work = client.request(req).and_then(|res| {
        handler(res);
        ok(())
    });
    core.run(work).unwrap()
}

pub fn response_for_method<F>(method: Method, url: &str, handler: F) where F: FnOnce(Response) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = Client::new(&core.handle());
    let req = Request::new(method, url.parse().unwrap());
    let work = client.request(req).and_then(|res| {
        handler(res);
        ok(())
    });
    core.run(work).unwrap()
}

pub fn response_for<F>(url: &str, handler: F) where F: FnOnce(Response) {
    response_for_method(Method::Get, url, handler)
}

pub fn for_body<F>(res: Response, f: F) where F: FnOnce(Chunk) {
    res.body().concat2().map(|b| f(b));
}

pub fn for_body_as_string<F>(res: Response, f: F) where F: FnOnce(String) {
    for_body(res, |b| {
        let s = from_utf8(&b).map(|s| s.to_string()).unwrap();
        f(s)
    });
}

pub fn read_url(url: &str) -> String {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = Client::new(&core.handle());
    let req = Request::new(Method::Get, url.parse().unwrap());
    let work = client.request(req).and_then(|response| {
        response.body().concat2().map(|b| {
            from_utf8(&b).map(|s| s.to_string()).unwrap()
        })
    });
    core.run(work).unwrap()
}

pub fn run_example<F>(name: &str, f: F)
where F: FnOnce(u16) {
    cargo_build(name);

    let command = format!("target/debug/examples/{}", name);
    println!("run_example: command = {:?}", command);
    let child = Command::new(&command)
                        .env("NICKEL_TEST_HARNESS", "1")
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();
    println!("run_example: child = {:?}", child);
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
    let mut built_set = BUILD_LOCK.lock().expect("Failed to get build lock");

    if !built_set.insert(name.to_owned()) {
        // insert returned false, key was already in the set
        return
    }

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
    static ref BUILD_LOCK : Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn parse_port(&mut Bomb(ref mut process): &mut Bomb) -> u16 {
    let stdout = BufReader::new(process.stdout.as_mut().unwrap());

    let line = stdout.lines()
        .map(Result::unwrap)
        .inspect(|line| println!("Processing Stdout: {:?}", line))
        .filter(|line| line.starts_with("Listening"))
        .next()
        .expect("Didn't find a line from stdout");

    let port = line.rsplitn(2, ':')
        .next()
        .and_then(|s| s.parse().ok())
        .expect(&format!("Failed to parse port from {:?}", line));

    println!("Parsed: port={} from {:?}", port, line);
    port
}
