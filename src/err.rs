use std::fmt;

#[derive(Debug)]
pub enum Error {
  BadFormat(String),
  Figment(String),
  FileNotFound(String),
  SerializeError(String)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Error::BadFormat(s) => {
        write!(f, "Bad format; {}", s)
      }
      Error::Figment(s) => {
        write!(f, "Figiment error; {}", s)
      }
      Error::FileNotFound(s) => {
        write!(f, "File not found; {}", s)
      }
      Error::SerializeError(s) => {
        write!(f, "Unable to serialize; {}", s)
      }
    }
  }
}

impl From<figment::Error> for Error {
  fn from(err: figment::Error) -> Self {
    Error::Figment(err.to_string())
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
