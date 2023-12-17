#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::path::Path;

use anyhow::Result;
use config::{Config, FOLLOW_SYMLINKS, MAX_DEPTH, MIN_DEPTH};
use napi::bindgen_prelude::Buffer;
use serialize::get_deserialized;
use walkdir::WalkDir;
mod config;
mod serialize;

#[cfg(all(
  not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[napi(ts_args_type = "entry: string, callback: (path: string) => void")]
pub fn walkdir<T>(entry: String, callback: T)
where
  T: Fn(String) -> napi::Result<()>,
{
  if let Err(err) = inner_walkdir(entry, callback) {
    eprint!("{err}");
  }
}

pub fn inner_walkdir<T: Fn(String) -> napi::Result<()>>(entrance: String, callback: T) -> Result<()>
where
  T: Fn(String) -> napi::Result<()>,
{
  for entry in WalkDir::new(&entrance) {
    let path = String::from(entry?.path().to_string_lossy());
    let _ = callback(path);
  }

  Ok(())
}

#[napi(ts_args_type = "entry: string, config: IConfig, callback: (path: string) => void")]
pub fn walkdir_with_config<T>(entry: String, config: Buffer, callback: T)
where
  T: Fn(String) -> napi::Result<()>,
{
  let config: napi::Result<Config> = get_deserialized(&config);
  let config = config.unwrap_or_default();

  if let Err(err) = inner_walkdir_with_config(entry, config, callback) {
    eprint!("{err}");
  }
}

pub fn inner_walkdir_with_config<T: Fn(String) -> napi::Result<()>>(
  entrance: String,
  config: Config,
  callback: T,
) -> Result<()>
where
  T: Fn(String) -> napi::Result<()>,
{
  for entry in WalkDir::new(&entrance)
    .follow_links(config.follow_symlinks.unwrap_or_else(|| FOLLOW_SYMLINKS))
    .min_depth(config.min_depth.unwrap_or_else(|| MIN_DEPTH) as usize)
    .max_depth(config.max_depth.unwrap_or_else(|| MAX_DEPTH) as usize)
  {
    let path = String::from(entry?.path().to_string_lossy());
    let _ = callback(path);
  }

  Ok(())
}
