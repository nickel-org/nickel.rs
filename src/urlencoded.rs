use std::collections::hashmap::HashMap;
use url::form_urlencoded;

pub fn parse (encoded_string : &str) -> HashMap<String, Vec<String>> {
    create_hash_map(form_urlencoded::parse_str(encoded_string))
}

fn create_hash_map(parsed : Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in parsed.into_iter() {
        map.find_with_or_insert_with(key, value,
            |_, existing, value| { existing.push(value); },
            |_, value| vec![value]
        );
    }

    map
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
