extern crate map_to_javascript_html;

use std::collections::HashMap;

#[test]
fn hash_map_to_javascript_html() {
    let mut map: HashMap<&str, &str> = HashMap::new();

    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");
    map.insert("test-'3'", "Test '3'!");
    map.insert(r"test-\'4\'", r"Test \'4\'!");
    map.insert("script", "<script>alert('Hello world!');</script>");
    map.insert(
        r"'中'文",
        "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>",
    );

    let html = map_to_javascript_html::hash_map_to_javascript_html(&map, "text", &[
        "test-1",
        "test-2",
        "test-'3'",
        r"test-\'4\'",
        "script",
        r"'中'文",
    ])
    .unwrap();

    assert_eq!(r#"text['test-1']='Test 1!';text['test-2']='Test 2!';text['test-\'3\'']='Test \'3\'!';text['test-\'4\'']='Test \'4\'!';text['script']='<script>alert(\'Hello world!\');<\/script>';text['\'中\'文']='<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';"#, html);
}

#[test]
fn hash_map_to_javascript_beautify_1() {
    let mut map: HashMap<&str, &str> = HashMap::new();

    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");
    map.insert("test-'3'", "Test '3'!");
    map.insert(r"test-\'4\'", r"Test \'4\'!");
    map.insert("script", "<script>alert('Hello world!');</script>");
    map.insert(
        r"'中'文",
        "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>",
    );

    let html = map_to_javascript_html::hash_map_to_javascript_html_beautify(
        &map,
        "text",
        &["test-1", "test-2", "test-'3'", r"test-\'4\'", "script", r"'中'文"],
        0,
        0,
    )
    .unwrap();

    assert_eq!(r#"text['test-1'] = 'Test 1!';
text['test-2'] = 'Test 2!';
text['test-\'3\''] = 'Test \'3\'!';
text['test-\'4\''] = 'Test \'4\'!';
text['script'] = '<script>alert(\'Hello world!\');<\/script>';
text['\'中\'文'] = '<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';"#, html);
}

#[test]
fn hash_map_to_javascript_beautify_2() {
    let mut map: HashMap<&str, &str> = HashMap::new();

    map.insert("test-1", "Test 1!");
    map.insert("test-2", "Test 2!");
    map.insert("test-'3'", "Test '3'!");
    map.insert(r"test-\'4\'", r"Test \'4\'!");
    map.insert("script", "<script>alert('Hello world!');</script>");
    map.insert(
        r"'中'文",
        "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>",
    );

    let html = map_to_javascript_html::hash_map_to_javascript_html_beautify(
        &map,
        "text",
        &["test-1", "test-2", "test-'3'", r"test-\'4\'", "script", r"'中'文"],
        12,
        1,
    )
    .unwrap();

    assert_eq!(r#"            text['test-1'] = 'Test 1!';
            text['test-2'] = 'Test 2!';
            text['test-\'3\''] = 'Test \'3\'!';
            text['test-\'4\''] = 'Test \'4\'!';
            text['script'] = '<script>alert(\'Hello world!\');<\/script>';
            text['\'中\'文'] = '<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';"#, html);
}
