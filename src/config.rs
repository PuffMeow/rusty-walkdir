use serde::Deserialize;

pub const FOLLOW_SYMLINKS: bool = false;
pub const MAX_DEPTH: i32 = std::i32::MAX;
pub const MIN_DEPTH: i32 = 0;

#[napi(object, js_name = "IConfig")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub follow_symlinks: Option<bool>,
  pub max_depth: Option<i32>,
  pub min_depth: Option<i32>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      follow_symlinks: Some(FOLLOW_SYMLINKS),
      max_depth: Some(MAX_DEPTH),
      min_depth: Some(MIN_DEPTH),
    }
  }
}
