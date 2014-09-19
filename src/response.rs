extern crate mustache;

use std::str;
use std::sync::{Arc, RWLock};
use std::collections::HashMap;
use std::io::{IoResult, File};
use std::io::util::copy;
use std::path::BytesContainer;
use http;
use time;
use mimes::get_media_type;

///A container for the response
pub struct Response<'a, 'b: 'a> {
    ///the original `http::server::ResponseWriter`
    pub origin: &'a mut http::server::ResponseWriter<'b>,
    templates: Arc<RWLock<HashMap<&'static str, mustache::Template>>>
}

impl<'a, 'b> Response<'a, 'b> {

    pub fn from_internal<'c, 'd>(response: &'c mut http::server::ResponseWriter<'d>,
                                 templates: Arc<RWLock<HashMap<&'static str, mustache::Template>>>
                                ) -> Response<'c, 'd> 
    {
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
    pub fn set_content_type(&mut self, text: &str) {
        // TODO: make this a chaining API. (Fight the lifetime hell!)
        self.origin.headers.content_type = get_media_type(text);
    }

    fn set_headers(response_writer: &mut http::server::ResponseWriter) {
        response_writer.headers.date = Some(time::now_utc());

        // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
        response_writer.headers.content_length = None;
        response_writer.headers.content_type = response_writer.headers.content_type
                                                                      .clone()
                                                                      .or(get_media_type("txt"));

        response_writer.headers.server = Some(String::from_str("Nickel"));
    }

    pub fn send_file(&mut self, path: &Path) -> IoResult<()> {
        let mut file = try!(File::open(path));
        self.origin.headers.content_length = None;

        self.origin.headers.content_type = path.extension_str().and_then(get_media_type);
        self.origin.headers.server = Some(String::from_str("Nickel"));
        copy(&mut file, self.origin)
    }

    /// renders a mustache template.
    /// # Example
    /// ```
    /// fn handler (_request: &Request, response: &mut Response) {
    ///     let mut data = HashMap::<&'static str, &'static str>::new();
    ///     data.insert("name", "Simon");
    ///     response.render("examples/assets/template.htm", &data);
    /// }
    /// ```
    pub fn render(&mut self, path: &'static str, data: &HashMap<&'static str, &'static str>)
    {
        let mut templates = self.templates.write();
        let template = templates.find_or_insert_with(path, |_|
                     {
                         mustache::compile_str(match str::from_utf8(
                            match File::open(&Path::new(path)).read_to_end()
                            {
                                Ok(s) => s,
                                Err(e) => fail!("Couldn't open the template file: {}", e)
                            }.as_slice()
                        )
                        {
                            Some(s) => s,
                            None => fail!("Coulnt't read template file as utf8"),
                        }
                    )                        
                }
            );
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
