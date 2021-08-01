use reqwest::Method;
use reqwest::blocking::{get, Client, Response};

use std::collections::HashSet;
use std::process::{Child, Command, Stdio};
use std::{env, thread, time};
use std::io::{BufReader, BufRead, Read};
use std::sync::Mutex;

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
    let client = reqwest::blocking::Client::new();
    client.post(url).body(body.to_string()).send().unwrap()
}

pub fn response_for_method(method: Method, url: &str) -> Response {
    let client = reqwest::blocking::Client::new();
    client.request(method, url).send().unwrap()
}

pub fn response_for(url: &str) -> Response {
    get(url).unwrap()
}

pub fn read_body_to_string(mut res: Response) -> String {
    res.text().unwrap()
}

pub fn read_url(url: &str) -> String {
    let mut res = response_for(url);
    read_body_to_string(res)
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

    //thread::sleep(time::Duration::from_secs(5));
    let mut bomb = Bomb(child);
    let port = parse_port(&mut bomb);

    f(port);
}

pub fn run_example_with_env<F>(name: &str, env_var: &str, env_val: &str, f: F)
where F: FnOnce(u16) {
    cargo_build(name);

    let command = format!("target/debug/examples/{}", name);
    let child = Command::new(&command)
        .env("NICKEL_TEST_HARNESS", "1")
        .env(env_var, env_val)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    //thread::sleep(time::Duration::from_secs(5));
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
