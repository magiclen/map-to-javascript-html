Map to JavaScript in HTML
====================

[![Build Status](https://travis-ci.org/magiclen/map-to-javascript-html.svg?branch=master)](https://travis-ci.org/magiclen/map-to-javascript-html)
[![Build status](https://ci.appveyor.com/api/projects/status/a0t05l2qxbqp9902/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/map-to-javascript-html/branch/master)

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

## Crates.io

https://crates.io/crates/map-to-javascript-html

## Documentation

https://docs.rs/map-to-javascript-html

## License

[MIT](LICENSE)