use util::{run_example, read_url};

fn t(file: &str) {
    run_example(file, |port| {
        let paths = ["", "foo", "bar.html", "foo-barrrr/baz"];

        for path in &paths {
            let url = format!("http://localhost:{}/{}", port, path);
            println!("hello_world: reading {:?}", url);
            let s = read_url(&url);
            println!("got {:?}", s);
            assert_eq!(s, "Hello World");
        }
    })
}

#[test] fn non_macro() { t("hello_world") }
#[test] fn _macro() { t("hello_world_macro") }
