extern crate map_to_javascript_html;

use std::collections::BTreeMap;

use map_to_javascript_html::MapToJavaScriptHTML;

#[test]
fn to_javascript_html() {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    map.insert("test-1", "Test 1!");
    assert_eq!("text['test-1']='Test 1!';", map.to_javascript_html("text"));

    let mut map: BTreeMap<u8, &str> = BTreeMap::new();
    map.insert(1, "Test 1'!");
    assert_eq!(r"text[1]='Test 1\'!';", map.to_javascript_html("text"));

    let mut map: BTreeMap<u8, u8> = BTreeMap::new();
    map.insert(1, 2);
    assert_eq!("text[1]=2;", map.to_javascript_html("text"));

    let mut map: BTreeMap<&str, u8> = BTreeMap::new();
    map.insert("test-1'", 2);
    assert_eq!(r"text['test-1\'']=2;", map.to_javascript_html("text"));
}

#[test]
fn to_javascript_html_to_writer() {
    let mut s = String::new();

    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    map.insert("test-1", "Test 1!");
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!("text['test-1']='Test 1!';", s);

    let mut map: BTreeMap<u8, &str> = BTreeMap::new();
    map.insert(1, "Test 1'!");
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!(r"text[1]='Test 1\'!';", s);

    let mut map: BTreeMap<u8, u8> = BTreeMap::new();
    map.insert(1, 2);
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!("text[1]=2;", s);

    let mut map: BTreeMap<&str, u8> = BTreeMap::new();
    map.insert("test-1'", 2);
    s.clear();
    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();
    assert_eq!(r"text['test-1\'']=2;", s);
}

#[test]
fn to_javascript_html_complex() {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();

    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");
    map.insert("test-'3'", "Test '3'!");
    map.insert(r"test-\'4\'", r"Test \'4\'!");
    map.insert("script", "<script>alert('Hello world!');</script>");
    map.insert(
        r"'中'文",
        "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>",
    );

    let expect = r"text['\'中\'文']='<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';text['script']='<script>alert(\'Hello world!\');<\/script>';text['test-\'3\'']='Test \'3\'!';text['test-1']='Test 1!';text['test-2']='Test 2!';text['test-\'4\'']='Test \'4\'!';";

    assert_eq!(expect, map.to_javascript_html("text"));
}

#[test]
fn to_javascript_html_to_writer_complex() {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();

    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");
    map.insert("test-'3'", "Test '3'!");
    map.insert(r"test-\'4\'", r"Test \'4\'!");
    map.insert("script", "<script>alert('Hello world!');</script>");
    map.insert(
        r"'中'文",
        "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>",
    );

    let expect = r"text['\'中\'文']='<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';text['script']='<script>alert(\'Hello world!\');<\/script>';text['test-\'3\'']='Test \'3\'!';text['test-1']='Test 1!';text['test-2']='Test 2!';text['test-\'4\'']='Test \'4\'!';";

    let mut s = String::new();

    map.to_javascript_html_to_writer("text", unsafe { s.as_mut_vec() }).unwrap();

    assert_eq!(expect, s);
}

#[test]
fn to_javascript_html_with_keys() {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");

    assert_eq!("text['test-1']='Test 1!';", map.to_javascript_html_with_keys("text", &["test-1"]));
    assert_eq!("text['test-3']=undefined;", map.to_javascript_html_with_keys("text", &["test-3"]));
}

#[test]
fn to_javascript_html_with_keys_to_writer() {
    let mut map: BTreeMap<&str, &str> = BTreeMap::new();
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
