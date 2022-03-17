/*!
# Map to JavaScript in HTML

This is a library for serializing a map to JavaScript code in HTML, usually for dynamically generating strings on web pages.

## Usage

In your HTML or templates to generate HTML, such as Handlebars, for instance,

```html
<script>
var _text = {};
{{{text}}}
</script>
```

Then, you can use the `MapToJavaScriptHTML` trait to insert your text from a map,

```rust
use std::collections::BTreeMap;

use map_to_javascript_html::MapToJavaScriptHTML;

let mut map = BTreeMap::new();

map.insert("hello", "Hello world!");
map.insert("welcome", "Welcome to my website.");
map.insert("other keys", "Hello world!");

let text = map.to_javascript_html("_text");

assert_eq!("_text['hello']='Hello world!';_text['other keys']='Hello world!';_text['welcome']='Welcome to my website.';", text);
```

After Handlebars replaces **{{{text}}}** with your text, the HTML will be,

```html
<script>
var _text = {};
_text['hello']='Hello world!';_text['other keys']='Hello world!';_text['welcome']='Welcome to my website.';
</script>
```

The key and the value used in a map must implement the `Display` trait.

Methods suffixed with `_to_string`, `_to_vec`, `_to_writer` can be used to generate HTML.

There are also methods prefixed with `to_javascript_html_with_keys` which can be used with keys to filter the output.

```rust
use std::collections::BTreeMap;

use map_to_javascript_html::MapToJavaScriptHTML;

let mut map = BTreeMap::new();

map.insert("hello", "Hello world!");
map.insert("welcome", "Welcome to my website.");
map.insert("other keys", "Hello world!");

let text = map.to_javascript_html_with_keys("_text", &["hello", "welcome"]);

assert_eq!("_text['hello']='Hello world!';_text['welcome']='Welcome to my website.';", text);
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.map-to-javascript-html]
version = "*"
default-features = false
```

## Serde Support

To support the maps from the `serde` framework, enable the `serde` feature.

```toml
[dependencies.map-to-javascript-html]
version = "*"
features = ["serde"]
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod maps;
mod traits;

pub use maps::*;
pub use traits::*;
