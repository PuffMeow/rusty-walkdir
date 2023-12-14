#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

#[napi(ts_args_type = "entry: string, callback: (path: string) => void")]
pub fn walkdir<T>(entry: String, callback: T)
where
  T: Fn(String) -> napi::Result<()>,
{
  if let Err(err) = inner_walkdir(entry, callback) {
    eprint!("{err}");
  }
}

pub fn inner_walkdir<P, T: Fn(String) -> napi::Result<()>>(entry: P, callback: T) -> Result<()>
where
  P: AsRef<Path>,
  T: Fn(String) -> napi::Result<()>,
{
  for entry in WalkDir::new(entry) {
    let path = String::from(entry?.path().to_string_lossy());
    let _ = callback(path);
  }

  Ok(())
}
