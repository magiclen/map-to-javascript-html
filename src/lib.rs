//! # Map to JavaScript in HTML
//! This is a library for serializing a (hash) map to JavaScript code in HTML, usually for dynamically generating strings on web pages.
//!
//! # Example
//!
//! In your HTML or templates to generate HTML, such as Handlebars, for instance,
//!
//! ```html
//! <script>
//! var _text = {};
//! {{{text}}}
//! </script>
//! ```
//!
//! Then, you can use this crate to insert your text into JavaScript code in HTML,
//!
//! ```
//! extern crate map_to_javascript_html;
//!
//! use map_to_javascript_html::hash_map_to_javascript_html;
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//!
//! map.insert("hello", "Hello world!");
//! map.insert("welcome", "Welcome to my website.");
//! map.insert("other keys", "Hello world!");
//!
//! let text = hash_map_to_javascript_html(&map, "_text", &["welcome", "hello"]).unwrap();
//! ```
//!
//! If you want your text to be beautified, you can use `hash_map_to_javascript_html_beautify` function.
//!
//! After Handlebars replaces **{{{text}}}** with your text, the HTML will be,
//!
//! ```html
//! <script>
//! var _text = {};
//! _text['welcome']='Welcome to my website.';_text['hello']='Hello world!';
//! </script>
//! ```
//!
//! The key and the value used in a map must implement the `Display` trait.

extern crate regex;

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Display;

use regex::Regex;

fn find_all(text: &str, search: &str) -> Vec<usize> {
    let mut result = Vec::new();

    let mut start = 0;

    loop {
        //println!("{}", start);
        match text[start..].find(search) {
            Some(index) => {
                result.push(start + index);

                start = start + index + search.len();
            }
            None => break
        }
    }

    result
}

fn escape_html_script_text(text: &str) -> String {
    let index_array = find_all(text, "</script>");

    let mut s = String::new();

    let mut offset = 0;

    for index in index_array {
        s.push_str(&text[offset..(index + 1)]);
        s.push('\\');
        offset = index + 1;
    }

    s.push_str(&text[offset..]);

    s
}

fn escape_quote(text: &str) -> String {
    let regex = Regex::new(r"([^\\]')|(^')").unwrap();

    let mut s = String::new();

    let mut offset = 0;

    for m in regex.find_iter(text) {
        let start = m.start();
        let end = m.end();

        if start == 0 {
            s.push_str(r"\'");
            offset = 1;
        } else {
            s.push_str(&text[offset..end - 1]);
            s.push('\\');

            offset = end - 1;
        }
    }

    s.push_str(&text[offset..]);

    s
}

fn replace_new_line(text: &str) -> String {
    let regex = Regex::new(r"\n").unwrap();

    regex.replace_all(text, r"\n").to_string()
}

/// Convert a HashMap to minified JavaScript code in HTML.
pub fn hash_map_to_javascript_html<K: Display + Eq + Hash, V: Display>(hash_map: &HashMap<K, V>, variable_name: &str, keys: &[K]) -> Result<String, String> {
    let mut s = String::new();

    for key in keys.iter() {
        let k = key.to_string();

        let v = match hash_map.get(key) {
            Some(s) => s.to_string(),
            None => return Err(format!("`{}` is not found.", k))
        };

        s.push_str(&format!("{}['{}']='{}';", variable_name, escape_html_script_text(&replace_new_line(&escape_quote(&k))), escape_html_script_text(&replace_new_line(&escape_quote(&v)))));
    }

    Ok(s)
}

/// Convert a HashMap to beautified JavaScript code in HTML.
pub fn hash_map_to_javascript_html_beautify<K: Display + Eq + Hash, V: Display>(hash_map: &HashMap<K, V>, variable_name: &str, keys: &[K], spaces_a_tab: u8, tab_count: u8) -> Result<String, String> {
    let mut s = String::new();

    let mut indices = Vec::new();

    for key in keys.iter() {
        let k = key.to_string();

        let v = match hash_map.get(key) {
            Some(s) => s.to_string(),
            None => return Err(format!("`{}` is not found.", k))
        };

        indices.push(s.len());

        s.push_str(&format!("{}['{}'] = '{}';", variable_name, escape_html_script_text(&replace_new_line(&escape_quote(&k))), escape_html_script_text(&replace_new_line(&escape_quote(&v)))));
    }

    let len = indices.len();

    if len > 0 {
        let n;
        let tab;

        if spaces_a_tab > 0 {
            n = spaces_a_tab as usize * tab_count as usize;

            tab = ' ';
        } else {
            n = tab_count as usize;

            tab = '\t';
        }

        let tab_n = {
            let mut s = String::with_capacity(n);

            for _ in 0..n {
                s.push(tab);
            }

            s
        };

        s.reserve(n + (n + 1) * (len - 1));

        for &i in indices[1..].iter().rev() {
            s.insert(i, '\n');
            s.insert_str(i + 1, &tab_n);
        }

        s.insert_str(0, &tab_n);
    }

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_hash_map_to_javascript_html() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("test-1", "Test 1!");
        map.insert("test-2", "Test 2!");
        map.insert("test-'3'", "Test '3'!");
        map.insert(r"test-\'4\'", r"Test \'4\'!");
        map.insert("script", "<script>alert('Hello world!');</script>");
        map.insert(r"'中'文", "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>");

        let html = hash_map_to_javascript_html(&map, "text", &vec!["test-1", "test-2", "test-'3'", r"test-\'4\'", "script", r"'中'文"]).unwrap();

        assert_eq!(r#"text['test-1']='Test 1!';text['test-2']='Test 2!';text['test-\'3\'']='Test \'3\'!';text['test-\'4\'']='Test \'4\'!';text['script']='<script>alert(\'Hello world!\');<\/script>';text['\'中\'文']='<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';"#, html);
    }

    #[test]
    fn text_hash_map_to_javascript_beautify_1() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("test-1", "Test 1!");
        map.insert("test-2", "Test 2!");
        map.insert("test-'3'", "Test '3'!");
        map.insert(r"test-\'4\'", r"Test \'4\'!");
        map.insert("script", "<script>alert('Hello world!');</script>");
        map.insert(r"'中'文", "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>");

        let html = hash_map_to_javascript_html_beautify(&map, "text", &vec!["test-1", "test-2", "test-'3'", r"test-\'4\'", "script", r"'中'文"], 0, 0).unwrap();

        assert_eq!(r#"text['test-1'] = 'Test 1!';
text['test-2'] = 'Test 2!';
text['test-\'3\''] = 'Test \'3\'!';
text['test-\'4\''] = 'Test \'4\'!';
text['script'] = '<script>alert(\'Hello world!\');<\/script>';
text['\'中\'文'] = '<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';"#, html);
    }

    #[test]
    fn text_hash_map_to_javascript_beautify_2() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        map.insert("test-1", "Test 1!");
        map.insert("test-2", "Test 2!");
        map.insert("test-'3'", "Test '3'!");
        map.insert(r"test-\'4\'", r"Test \'4\'!");
        map.insert("script", "<script>alert('Hello world!');</script>");
        map.insert(r"'中'文", "<script>alert('Hello world!');</script><script>alert('哈囉，世界！');</script>");

        let html = hash_map_to_javascript_html_beautify(&map, "text", &vec!["test-1", "test-2", "test-'3'", r"test-\'4\'", "script", r"'中'文"], 12, 1).unwrap();

        assert_eq!(r#"            text['test-1'] = 'Test 1!';
            text['test-2'] = 'Test 2!';
            text['test-\'3\''] = 'Test \'3\'!';
            text['test-\'4\''] = 'Test \'4\'!';
            text['script'] = '<script>alert(\'Hello world!\');<\/script>';
            text['\'中\'文'] = '<script>alert(\'Hello world!\');<\/script><script>alert(\'哈囉，世界！\');<\/script>';"#, html);
    }
}