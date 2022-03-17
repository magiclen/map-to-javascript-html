#![cfg(feature = "std")]

use std::collections::HashMap;

use map_to_javascript_html::MapToJavaScriptHTML;

#[test]
fn to_javascript_html() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("test-1", "Test 1!");
    assert_eq!("text['test-1']='Test 1!';", map.to_javascript_html("text"));

    let mut map: HashMap<u8, &str> = HashMap::new();
    map.insert(1, "Test 1'!");
    assert_eq!(r"text['1']='Test 1\'!';", map.to_javascript_html("text"));

    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(1, 2);
    assert_eq!("text['1']='2';", map.to_javascript_html("text"));

    let mut map: HashMap<&str, u8> = HashMap::new();
    map.insert("test-1'", 2);
    assert_eq!(r"text['test-1\'']='2';", map.to_javascript_html("text"));
}

#[cfg(feature = "std")]
#[test]
fn to_javascript_html_to_writer() {
    let mut s = String::new();

    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("test-1", "Test 1!");
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!("text['test-1']='Test 1!';", s);

    let mut map: HashMap<u8, &str> = HashMap::new();
    map.insert(1, "Test 1'!");
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!(r"text['1']='Test 1\'!';", s);

    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(1, 2);
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!("text['1']='2';", s);

    let mut map: HashMap<&str, u8> = HashMap::new();
    map.insert("test-1'", 2);
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!(r"text['test-1\'']='2';", s);
}

#[test]
fn to_javascript_html_with_keys() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");

    assert_eq!("text['test-1']='Test 1!';", map.to_javascript_html_with_keys("text", &["test-1"]));
    assert_eq!("text['test-3']=undefined;", map.to_javascript_html_with_keys("text", &["test-3"]));
}

#[cfg(feature = "std")]
#[test]
fn to_javascript_html_with_keys_to_writer() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");

    let mut s = String::new();

    map.to_javascript_html_with_keys_to_writer("text", &["test-1"], unsafe { s.as_mut_vec() })
        .unwrap();
    assert_eq!("text['test-1']='Test 1!';", s);

    s.clear();
    map.to_javascript_html_with_keys_to_writer("text", &["test-3"], unsafe { s.as_mut_vec() })
        .unwrap();
    assert_eq!("text['test-3']=undefined;", s);
}
