use std::sync::{Arc, RWLock};
use std::collections::HashMap;
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

///A container for the response
pub struct Response<'a, 'b: 'a> {
    ///the original `http::server::ResponseWriter`
    pub origin: &'a mut ResponseWriter<'b>,
    templates: Arc<RWLock<HashMap<&'static str, Template>>>
}

impl<'a, 'b> Response<'a, 'b> {
    pub fn from_internal<'c, 'd>(response: &'c mut ResponseWriter<'d>,
                                 templates: Arc<RWLock<HashMap<&'static str, Template>>>)
                                -> Response<'c, 'd> {
        Response {
            origin: response,
            templates: templates
        }
    }

    /// Writes a response
    ///
    /// # Example
    /// ```{rust,ignore}
    /// response.send("hello world");
    /// ```
    pub fn send<T: BytesContainer> (&mut self, text: T) {
        // TODO: This needs to be more sophisticated to return the correct headers
        // not just "some headers" :)
        Response::set_headers(self.origin);
        let _ = self.origin.write(text.container_as_bytes());
    }

    /// sets the content type by it's short form.
    ///
    /// # Example
    /// ```{rust,ignore}
    /// response.set_content_type("html");
    /// ```
    pub fn content_type(&mut self, text: &str) -> &mut Response<'a,'b> {
        // TODO: make this a chaining API. (Fight the lifetime hell!)
        self.origin.headers.content_type = get_media_type(text);
        self
    }

    fn set_headers(response_writer: &mut http::server::ResponseWriter) {
        let ref mut headers = response_writer.headers;
        headers.date = Some(time::now_utc());

        // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
        headers.content_length = None;
        if headers.content_type.is_none() {
            headers.content_type = get_media_type("txt");
        }

        headers.server = Some(String::from_str("Nickel"));
    }

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
    /// ```{rust,ignore}
    /// let mut data = HashMap::<&'static str, &'static str>::new();
    /// data.insert("name", "user");
    /// response.render("examples/assets/template.tpl", &data);
    /// ```
    pub fn render<'a, T: Encodable<Encoder<'a>, Error>>
        (&mut self, path: &'static str, data: &T) {
        // Fast path doesn't need writer lock
        let _ = match self.templates.read().find(&path) {
            Some(template) => template.render(self.origin, data),
            None => {
                // Search again incase there was a race to compile the template
                let mut templates = self.templates.write();
                let template = templates.find_or_insert_with(path, |_| {
                    let mut file = File::open(&Path::new(path));
                    let raw_template = file.read_to_string()
                                            .ok()
                                            .expect(format!("Couldn't open the template file: {}",
                                                            path).as_slice());
                    mustache::compile_str(raw_template.as_slice())
                });

                template.render(self.origin, data)
            }
        };
    }
}

#[test]
fn matches_content_type () {
    let path = &Path::new("test.txt");
    let content_type = path.extension_str().and_then(get_media_type).unwrap();

    assert_eq!(content_type.type_.as_slice(), "text");
    assert_eq!(content_type.subtype.as_slice(), "plain");
}
