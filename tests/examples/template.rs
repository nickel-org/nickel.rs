use crate::util::{run_example, read_url};

// TODO: when did this stop being correct, and why?
static EXPECTED : &'static str ="
<html>
    <head>
        <title>
            nickel.rs - example
        </title>
    </head>
    <body>
    <h1>
        Hello user!
    </h1>
    </body>
</html>";

// static EXPECTED : &'static str ="
// <h1>
//         Hello user!
//     </h1>
// ";

#[test]
fn renders_data() {
    run_example("template", |port| {
        let url = format!("http://localhost:{}/", port);
        let s = read_url(&url);

        assert_eq!(s.trim(), EXPECTED.trim());
    })
}
