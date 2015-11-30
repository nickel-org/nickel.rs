extern crate rustc_serialize;
extern crate hyper;
#[macro_use] extern crate lazy_static;

mod util;

mod examples {
    mod hello_world;
    mod mount;
    mod route_data;
    mod routing;
    mod template;
    mod moved_ownership;
    mod chaining;
    mod json;
    mod query_string;
    mod regex_route;
    mod custom_error_handler;
    mod static_files;
}
