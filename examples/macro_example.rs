#[macro_use] extern crate nickel;
extern crate regex;
extern crate hyper;

#[cfg(not(feature = "with-serde"))]
extern crate rustc_serialize;
#[cfg(feature = "with-serde")]
extern crate serde_json;
#[cfg(feature = "with-serde")]
extern crate serde;

use std::io::Write;
use nickel::status::StatusCode::{self, NotFound};
use nickel::{
    Nickel, NickelError, Continue, Halt, Request, Response, MediaType,
    QueryString, JsonBody, StaticFilesHandler, MiddlewareResult, HttpRouter, Action
};
use regex::Regex;
use hyper::header::Location;

#[cfg(not(feature = "with-serde"))]
mod person {
    use std::collections::BTreeMap;
    use rustc_serialize;
    use rustc_serialize::json::{ ToJson, Json };
    
    #[derive(RustcEncodable, RustcDecodable)]
    pub struct Person {
        pub firstname: String,
        pub lastname:  String,
    }
    impl ToJson for Person {
        fn to_json(&self) -> Json {
            let mut map = BTreeMap::new();
            map.insert("firstname".to_string(), self.firstname.to_json());
            map.insert("lastname".to_string(), self.lastname.to_json());
            Json::Object(map)
        }
    }
}

#[cfg(feature = "with-serde")]
mod person {
    use serde;

    pub struct Person {
        pub firstname: String,
        pub lastname:  String,
    }
    impl serde::Serialize for Person {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer
        {
            serializer.serialize_struct("Person", PersonMapVisitor {
                value: self,
                state: 0,
            })
        }
    }

    struct PersonMapVisitor<'a> {
        value: &'a Person,
        state: u8,
    }

    impl<'a> serde::ser::MapVisitor for PersonMapVisitor<'a> {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
            where S: serde::Serializer
        {
            match self.state {
                0 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("firstname", &self.value.firstname))))
                }
                1 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("lastname", &self.value.lastname))))
                }
                _ => {
                    Ok(None)
                }
            }
        }
    }

    enum PersonField {
        FirstName,
        LastName,
    }

    impl serde::Deserialize for PersonField {
        fn deserialize<D>(deserializer: &mut D) -> Result<PersonField, D::Error>
            where D: serde::de::Deserializer
        {
            struct PersonFieldVisitor;

            impl serde::de::Visitor for PersonFieldVisitor {
                type Value = PersonField;

                fn visit_str<E>(&mut self, value: &str) -> Result<PersonField, E>
                    where E: serde::de::Error
                {
                    match value {
                        "firstname" => Ok(PersonField::FirstName),
                        "lastname" => Ok(PersonField::LastName),
                        _ => Err(serde::de::Error::custom("expected firstname or lastname")),
                    }
                }
            }

            deserializer.deserialize(PersonFieldVisitor)
        }
    }

    impl serde::Deserialize for Person {
        fn deserialize<D>(deserializer: &mut D) -> Result<Person, D::Error>
            where D: serde::de::Deserializer
        {
            static FIELDS: &'static [&'static str] = &["firstname", "lastname"];
            deserializer.deserialize_struct("Person", FIELDS, PersonVisitor)
        }
    }

    struct PersonVisitor;

    impl serde::de::Visitor for PersonVisitor {
        type Value = Person;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Person, V::Error>
            where V: serde::de::MapVisitor
        {
            let mut firstname = None;
            let mut lastname = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(PersonField::FirstName) => { firstname = Some(try!(visitor.visit_value())); }
                    Some(PersonField::LastName) => { lastname = Some(try!(visitor.visit_value())); }
                    None => { break; }
                }
            }

            let firstname = match firstname {
                Some(firstname) => firstname,
                None => try!(visitor.missing_field("firstname")),
            };

            let lastname = match lastname {
                Some(lastname) => lastname,
                None => try!(visitor.missing_field("lastname")),
            };

            try!(visitor.end());

            Ok(Person{ firstname: firstname, lastname: lastname })
        }
    }
}

use self::person::Person;

//this is an example middleware function that just logs each request
fn logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    println!("logging request: {:?}", request.origin.uri);
    response.next_middleware()
}

//this is how to overwrite the default error handler to handle 404 cases with a custom view
fn custom_404<'a, D>(err: &mut NickelError<D>, _req: &mut Request<D>) -> Action {
    if let Some(ref mut res) = err.stream {
        if res.status() == NotFound {
            let _ = res.write_all(b"<h1>Call the police!</h1>");
            return Halt(())
        }
    }

    Continue(())
}

fn main() {
    let mut server = Nickel::new();

    // middleware is optional and can be registered with `utilize`
    server.utilize(logger);

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    let hello_regex = Regex::new("/hello/(?P<name>[a-zA-Z]+)").unwrap();

    // The return type for a route can be anything that implements `Responder`
    server.utilize(router!(
        // go to http://localhost:6767/user/4711 to see this route in action
        get "/user/:userid" => |request| {
            // returning a String
            format!("This is user: {}", request.param("userid").unwrap())
        }

        // go to http://localhost:6767/no_alloc/4711 to see this route in action
        get "/no_alloc/:userid" => |request, response| {
            // returning a slice of T where T: Display
            &["This is user: ", request.param("userid").unwrap()][..]
        }

        // go to http://localhost:6767/bar to see this route in action
        get "/bar" => {
            // returning a http status code and a static string
            (200u16, "This is the /bar handler")
        }

        // go to http://localhost:6767/content-type to see this route in action
        get "/content-type" => |_, mut response| {
            response.set(MediaType::Json);
            "{'foo':'bar'}"
        }

        // go to http://localhost:6767/hello/moomah to see this route in action
        get hello_regex => |request| {
            format!("Hello {}", request.param("name").unwrap())
        }

        // go to http://localhost:6767/redirect to see this route in action
        get "/redirect" => |_, mut response| {
            response.set(Location("http://nickel.rs".into()));

            StatusCode::PermanentRedirect
        }

        // go to http://localhost:6767/private to see this route in action
        get "/private" => {
            // returning a typed http status and a response body
            (StatusCode::Unauthorized, "This is a private place")
        }

        // go to http://localhost:6767/some/crazy/route to see this route in action
        get "/some/*/route" => {
            // returning a static string
            "This matches /some/crazy/route but not /some/super/crazy/route"
        }

        // go to http://localhost:6767/a/some/crazy/route to see this route in action
        get "/a/**/route" => {
            "This matches /a/crazy/route and also /a/super/crazy/route"
        }

        // try it with curl
        // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
        post "/a/post/request" => |request| {
            let person = request.json_as::<Person>().unwrap();
            format!("Hello {} {}", person.firstname, person.lastname)
        }

        // try calling http://localhost:6767/query?foo=bar
        get "/query" => |request| {
            let query = request.query();
            let foo = query.get("foo").unwrap_or("This is only a default value");
            let bar = query.get("bar").unwrap_or("This is only a default value");
            let text = format!("<p>Your foo values in the query string are: {:?}\
                                <p>Your bar values are: {:?}",
                               foo, bar);
            text
        }
    ));

    // issue #20178
    let custom_handler: fn(&mut NickelError<()>, &mut Request<()>) -> Action = custom_404;

    server.handle_error(custom_handler);

    println!("Running server!");
    server.listen("127.0.0.1:6767").unwrap();
}
