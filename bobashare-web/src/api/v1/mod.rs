//! Version 1 of the bobashare API, hosted at `/api/v1/`

use serde::Serializer;

pub mod upload;

pub fn serialize_error<S>(err: &anyhow::Error, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&err.to_string())
}
