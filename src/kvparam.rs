//! Utility functions for managing key-value parameter buffers, sometimes used
//! for metadata buffers in DDMW.

use std::collections::HashMap;
use std::str::FromStr;

#[cfg(feature = "bytes")]
use bytes::{BytesMut, BufMut};

use crate::Error;

/// Representation of a list of key-value that can be serialized for commonly
/// used metadata format.
pub struct KVParams {
  hm: HashMap<String, String>
}

impl KVParams {
  /// Initialize a new key/value parameters object.
  pub fn new() -> Self {
    KVParams { hm: HashMap::new() }
  }

  /// Take ownership of a HashMap and create a key-value parameter object from
  /// it.
  pub fn from_hashmap(hm: HashMap<String, String>) -> Self {
    KVParams { hm }
  }

  /// Precalculate the size requirement for the serialized version
  /// of this buffer.
  ///
  /// This function returns 0 if there are no entries.
  #[inline]
  pub fn calc_buf_size(&self) -> usize {
    calc_buf_size(&self.hm)
  }

  /// Consume object and return its internal HashMap.
  pub fn into_inner(self) -> HashMap<String, String> {
    self.hm
  }

  /// Add a parameter to the message.
  pub fn add_param<T: ToString, U: ToString>(
      &mut self,
      key: T,
      value: U
  ) {
    // ToDo: Validate key name
    self.hm.insert(key.to_string(), value.to_string());
  }

  /// Get parameter string value for a key.  Returns Some(value) if the key
  /// exists, returns None otherwise.
  pub fn get_param(&self, key: &str) -> Option<&str> {
    let kv = self.hm.get_key_value(key);
    if let Some((_k, v)) = kv {
      return Some(v);
    }
    None
  }

  pub fn get_str(&self, key: &str) -> Option<&str> {
    self.get_param(key)
  }

  pub fn get_int<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!("Unable to parse numeric value from \
parameter '{}'", key)));
    }
    Err(Error::KeyNotFound(key.to_string()))
  }

  #[cfg(feature = "bytes")]
  pub fn write_bytes(&self, buf: &mut BytesMut) -> Result<(), Error> {
    // Calculate the required buffer size
    let sz = self.calc_buf_size();
    if sz == 0 {
      return Ok(());
    }

    // Reserve space
    buf.reserve(sz);

    // Write data to output buffer
    for (key, value) in &self.hm {
      buf.put(key.as_bytes());
      buf.put_u8(b' ');
      buf.put(value.as_bytes());
      buf.put_u8(b'\n');
    }
    buf.put_u8(b'\n');

    Ok(())
  }
}


/// A key-value parameter buffer is implemented as a HashMap<String,String>.
/// This function precalculates the size requirement for the serialized version
/// of the kvparam buffer.
///
/// Note that while a trailing empty line is required by convention for
/// KVparams if there are no parameters in the buffer a size of 0 will be
/// returned.
#[inline]
pub fn calc_buf_size(params: &HashMap<String, String>) -> usize {
  if params.is_empty() {
    return 0;
  }

  let mut sz = 0;
  for (k, v) in params.iter() {
    // key space + whitespace + value space + eol
    sz += k.len() + 1 + v.len() + 1;
  }
  // Terminating empty line
  sz += 1;

  sz
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_kvparam_size() {
    let params: HashMap<String, String> = HashMap::new();
    assert_eq!(calc_buf_size(&params), 0);
  }

  #[test]
  fn single_kvparam_size() {
    let mut params = HashMap::new();
    params.insert("a".to_string(), "b".to_string());
    assert_eq!(calc_buf_size(&params), 5);
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
