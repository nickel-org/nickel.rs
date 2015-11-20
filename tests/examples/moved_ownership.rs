use util::{run_example, read_url};

#[test]
fn prints_borrowed_data() {
    run_example("moved_ownership", |port| {
        let url = format!("http://localhost:{}/", port);
        let s = read_url(&url);

        assert_eq!(s, "Expensive computation");
    })
}
