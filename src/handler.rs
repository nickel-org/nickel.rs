extern crate mustache;

use std::str;
use request::Request;
use response::Response;
use std::collections::HashMap;
use std::io::File;

pub struct Handler
{
    template: mustache::Template,
    exec: Option<fn(request: &Request, map: &mut HashMap<String, String>)>
}

impl Clone for Handler
{
    fn clone(&self) -> Handler 
    {
        Handler{ template: self.template.clone(), exec: self.exec }
    }
}

impl Handler
{
    pub fn new(template_str: &str, exec: Option<fn(request: &Request, &mut HashMap<String, String>)>) -> Handler
    {
        Handler{template: mustache::compile_str(template_str), exec: exec}
    }

    pub fn new_from_file(path_str: &str, exec: Option<fn(request: &Request, &mut HashMap<String, String>)>) -> Handler
    {
        Handler{template: mustache::compile_str( match str::from_utf8(
                                                    match File::open(&Path::new(path_str)).read_to_end()
                                                    {
                                                         Ok(s) => s,
                                                         Err(e) => fail!("Couldn't open the template file: {}", e)
                                                    }.as_slice()
                                                )
                                                {
                                                    Some(s) => s,
                                                    None => fail!("Coulnt't read template file as utf8"),
                                                }
                                               ), exec: exec}
    }
}

impl Handler
{
    pub fn handle(&self, request: &Request, response: &mut Response)
    {
        let mut map = HashMap::new();
        match self.exec
        {
            Some(exec) => exec(request, &mut map),
            None => ()
        }
        let _ = self.template.render(response.origin, &map);
    }
}
