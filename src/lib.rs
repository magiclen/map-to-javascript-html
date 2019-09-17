/*!
# Map to JavaScript in HTML

This is a library for serializing a (hash) map to JavaScript code in HTML, usually for dynamically generating strings on web pages.

# Example

In your HTML or templates to generate HTML, such as Handlebars, for instance,

```html
<script>
var _text = {};
{{{text}}}
</script>
```

Then, you can use this crate to insert your text into JavaScript code in HTML,

```rust
extern crate map_to_javascript_html;

use map_to_javascript_html::hash_map_to_javascript_html;
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert("hello", "Hello world!");
map.insert("welcome", "Welcome to my website.");
map.insert("other keys", "Hello world!");

let text = hash_map_to_javascript_html(&map, "_text", &["welcome", "hello"]).unwrap();
```

If you want your text to be beautified, you can use `hash_map_to_javascript_html_beautify` function.

After Handlebars replaces **{{{text}}}** with your text, the HTML will be,

```html
<script>
var _text = {};
_text['welcome']='Welcome to my website.';_text['hello']='Hello world!';
</script>
```

The key and the value used in a map must implement the `Display` trait.

By the way, the `text_to_javascript_html` function can be useful when you just want to insert text as a JavaScript string into your HTML code.
*/

extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::hash::{BuildHasher, Hash};

use regex::Regex;

lazy_static! {
    static ref ESCAPE_QUOTE_RE: Regex = { Regex::new(r"([^\\]')|(^')").unwrap() };
}

fn find_all(text: &str, search: &str) -> Vec<usize> {
    let mut result = Vec::new();

    let mut start = 0;

    while let Some(index) = text[start..].find(search) {
        result.push(start + index);

        start = start + index + search.len();
    }

    result
}

fn escape_html_script_text(text: &str) -> String {
    let index_array = find_all(text, "</script>");

    let mut s = String::new();

    let mut offset = 0;

    for index in index_array {
        s.push_str(&text[offset..=index]);
        s.push('\\');
        offset = index + 1;
    }

    s.push_str(&text[offset..]);

    s
}

fn escape_quote(text: &str) -> String {
    let mut s = String::new();

    let mut offset = 0;

    for m in ESCAPE_QUOTE_RE.find_iter(text) {
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
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        if c == '\n' {
            continue;
        }

        result.push(c);
    }

    result
}

/// Convert text to a JavaScript string with single quotes in HTML.
pub fn text_to_javascript_html<S: AsRef<str>>(text: S) -> String {
    let mut string = escape_html_script_text(&replace_new_line(&escape_quote(text.as_ref())));
    string.insert(0, '\'');
    string.push('\'');
    string
}

/// Convert a HashMap to minified JavaScript code in HTML.
pub fn hash_map_to_javascript_html<
    K: Display + Eq + Hash,
    V: Display,
    S: AsRef<str>,
    B: BuildHasher,
>(
    hash_map: &HashMap<K, V, B>,
    variable_name: S,
    keys: &[K],
) -> Result<String, String> {
    let mut s = String::new();

    for key in keys.iter() {
        let k = key.to_string();

        let v = match hash_map.get(key) {
            Some(s) => s.to_string(),
            None => return Err(format!("`{}` is not found.", k)),
        };

        s.write_fmt(format_args!(
            "{}[{}]={};",
            variable_name.as_ref(),
            text_to_javascript_html(&k),
            text_to_javascript_html(&v)
        ))
        .unwrap();
    }

    Ok(s)
}

/// Convert a HashMap to beautified JavaScript code in HTML.
pub fn hash_map_to_javascript_html_beautify<
    K: Display + Eq + Hash,
    V: Display,
    S: AsRef<str>,
    B: BuildHasher,
>(
    hash_map: &HashMap<K, V, B>,
    variable_name: S,
    keys: &[K],
    spaces_a_tab: u8,
    tab_count: u8,
) -> Result<String, String> {
    let mut s = String::new();

    let mut indices = Vec::new();

    for key in keys.iter() {
        let k = key.to_string();

        let v = match hash_map.get(key) {
            Some(s) => s.to_string(),
            None => return Err(format!("`{}` is not found.", k)),
        };

        indices.push(s.len());

        s.write_fmt(format_args!(
            "{}[{}] = {};",
            variable_name.as_ref(),
            text_to_javascript_html(&k),
            text_to_javascript_html(&v)
        ))
        .unwrap();
    }

    let len = indices.len();

    if len > 0 {
        let n;

        let tab = if spaces_a_tab > 0 {
            n = spaces_a_tab as usize * tab_count as usize;

            ' '
        } else {
            n = tab_count as usize;

            '\t'
        };

        let tab_n = {
            let mut s = String::with_capacity(n);

            for _ in 0..n {
                s.push(tab);
            }

            s
        };

        s.reserve(n + (n + 1) * (len - 1));

        for &i in indices.iter().skip(1).rev() {
            s.insert(i, '\n');
            s.insert_str(i + 1, &tab_n);
        }

        s.insert_str(0, &tab_n);
    }

    Ok(s)
}
