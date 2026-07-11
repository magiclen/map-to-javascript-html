use alloc::{string::String, vec::Vec};
use core::{borrow::Borrow, fmt::Display, hash::Hash, str::from_utf8_unchecked};
#[cfg(feature = "std")]
use std::io::{self, Write};

use serde_json::{Map, Value};

#[cfg(feature = "std")]
use crate::encode::{encode_display_to_writer, encode_serializable_to_writer};
use crate::{
    encode::{encode_display_to_vec, encode_serializable_to_vec},
    MapToJavaScriptHTML,
};

#[inline]
fn value_to_javascript_value_end_with_semicolon_in_html_to_vec(
    value: &Value,
    scratch: &mut Vec<u8>,
    output: &mut Vec<u8>,
) {
    encode_serializable_to_vec(value, scratch, output);
    output.push(b';');
}

#[cfg(feature = "std")]
#[inline]
fn value_to_javascript_value_end_with_semicolon_in_html_to_writer<W: Write>(
    value: &Value,
    scratch: &mut Vec<u8>,
    output: &mut W,
) -> Result<(), io::Error> {
    encode_serializable_to_writer(value, scratch, output)?;
    output.write_all(b";")
}

impl MapToJavaScriptHTML<String> for Map<String, Value> {
    fn to_javascript_html_to_vec<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut Vec<u8>,
    ) -> &'a [u8] {
        let variable_name = format!("{}", variable_name);

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());
        let mut scratch = Vec::new();

        for (key, value) in self {
            output.extend_from_slice(variable_name.as_bytes());
            output.push(b'[');
            encode_display_to_vec(key, &mut scratch, output);
            output.extend_from_slice(b"]=");
            value_to_javascript_value_end_with_semicolon_in_html_to_vec(
                value,
                &mut scratch,
                output,
            );
        }

        &output[current_length..]
    }

    #[inline]
    fn to_javascript_html_to_string<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut String,
    ) -> &'a str {
        // SAFETY: This implementation only appends UTF-8 produced by formatting, serde_json, and ASCII constants.
        unsafe {
            from_utf8_unchecked(self.to_javascript_html_to_vec(variable_name, output.as_mut_vec()))
        }
    }

    #[cfg(feature = "std")]
    fn to_javascript_html_to_writer<S: Display, W: Write>(
        &self,
        variable_name: S,
        output: &mut W,
    ) -> Result<(), io::Error> {
        let variable_name = format!("{}", variable_name);
        let mut scratch = Vec::new();

        for (key, value) in self {
            output.write_all(variable_name.as_bytes())?;
            output.write_all(b"[")?;
            encode_display_to_writer(key, &mut scratch, output)?;
            output.write_all(b"]=")?;
            value_to_javascript_value_end_with_semicolon_in_html_to_writer(
                value,
                &mut scratch,
                output,
            )?;
        }

        Ok(())
    }

    #[inline]
    fn to_javascript_html_with_keys_to_vec<'a, S: Display, KS: ?Sized + Display + Ord + Hash>(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &'a mut Vec<u8>,
    ) -> &'a [u8]
    where
        String: Borrow<KS>, {
        let variable_name = format!("{}", variable_name);

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * keys.len());
        let mut scratch = Vec::new();

        for key in keys {
            output.extend_from_slice(variable_name.as_bytes());
            output.push(b'[');
            encode_display_to_vec(*key, &mut scratch, output);
            output.extend_from_slice(b"]=");
            match self.get(key) {
                Some(value) => {
                    value_to_javascript_value_end_with_semicolon_in_html_to_vec(
                        value,
                        &mut scratch,
                        output,
                    );
                },
                None => {
                    output.extend_from_slice(b"undefined;");
                },
            }
        }

        &output[current_length..]
    }

    #[inline]
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
        String: Borrow<KS>, {
        // SAFETY: This implementation only appends UTF-8 produced by formatting, serde_json, and ASCII constants.
        unsafe {
            from_utf8_unchecked(self.to_javascript_html_with_keys_to_vec(
                variable_name,
                keys,
                output.as_mut_vec(),
            ))
        }
    }

    #[cfg(feature = "std")]
    fn to_javascript_html_with_keys_to_writer<
        S: Display,
        W: Write,
        KS: ?Sized + Display + Ord + Hash,
    >(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &mut W,
    ) -> Result<(), io::Error>
    where
        String: Borrow<KS>, {
        let variable_name = format!("{}", variable_name);
        let mut scratch = Vec::new();

        for key in keys {
            output.write_all(variable_name.as_bytes())?;
            output.write_all(b"[")?;
            encode_display_to_writer(*key, &mut scratch, output)?;
            output.write_all(b"]=")?;
            match self.get(key) {
                Some(value) => {
                    value_to_javascript_value_end_with_semicolon_in_html_to_writer(
                        value,
                        &mut scratch,
                        output,
                    )?;
                },
                None => {
                    output.write_all(b"undefined;")?;
                },
            }
        }

        Ok(())
    }
}
