use alloc::vec::Vec;
use core::{fmt::Display, str::from_utf8_unchecked};
#[cfg(feature = "std")]
use std::io::{self, Write};

use serde::{Serialize, Serializer};

struct DisplayAsString<'a, T: ?Sized>(&'a T);

impl<T: ?Sized + Display> Serialize for DisplayAsString<'_, T> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self.0)
    }
}

#[inline]
#[cfg(feature = "std")]
fn serialize_to_scratch<T: ?Sized + Serialize>(value: &T, scratch: &mut Vec<u8>) {
    scratch.clear();
    value
        .serialize(&mut serde_json::Serializer::new(&mut *scratch))
        .expect("serializing to a byte vector should not fail");
}

#[inline]
#[cfg(not(feature = "std"))]
fn serialize_to_scratch<T: ?Sized + Serialize>(value: &T, scratch: &mut Vec<u8>) {
    *scratch = serde_json::to_vec(value).expect("serializing to a byte vector should not fail");
}

#[inline]
fn scratch_as_str(scratch: &[u8]) -> &str {
    // SAFETY: serde_json guarantees that serialization only writes valid UTF-8 bytes.
    unsafe { from_utf8_unchecked(scratch) }
}

#[inline]
pub(crate) fn encode_display_to_vec<T: ?Sized + Display>(
    value: &T,
    scratch: &mut Vec<u8>,
    output: &mut Vec<u8>,
) {
    serialize_to_scratch(&DisplayAsString(value), scratch);
    html_escape::encode_script_to_vec(scratch_as_str(scratch), output);
}

#[cfg(feature = "std")]
#[inline]
pub(crate) fn encode_display_to_writer<T: ?Sized + Display, W: Write>(
    value: &T,
    scratch: &mut Vec<u8>,
    output: &mut W,
) -> Result<(), io::Error> {
    serialize_to_scratch(&DisplayAsString(value), scratch);
    html_escape::encode_script_to_writer(scratch_as_str(scratch), output)
}

#[cfg(feature = "serde")]
#[inline]
pub(crate) fn encode_serializable_to_vec<T: ?Sized + Serialize>(
    value: &T,
    scratch: &mut Vec<u8>,
    output: &mut Vec<u8>,
) {
    serialize_to_scratch(value, scratch);
    html_escape::encode_script_to_vec(scratch_as_str(scratch), output);
}

#[cfg(all(feature = "serde", feature = "std"))]
#[inline]
pub(crate) fn encode_serializable_to_writer<T: ?Sized + Serialize, W: Write>(
    value: &T,
    scratch: &mut Vec<u8>,
    output: &mut W,
) -> Result<(), io::Error> {
    serialize_to_scratch(value, scratch);
    html_escape::encode_script_to_writer(scratch_as_str(scratch), output)
}
