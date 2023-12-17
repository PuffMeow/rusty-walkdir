use std::any::type_name;

use anyhow::Context;
use napi::Status;
use serde::de::DeserializeOwned;

pub trait MapErr<T>: Into<Result<T, anyhow::Error>> {
  fn convert_err(self) -> napi::Result<T> {
    self
      .into()
      .map_err(|err| napi::Error::new(Status::GenericFailure, format!("{:?}", err)))
  }
}

impl<T> MapErr<T> for Result<T, anyhow::Error> {}

pub fn get_deserialized<T, B>(buffer: B) -> napi::Result<T>
where
  T: DeserializeOwned,
  B: AsRef<[u8]>,
{
  let mut deserializer = serde_json::Deserializer::from_slice(buffer.as_ref());

  let v = T::deserialize(&mut deserializer)
    .with_context(|| {
      format!(
        "Failed to deserialize buffer as {}\nJSON: {}",
        type_name::<T>(),
        String::from_utf8_lossy(buffer.as_ref())
      )
    })
    .convert_err()?;

  Ok(v)
}
