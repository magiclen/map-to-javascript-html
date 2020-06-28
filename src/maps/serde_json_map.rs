extern crate serde_json;

use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::io::{self, Write};
use std::string::ToString;

use crate::{html_escape, MapToJavaScriptHTML};

use serde_json::{Map, Value};

#[inline]
fn value_to_javascript_value_end_with_semicolon_in_html_to_vec(
    value: &Value,
    output: &mut Vec<u8>,
) {
    match value {
        Value::Null => output.extend_from_slice(b"null;"),
        Value::String(s) => {
            output.push(b'\'');
            html_escape::encode_script_single_quoted_text_to_vec(s, output);
            output.extend_from_slice(b"\';");
        }
        Value::Bool(b) => {
            output.write_fmt(format_args!("{};", b)).unwrap();
        }
        Value::Number(n) => {
            output.write_fmt(format_args!("{};", n)).unwrap();
        }
        Value::Object(_) | Value::Array(_) => {
            let json = value.to_string();

            html_escape::encode_script_to_vec(json, output);
            output.push(b';');
        }
    }
}

#[inline]
fn value_to_javascript_value_end_with_semicolon_in_html_to_writer<W: Write>(
    value: &Value,
    output: &mut W,
) -> Result<(), io::Error> {
    match value {
        Value::Null => output.write_all(b"null;"),
        Value::String(s) => {
            output.write_all(b"'")?;
            html_escape::encode_script_single_quoted_text_to_writer(s, output)?;
            output.write_all(b"\';")
        }
        Value::Bool(b) => output.write_fmt(format_args!("{};", b)),
        Value::Number(n) => output.write_fmt(format_args!("{};", n)),
        Value::Object(_) | Value::Array(_) => {
            let json = value.to_string();

            html_escape::encode_script_to_writer(json, output)?;
            output.write_all(b";")
        }
    }
}

impl MapToJavaScriptHTML<String> for Map<String, Value> {
    fn to_javascript_html_to_vec<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut Vec<u8>,
    ) -> &'a [u8] {
        let variable_name = variable_name.to_string();

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());

        for (key, value) in self {
            output.extend_from_slice(variable_name.as_bytes());
            output.extend_from_slice(b"['");
            html_escape::encode_script_single_quoted_text_to_vec(key, output);
            output.extend_from_slice(b"']=");
            value_to_javascript_value_end_with_semicolon_in_html_to_vec(value, output);
        }

        &output[current_length..]
    }

    fn to_javascript_html_to_writer<S: Display, W: Write>(
        &self,
        variable_name: S,
        output: &mut W,
    ) -> Result<(), io::Error> {
        let variable_name = variable_name.to_string();

        for (key, value) in self {
            output.write_all(variable_name.as_bytes())?;
            output.write_all(b"['")?;
            html_escape::encode_script_single_quoted_text_to_writer(key, output)?;
            output.write_all(b"']=")?;
            value_to_javascript_value_end_with_semicolon_in_html_to_writer(value, output)?;
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
        let variable_name = variable_name.to_string();

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());

        for key in keys.iter() {
            output.extend_from_slice(variable_name.as_bytes());
            output.extend_from_slice(b"['");
            html_escape::encode_script_single_quoted_text_to_vec(key.to_string(), output);
            output.extend_from_slice(b"']=");
            match self.get(key) {
                Some(value) => {
                    value_to_javascript_value_end_with_semicolon_in_html_to_vec(value, output);
                }
                None => {
                    output.extend_from_slice(b"undefined;");
                }
            }
        }

        &output[current_length..]
    }

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
        let variable_name = variable_name.to_string();

        for key in keys.iter() {
            output.write_all(variable_name.as_bytes())?;
            output.write_all(b"['")?;
            html_escape::encode_script_single_quoted_text_to_writer(key.to_string(), output)?;
            output.write_all(b"']=")?;
            match self.get(key) {
                Some(value) => {
                    value_to_javascript_value_end_with_semicolon_in_html_to_writer(value, output)?;
                }
                None => {
                    output.write_all(b"undefined;")?;
                }
            }
        }

        Ok(())
    }
}
