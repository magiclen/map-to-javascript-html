use core::borrow::Borrow;
use core::fmt::Display;
use core::hash::Hash;

use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::io::{self, Write};

use crate::{html_escape, MapToJavaScriptHTML};

impl<K: Display + Eq + Hash, V: Display> MapToJavaScriptHTML<K> for HashMap<K, V> {
    fn to_javascript_html_to_vec<'a, S: Display>(
        &self,
        variable_name: S,
        output: &'a mut Vec<u8>,
    ) -> &'a [u8] {
        let variable_name = format!("{}", variable_name);

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());

        for (key, value) in self {
            output.extend_from_slice(variable_name.as_bytes());
            output.extend_from_slice(b"['");
            html_escape::encode_script_single_quoted_text_to_vec(format!("{}", key), output);
            output.extend_from_slice(b"']='");
            html_escape::encode_script_single_quoted_text_to_vec(format!("{}", value), output);
            output.extend_from_slice(b"';");
        }

        &output[current_length..]
    }

    #[cfg(feature = "std")]
    fn to_javascript_html_to_writer<S: Display, W: Write>(
        &self,
        variable_name: S,
        output: &mut W,
    ) -> Result<(), io::Error> {
        let variable_name = format!("{}", variable_name);

        for (key, value) in self {
            output.write_all(variable_name.as_bytes())?;
            output.write_all(b"['")?;
            html_escape::encode_script_single_quoted_text_to_writer(format!("{}", key), output)?;
            output.write_all(b"']='")?;
            html_escape::encode_script_single_quoted_text_to_writer(format!("{}", value), output)?;
            output.write_all(b"';")?;
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
        let variable_name = format!("{}", variable_name);

        let current_length = output.len();

        output.reserve((variable_name.len() + 11) * self.len());

        for key in keys.iter() {
            output.extend_from_slice(variable_name.as_bytes());
            output.extend_from_slice(b"['");
            html_escape::encode_script_single_quoted_text_to_vec(format!("{}", key), output);
            output.extend_from_slice(b"']=");
            match self.get(key) {
                Some(value) => {
                    output.push(b'\'');
                    html_escape::encode_script_single_quoted_text_to_vec(
                        format!("{}", value),
                        output,
                    );
                    output.extend_from_slice(b"';");
                }
                None => {
                    output.extend_from_slice(b"undefined;");
                }
            }
        }

        unsafe { output.get_unchecked(current_length..) }
    }

    #[cfg(feature = "std")]
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
        let variable_name = format!("{}", variable_name);

        for key in keys.iter() {
            output.write_all(variable_name.as_bytes())?;
            output.write_all(b"['")?;
            html_escape::encode_script_single_quoted_text_to_writer(format!("{}", key), output)?;
            output.write_all(b"']=")?;
            match self.get(key) {
                Some(value) => {
                    output.write_all(b"'")?;
                    html_escape::encode_script_single_quoted_text_to_writer(
                        format!("{}", value),
                        output,
                    )?;
                    output.write_all(b"';")?;
                }
                None => {
                    output.write_all(b"undefined;")?;
                }
            }
        }

        Ok(())
    }
}
