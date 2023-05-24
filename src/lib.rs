use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path, result::Result};

mod extension;
mod v2;
mod v3;

pub use extension::*;
pub use v2::*;
pub use v3::*;
/// Supported versions of the OpenApi.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Doc {
    V2(Swagger),
    V3(v3::OpenApi),
}

/// deserialize an open api spec from a path
pub fn from_path<P>(path: P) -> Result<Doc, serde_json::Error>
where
    P: AsRef<Path>,
{
    from_reader(File::open(path).unwrap())
}

/// deserialize from a string
pub fn from_str(str: &str) -> Result<Doc, serde_json::Error> {
    serde_json::from_str(str)
}

/// deserialize an open api spec from type which implements Read
pub fn from_reader<R>(read: R) -> Result<Doc, serde_json::Error>
where
    R: Read,
{
    serde_json::from_reader::<R, Doc>(read)
}

/// deserialize an swagger from type which implements Read
pub fn swagger_from_reader<R>(read: R) -> Result<Swagger, serde_json::Error>
where
    R: Read,
{
    serde_json::from_reader::<R, Swagger>(read)
}

/// deserialize an openapi from type which implements Read
pub fn openapi_from_reader<R>(read: R) -> Result<v3::OpenApi, serde_json::Error>
where
    R: Read,
{
    serde_json::from_reader::<R, v3::OpenApi>(read)
}
