#[macro_use] extern crate nickel;

#[cfg(not(feature = "with-serde"))]
extern crate rustc_serialize;
#[cfg(feature = "with-serde")]
extern crate serde_json;
#[cfg(feature = "with-serde")]
extern crate serde;

use nickel::status::StatusCode;
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};

#[cfg(not(feature = "with-serde"))]
mod json {
    use rustc_serialize::json::{ ToJson, Json };

    pub fn to_json<T: ToJson>(json: T) -> Json {
        json.to_json()
    }
}
#[cfg(feature = "with-serde")]
mod json {
    use serde;
    use serde_json::{ self, Value };

    pub fn to_json<T: serde::Serialize>(json: T) -> Value {
        serde_json::to_value(&json)
    }
}

use json::to_json;

#[cfg(not(feature = "with-serde"))]
mod person {
    use std::collections::BTreeMap;
    use rustc_serialize;
    
    #[derive(RustcEncodable, RustcDecodable)]
    pub struct Person {
        pub first_name: String,
        pub last_name:  String,
    }
    impl rustc_serialize::json::ToJson for Person {
        fn to_json(&self) -> rustc_serialize::json::Json {
            let mut map = BTreeMap::new();
            map.insert("first_name".to_string(), self.first_name.to_json());
            map.insert("last_name".to_string(), self.last_name.to_json());
            rustc_serialize::json::Json::Object(map)
        }
    }
}

#[cfg(feature = "with-serde")]
mod person {
    use serde;

    pub struct Person {
        pub first_name: String,
        pub last_name:  String,
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
                    Ok(Some(try!(serializer.serialize_struct_elt("first_name", &self.value.first_name))))
                }
                1 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("last_name", &self.value.last_name))))
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
                        "first_name" => Ok(PersonField::FirstName),
                        "last_name" => Ok(PersonField::LastName),
                        _ => Err(serde::de::Error::custom("expected first_name or last_name")),
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
            static FIELDS: &'static [&'static str] = &["first_name", "last_name"];
            deserializer.deserialize_struct("Person", FIELDS, PersonVisitor)
        }
    }

    struct PersonVisitor;

    impl serde::de::Visitor for PersonVisitor {
        type Value = Person;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Person, V::Error>
            where V: serde::de::MapVisitor
        {
            let mut first_name = None;
            let mut last_name = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(PersonField::FirstName) => { first_name = Some(try!(visitor.visit_value())); }
                    Some(PersonField::LastName) => { last_name = Some(try!(visitor.visit_value())); }
                    None => { break; }
                }
            }

            let first_name = match first_name {
                Some(first_name) => first_name,
                None => try!(visitor.missing_field("first_name")),
            };

            let last_name = match last_name {
                Some(last_name) => last_name,
                None => try!(visitor.missing_field("last_name")),
            };

            try!(visitor.end());

            Ok(Person{ first_name: first_name, last_name: last_name })
        }
    }
}

use self::person::Person;

fn main() {
    let mut server = Nickel::new();

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    server.post("/", middleware! { |request, response|
        let person = try_with!(response, {
            request.json_as::<Person>().map_err(|e| (StatusCode::BadRequest, e))
        });
        format!("Hello {} {}", person.first_name, person.last_name)
    });

    // go to http://localhost:6767/your/name to see this route in action
    server.get("/:first/:last", middleware! { |req|
        // These unwraps are safe because they are required parts of the route
        let first_name = req.param("first").unwrap();
        let last_name = req.param("last").unwrap();

        let person = Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };
        to_json(person)
    });

    // go to http://localhost:6767/content-type to see this route in action
    server.get("/raw", middleware! { |_, mut response|
        response.set(MediaType::Json);
        r#"{ "foo": "bar" }"#
    });

    server.listen("127.0.0.1:6767").unwrap();
}
