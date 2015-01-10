use std::sync::RwLock;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::old_io::{IoResult, File};
use std::old_io::util::copy;
use std::old_path::BytesContainer;
use serialize::Encodable;
use hyper::status::StatusCode;
use hyper::server::Response as HyperResponse;
use time;
use mimes;
use mustache;
use mustache::Template;

pub type TemplateCache = RwLock<HashMap<&'static str, Template>>;

///A container for the response
pub struct Response<'a, 'b: 'a> {
    ///the original `hyper::server::Response`
    pub origin: HyperResponse<'b>,
    templates: &'a TemplateCache
}

impl<'a, 'b> Response<'a, 'b> {
    pub fn from_internal<'c, 'd>(response: HyperResponse<'d>,
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
    /// use nickel::mimes::MediaType;
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.content_type(MediaType::Html);
    /// }
    /// ```
    pub fn content_type(&mut self, mt: mimes::MediaType) -> &mut Response<'a,'b> {
        self.origin.headers.content_type = Some(mimes::get_media_type(mt));
        self
    }

    /// Sets the status code and returns the response for chaining
    ///
    /// # Example
    /// ```{rust}
    /// # extern crate hyper;
    /// # extern crate nickel;
    /// # use nickel::{Request, Response};
    /// # fn main() {
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.status_code(hyper::status::StatusCode::NotFound);
    /// }
    /// # }
    /// ```
    pub fn status_code(&mut self, status: StatusCode) -> &mut Response<'a,'b> {
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
        let _ = self.origin.write_all(text.container_as_bytes());
    }

    fn set_headers(response_writer: &mut HyperResponse) {
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

        self.origin.headers.content_type = path.extension_str()
                                               .and_then(|s| s.parse().ok())
                                               .map(mimes::get_media_type);
        self.origin.headers.server = Some(String::from_str("Nickel"));
        copy(&mut file, self.origin)
    }

    /// Renders the given template bound with the given data.
    ///
    /// # Example
    /// ```{rust}
    /// # use nickel::{Request, Response};
    /// # use std::collections::HashMap;
    /// fn handler(request: &Request, response: &mut Response) {
    ///     let mut data = HashMap::new();
    ///     data.insert("name", "user");
    ///     response.render("examples/assets/template.tpl", &data);
    /// }
    /// ```
    pub fn render<'c, T: Encodable>
        (&mut self, path: &'static str, data: &T) {
            // Fast path doesn't need writer lock
            {
                let templates = self.templates.read().unwrap();
                if let Some(t) = templates.get(&path) {
                    let _ = t.render(self.origin, data);
                    return
                }
            }

            // We didn't find the template, get writers lock and
            let mut templates = self.templates.write().unwrap();
            // search again incase there was a race to compile the template
            let template = match templates.entry(path) {
                Vacant(entry) => {
                    let mut file = File::open(&Path::new(path));
                    let raw_template = file.read_to_string()
                                           .ok()
                                           .expect(&*format!("Couldn't open the template file: {}",
                                                            path));
                    entry.insert(mustache::compile_str(&*raw_template))
                },
                Occupied(entry) => entry.into_mut()
            };

            let _ = template.render(self.origin, data);
    }
}

#[test]
fn matches_content_type () {
    let path = &Path::new("test.txt");
    let content_type = path.extension_str().and_then(|s| s.parse().ok());

    assert_eq!(content_type, Some(mimes::MediaType::Txt));
    let content_type = content_type.map(mimes::get_media_type).unwrap();

    assert_eq!(content_type.type_, "text");
    assert_eq!(content_type.subtype, "plain");
}
