use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::io::{self, Write};
use std::str::from_utf8_unchecked;

/// Serializing a map to JavaScript code in HTML.
pub trait MapToJavaScriptHTML<K> {
    /// Convert this map to minified JavaScript code in HTML. Be careful of the `variable_name` which will not be encoded in HTML.
    #[inline]
    fn to_javascript_html<S: Display>(&self, variable_name: S) -> String {
        let mut s = String::new();

        self.to_javascript_html_to_string(variable_name, &mut s);

        s
    }

    /// Convert this map to minified JavaScript code in HTML. Write it to an existing `String` and return a string slice of the written HTML. Be careful of the `variable_name` which will not be encoded in HTML.
    #[inline]
    fn to_javascript_html_to_string<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut String,
    ) -> &'a str {
        unsafe {
            from_utf8_unchecked(self.to_javascript_html_to_vec(variable_name, output.as_mut_vec()))
        }
    }

    /// Convert this map to minified JavaScript code in HTML. Write it to an existing `Vec<u8>` and return a `u8` slice of the written HTML. Be careful of the `variable_name` which will not be encoded in HTML.
    fn to_javascript_html_to_vec<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut Vec<u8>,
    ) -> &'a [u8];

    /// Convert this map to minified JavaScript code in HTML. Write it to a writer. Be careful of the `variable_name` which will not be encoded in HTML.
    fn to_javascript_html_to_writer<S: Display, W: Write>(
        &self,
        variable_name: S,
        output: &mut W,
    ) -> Result<(), io::Error>;

    /// Convert this map to minified JavaScript code in HTML by given keys. If the key doesn't exist, the output value will be `undefined`. Be careful of the `variable_name` which will not be encoded in HTML.
    #[inline]
    fn to_javascript_html_with_keys<S: Display, KS: ?Sized + Display + Ord + Eq + Hash>(
        &self,
        variable_name: S,
        keys: &[&KS],
    ) -> String
    where
        K: Borrow<KS>, {
        let mut s = String::new();

        self.to_javascript_html_with_keys_to_string(variable_name, keys, &mut s);

        s
    }

    /// Convert this map to minified JavaScript code in HTML by given keys. Write it to an existing `String` and return a string slice of the written HTML. If the key doesn't exist, the output value will be `undefined`. Be careful of the `variable_name` which will not be encoded in HTML.
    fn to_javascript_html_with_keys_to_string<
        'a,
        S: Display,
        KS: ?Sized + Display + Ord + Eq + Hash,
    >(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &'a mut String,
    ) -> &'a str
    where
        K: Borrow<KS>, {
        unsafe {
            from_utf8_unchecked(self.to_javascript_html_with_keys_to_vec(
                variable_name,
                keys,
                output.as_mut_vec(),
            ))
        }
    }

    /// Convert this map to minified JavaScript code in HTML by given keys. Write it to an existing `Vec<u8>` and return a `u8` slice of the written HTML. If the key doesn't exist, the output value will be `undefined`. Be careful of the `variable_name` which will not be encoded in HTML.
    fn to_javascript_html_with_keys_to_vec<'a, S: Display, KS: ?Sized + Display + Ord + Eq + Hash>(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &'a mut Vec<u8>,
    ) -> &'a [u8]
    where
        K: Borrow<KS>;

    /// Convert this map to minified JavaScript code in HTML. Write it to a writer. If the key doesn't exist, the output value will be `undefined`. Be careful of the `variable_name` which will not be encoded in HTML.
    fn to_javascript_html_with_keys_to_writer<
        S: Display,
        W: Write,
        KS: ?Sized + Display + Ord + Eq + Hash,
    >(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &mut W,
    ) -> Result<(), io::Error>
    where
        K: Borrow<KS>;
}
