use std::collections::HashMap;
use url::form_urlencoded;
use groupable::Groupable;

pub fn parse (encoded_string : &str) -> HashMap<String, Vec<String>> {
    form_urlencoded::parse(encoded_string.as_bytes()).into_iter().group()
}

#[test]
fn parses_encoded_string_with_duplicate_keys() {
    let map = parse("foo=bar&message=hello&message=world");
    assert_eq!(map["foo"],
                vec!["bar".to_string()]);
    // Ensure the ordering is correct
    assert_eq!(map["message"],
                vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn parses_urlencoded_characters() {
    let map = parse("message=hello%20world");

    assert_eq!(map["message"],
                vec!["hello world".to_string()]);
}
