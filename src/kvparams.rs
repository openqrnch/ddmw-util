//! Utility functions for managing key-value parameter buffers, sometimes used
//! for metadata buffers in DDMW.

use std::collections::HashMap;

/// A key-value parameter buffer is implemented as a HashMap<String,String>.
/// This function precalculates the size requirement for the serialized version
/// of the kvparam buffer.
///
/// Note that a trailing empty line is required by convention for kvparams.
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
