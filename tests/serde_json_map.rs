#![cfg(feature = "serde_json")]

use serde_json::{Map, Value};

use map_to_javascript_html::MapToJavaScriptHTML;

#[test]
fn to_javascript_html() {
    let mut map = Map::new();
    map.insert("test-1".to_string(), Value::from("Test 1!"));
    assert_eq!("text['test-1']='Test 1!';", map.to_javascript_html("text"));

    let mut map = Map::new();
    map.insert("test-1'".to_string(), Value::from(1));
    assert_eq!(r"text['test-1\'']=1;", map.to_javascript_html("text"));

    let mut map = Map::new();
    map.insert("test-1".to_string(), serde_json::to_value([1, 2, 3, 4, 5]).unwrap());
    assert_eq!("text['test-1']=[1,2,3,4,5];", map.to_javascript_html("text"));
}

#[cfg(feature = "std")]
#[test]
fn to_javascript_html_to_writer() {
    let mut s = String::new();

    let mut map = Map::new();
    map.insert("test-1".to_string(), Value::from("Test 1!"));
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!("text['test-1']='Test 1!';", s);

    map.clear();
    map.insert("test-1'".to_string(), Value::from(1));
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!(r"text['test-1\'']=1;", s);

    map.clear();
    map.insert("test-1".to_string(), serde_json::to_value([1, 2, 3, 4, 5]).unwrap());
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!("text['test-1']=[1,2,3,4,5];", s);
}

#[test]
fn to_javascript_html_with_keys() {
    let mut map = Map::new();
    map.insert(String::from("test-1"), Value::from("Test 1!"));
    map.insert(String::from("test-2"), Value::from("Test 2!"));

    assert_eq!("text['test-1']='Test 1!';", map.to_javascript_html_with_keys("text", &["test-1"]));
    assert_eq!("text['test-3']=undefined;", map.to_javascript_html_with_keys("text", &["test-3"]));
}

#[cfg(feature = "std")]
#[test]
fn to_javascript_html_with_keys_to_writer() {
    let mut map = Map::new();
    map.insert(String::from("test-1"), Value::from("Test 1!"));
    map.insert(String::from("test-2"), Value::from("Test 2!"));

    let mut s = String::new();

    map.to_javascript_html_with_keys_to_writer("text", &["test-1"], unsafe { s.as_mut_vec() })
        .unwrap();
    assert_eq!("text['test-1']='Test 1!';", s);

    s.clear();
    map.to_javascript_html_with_keys_to_writer("text", &["test-3"], unsafe { s.as_mut_vec() })
        .unwrap();
    assert_eq!("text['test-3']=undefined;", s);
}
