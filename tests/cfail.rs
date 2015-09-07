#![cfg(feature = "unstable")]
extern crate compiletest_rs as compiletest;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::default_config();
    let cfg_mode = mode.parse().ok().expect("Invalid mode");

    if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        config.target = "x86_64-apple-darwin".into();
    }
    config.target_rustcflags = Some("--extern nickel=target/debug/libnickel.rlib -L target/debug -L target/debug/deps".to_string());
    config.mode = cfg_mode;
    config.src_base = format!("tests/{}", mode).into();

    if cfg!(not(feature = "secure_cookies")) {
        config.filter = Some("inference".into());
    }

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
