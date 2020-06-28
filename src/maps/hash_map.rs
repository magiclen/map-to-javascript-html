use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::{self, Write};
use std::string::ToString;

use crate::{functions::*, html_escape, MapToJavaScriptHTML};

impl<K: 'static + Display + Eq + Hash, V: 'static + Display> MapToJavaScriptHTML<K>
    for HashMap<K, V>
{
    fn to_javascript_html_to_vec<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut Vec<u8>,
    ) -> &'a [u8] {
        let variable_name = variable_name.to_string();

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());

        if is_number::<K>() {
            if is_number::<V>() {
                for (key, value) in self {
                    output.extend_from_slice(variable_name.as_bytes());
                    output.write_fmt(format_args!("[{}]={};", key, value)).unwrap();
                }
            } else {
                for (key, value) in self {
                    output.extend_from_slice(variable_name.as_bytes());
                    output.write_fmt(format_args!("[{}]='", key)).unwrap();
                    html_escape::encode_script_single_quoted_text_to_vec(value.to_string(), output);
                    output.extend_from_slice(b"';");
                }
            }
        } else if is_number::<V>() {
            for (key, value) in self {
                output.extend_from_slice(variable_name.as_bytes());
                output.extend_from_slice(b"['");
                html_escape::encode_script_single_quoted_text_to_vec(key.to_string(), output);
                output.extend_from_slice(b"']=");
                output.write_fmt(format_args!("{};", value)).unwrap();
            }
        } else {
            for (key, value) in self {
                output.extend_from_slice(variable_name.as_bytes());
                output.extend_from_slice(b"['");
                html_escape::encode_script_single_quoted_text_to_vec(key.to_string(), output);
                output.extend_from_slice(b"']='");
                html_escape::encode_script_single_quoted_text_to_vec(value.to_string(), output);
                output.extend_from_slice(b"';");
            }
        }

        &output[current_length..]
    }

    fn to_javascript_html_to_writer<S: Display, W: Write>(
        &self,
        variable_name: S,
        output: &mut W,
    ) -> Result<(), io::Error> {
        let variable_name = variable_name.to_string();

        if is_number::<K>() {
            if is_number::<V>() {
                for (key, value) in self {
                    output.write_all(variable_name.as_bytes())?;
                    output.write_fmt(format_args!("[{}]={};", key, value))?;
                }
            } else {
                for (key, value) in self {
                    output.write_all(variable_name.as_bytes())?;
                    output.write_fmt(format_args!("[{}]='", key))?;
                    html_escape::encode_script_single_quoted_text_to_writer(
                        value.to_string(),
                        output,
                    )?;
                    output.write_all(b"';")?;
                }
            }
        } else if is_number::<V>() {
            for (key, value) in self {
                output.write_all(variable_name.as_bytes())?;
                output.write_all(b"['")?;
                html_escape::encode_script_single_quoted_text_to_writer(key.to_string(), output)?;
                output.write_all(b"']=")?;
                output.write_fmt(format_args!("{};", value))?;
            }
        } else {
            for (key, value) in self {
                output.write_all(variable_name.as_bytes())?;
                output.write_all(b"['")?;
                html_escape::encode_script_single_quoted_text_to_writer(key.to_string(), output)?;
                output.write_all(b"']='")?;
                html_escape::encode_script_single_quoted_text_to_writer(value.to_string(), output)?;
                output.write_all(b"';")?;
            }
        }

        Ok(())
    }

    #[inline]
    fn to_javascript_html_with_keys_to_vec<'a, S: Display, KS: ?Sized + Display + Eq + Hash>(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &'a mut Vec<u8>,
    ) -> &'a [u8]
    where
        K: Borrow<KS>, {
        let variable_name = variable_name.to_string();

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());

        if is_number::<K>() {
            if is_number::<V>() {
                for key in keys.iter() {
                    output.extend_from_slice(variable_name.as_bytes());
                    match self.get(key) {
                        Some(value) => {
                            output.write_fmt(format_args!("[{}]={};", key, value)).unwrap();
                        }
                        None => {
                            output.write_fmt(format_args!("[{}]=undefined;", key)).unwrap();
                        }
                    }
                }
            } else {
                for key in keys.iter() {
                    output.extend_from_slice(variable_name.as_bytes());
                    output.write_fmt(format_args!("[{}]=", key)).unwrap();
                    match self.get(key) {
                        Some(value) => {
                            output.push(b'\'');
                            html_escape::encode_script_single_quoted_text_to_vec(
                                value.to_string(),
                                output,
                            );
                            output.extend_from_slice(b"';");
                        }
                        None => {
                            output.extend_from_slice(b"undefined;");
                        }
                    }
                }
            }
        } else if is_number::<V>() {
            for key in keys.iter() {
                output.extend_from_slice(variable_name.as_bytes());
                output.extend_from_slice(b"['");
                html_escape::encode_script_single_quoted_text_to_vec(key.to_string(), output);
                output.extend_from_slice(b"']=");
                match self.get(key) {
                    Some(value) => {
                        output.write_fmt(format_args!("{};", value)).unwrap();
                    }
                    None => {
                        output.extend_from_slice(b"undefined;");
                    }
                }
            }
        } else {
            for key in keys.iter() {
                output.extend_from_slice(variable_name.as_bytes());
                output.extend_from_slice(b"['");
                html_escape::encode_script_single_quoted_text_to_vec(key.to_string(), output);
                output.extend_from_slice(b"']=");
                match self.get(key) {
                    Some(value) => {
                        output.push(b'\'');
                        html_escape::encode_script_single_quoted_text_to_vec(
                            value.to_string(),
                            output,
                        );
                        output.extend_from_slice(b"';");
                    }
                    None => {
                        output.extend_from_slice(b"undefined;");
                    }
                }
            }
        }

        unsafe { output.get_unchecked(current_length..) }
    }

    fn to_javascript_html_with_keys_to_writer<
        S: Display,
        W: Write,
        KS: ?Sized + Display + Eq + Hash,
    >(
        &self,
        variable_name: S,
        keys: &[&KS],
        output: &mut W,
    ) -> Result<(), io::Error>
    where
        K: Borrow<KS>, {
        let variable_name = variable_name.to_string();

        if is_number::<K>() {
            if is_number::<V>() {
                for key in keys.iter() {
                    output.write_all(variable_name.as_bytes())?;
                    match self.get(key) {
                        Some(value) => {
                            output.write_fmt(format_args!("[{}]={};", key, value))?;
                        }
                        None => {
                            output.write_fmt(format_args!("[{}]=undefined;", key))?;
                        }
                    }
                }
            } else {
                for key in keys.iter() {
                    output.write_all(variable_name.as_bytes())?;
                    output.write_fmt(format_args!("[{}]=", key))?;
                    match self.get(key) {
                        Some(value) => {
                            output.write_all(b"'")?;
                            html_escape::encode_script_single_quoted_text_to_writer(
                                value.to_string(),
                                output,
                            )?;
                            output.write_all(b"';")?;
                        }
                        None => {
                            output.write_all(b"undefined;")?;
                        }
                    }
                }
            }
        } else if is_number::<V>() {
            for key in keys.iter() {
                output.write_all(variable_name.as_bytes())?;
                output.write_all(b"['")?;
                html_escape::encode_script_single_quoted_text_to_writer(key.to_string(), output)?;
                output.write_all(b"']=")?;
                match self.get(key) {
                    Some(value) => {
                        output.write_fmt(format_args!("{};", value))?;
                    }
                    None => {
                        output.write_all(b"undefined;")?;
                    }
                }
            }
        } else {
            for key in keys.iter() {
                output.write_all(variable_name.as_bytes())?;
                output.write_all(b"['")?;
                html_escape::encode_script_single_quoted_text_to_writer(key.to_string(), output)?;
                output.write_all(b"']=")?;
                match self.get(key) {
                    Some(value) => {
                        output.write_all(b"'")?;
                        html_escape::encode_script_single_quoted_text_to_writer(
                            value.to_string(),
                            output,
                        )?;
                        output.write_all(b"';")?;
                    }
                    None => {
                        output.write_all(b"undefined;")?;
                    }
                }
            }
        }

        Ok(())
    }
}
