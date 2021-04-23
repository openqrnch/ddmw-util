//! Helper module for common DDMW applications.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use figment::{
  providers::{Format, Toml},
  Figment
};

use crate::err::Error;


#[derive(Debug, Default, Deserialize)]
pub struct Config {
  pub channel: Option<u8>,
  pub auth: Option<Auth>,
  pub sender: Option<Sender>,
  pub receiver: Option<Receiver>
}


#[derive(Debug, Default, Deserialize)]
pub struct Auth {
  pub name: Option<String>,
  pub pass: Option<String>,
  #[serde(rename = "pass-file")]
  pub pass_file: Option<String>,
  pub token: Option<String>,
  #[serde(rename = "token-file")]
  pub token_file: Option<String>
}


#[derive(Debug, Default, Deserialize)]
pub struct Sender {
  pub mgmtif: Option<String>,
  pub msgif: Option<String>
}


#[derive(Debug, Default, Deserialize)]
pub struct Receiver {
  pub mgmtif: Option<String>,
  pub subif: Option<String>,
  #[serde(rename = "sub-retries")]
  pub sub_retries: Option<u32>,
  #[serde(rename = "sub-retry-delay")]
  pub sub_retry_delay: Option<String>,
  #[serde(rename = "push-listenif")]
  pub push_listenif: Option<String>
}


/// Load a DDMW application configuration file.
///
/// This function will attempt to load a configuration file in the following
/// order:
/// 1. If `fname` is has `Some` value, that will be used.  If `fname` is None:
/// 2. If the environment variable `DDMW_APPCONF` is set, its value will be
///    used.  If it is not set:
/// 3. The filename `ddmwapp.toml`, in the current working directory, will be
///    used.
///
/// If none of these could be be found, `Ok(None)` will be returned.
///
/// # Example
/// Attempt to load a "hello.toml".
///
/// ```no_run
/// use std::path::Path;
/// use ddmw_util::app::{Config, load_conf};
/// use ddmw_util::Error;
/// fn get_conf() -> Result<Config, Error> {
///   let fname = Path::new("hello.toml");
///   Ok(load_conf(Some(&fname))?.unwrap_or_default())
/// }
/// ```
pub fn load_conf(fname: Option<&Path>) -> Result<Option<Config>, Error> {
  let f = match fname {
    Some(p) => p.to_path_buf(),
    None => match std::env::var_os("DDMW_APPCONF") {
      Some(val) => PathBuf::from(val),
      None => PathBuf::from("ddmwapp.toml")
    }
  };

  if !f.exists() {
    Ok(None)
  } else {
    let conf = Figment::new().merge(Toml::file(f)).extract()?;
    Ok(conf)
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
