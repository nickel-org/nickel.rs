use std::collections::hashmap::HashMap;
use url::form_urlencoded;
use groupable::Groupable;

pub fn parse (encoded_string : &str) -> HashMap<String, Vec<String>> {
    form_urlencoded::parse_str(encoded_string).into_iter().group()
}

#[test]
fn parses_encoded_string_with_duplicate_keys() {
    let map = parse(
      "foo=bar&message=hello&message=world"
    );
    assert!(map.get(&"foo".to_string()).len() == 1);
    assert!(map.get(&"foo".to_string()).contains(&"bar".to_string()));
    assert!(map.get(&"message".to_string()).len() == 2);
    assert!(map.get(&"message".to_string()).contains(&"hello".to_string()));
    assert!(map.get(&"message".to_string()).contains(&"world".to_string()));
}

#[test]
fn parses_urlencoded_characters() {
    let map = parse(
        "message=hello%20world"
    );
    assert!(map.get(&"message".to_string()).len() == 1);
    assert!(map.get(&"message".to_string()).contains(&"hello world".to_string()));
}
