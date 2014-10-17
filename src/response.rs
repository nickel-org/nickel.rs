use std::sync::RWLock;
use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};
use std::io::{IoResult, File};
use std::io::util::copy;
use std::path::BytesContainer;
use serialize::Encodable;
use http;
use http::server::ResponseWriter;
use time;
use mimes::get_media_type;
use mustache;
use mustache::{Template, Encoder, Error};

pub type TemplateCache = RWLock<HashMap<&'static str, Template>>;

///A container for the response
pub struct Response<'a, 'b: 'a> {
    ///the original `http::server::ResponseWriter`
    pub origin: &'a mut ResponseWriter<'b>,
    templates: &'a TemplateCache
}

impl<'a, 'b> Response<'a, 'b> {
    pub fn from_internal<'c, 'd>(response: &'c mut ResponseWriter<'d>,
                                 templates: &'c TemplateCache)
                                -> Response<'c, 'd> {
        Response {
            origin: response,
            templates: templates
        }
    }

    /// Sets the content type by it's short form.
    /// Returns the response for chaining.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.content_type("html");
    /// }
    /// ```
    pub fn content_type(&mut self, text: &str) -> &mut Response<'a,'b> {
        self.origin.headers.content_type = get_media_type(text);
        self
    }

    /// Sets the status code and returns the response for chaining
    ///
    /// # Example
    /// ```{rust}
    /// # extern crate http;
    /// # extern crate nickel;
    /// # use nickel::{Request, Response};
    /// # fn main() {
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.status_code(http::status::NotFound);
    /// }
    /// # }
    /// ```
    pub fn status_code(&mut self, status: http::status::Status) -> &mut Response<'a,'b> {
        self.origin.status = status;
        self
    }

    /// Writes a response
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("hello world");
    /// }
    /// ```
    pub fn send<T: BytesContainer> (&mut self, text: T) {
        // TODO: This needs to be more sophisticated to return the correct headers
        // not just "some headers" :)
        Response::set_headers(self.origin);
        let _ = self.origin.write(text.container_as_bytes());
    }

    fn set_headers(response_writer: &mut http::server::ResponseWriter) {
        let ref mut headers = response_writer.headers;
        headers.date = Some(time::now_utc());

        // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
        headers.content_length = None;

        headers.server = Some(String::from_str("Nickel"));
    }

    /// Writes a file to the output.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     let favicon = Path::new("/assets/favicon.ico");
    ///     response.send_file(&favicon).ok().expect("Failed to send favicon");
    /// }
    /// ```
    pub fn send_file(&mut self, path: &Path) -> IoResult<()> {
        let mut file = try!(File::open(path));
        self.origin.headers.content_length = None;

        self.origin.headers.content_type = path.extension_str().and_then(get_media_type);
        self.origin.headers.server = Some(String::from_str("Nickel"));
        copy(&mut file, self.origin)
    }

    /// Renders the given template bound with the given data.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// # use std::collections::hashmap::HashMap;
    /// fn handler(request: &Request, response: &mut Response) {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     response.render("examples/assets/template.tpl", &data);
    /// }
    /// ```
    pub fn render<'a, T: Encodable<Encoder<'a>, Error>>
        (&mut self, path: &'static str, data: &T) {
            // Fast path doesn't need writer lock
            match self.templates.read().find(&path) {
                Some(t) => {
                    let _ = t.render(self.origin, data);
                    return
                },
                None => {}
            }

            // We didn't find the template, get writers lock
            let mut templates = self.templates.write();
            // Search again incase there was a race to compile the template
            let template = match templates.entry(path) {
                Vacant(entry) => {
                    let mut file = File::open(&Path::new(path));
                    let raw_template =
                        file.read_to_string()
                            .ok()
                            .expect(format!("Couldn't open the template file: {}",
                                            path).as_slice());

                    entry.set(mustache::compile_str(raw_template.as_slice()))
                },
                Occupied(entry) => entry.into_mut()
            };

            let _ = template.render(self.origin, data);
    }
}

#[test]
fn matches_content_type () {
    let path = &Path::new("test.txt");
    let content_type = path.extension_str().and_then(get_media_type).unwrap();

    assert_eq!(content_type.type_.as_slice(), "text");
    assert_eq!(content_type.subtype.as_slice(), "plain");
}
