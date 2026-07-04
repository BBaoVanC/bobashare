//! A simple ephemeral file sharing service. This crate contains the backend
//! API.

pub mod serde;
pub mod storage;

use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};

/// Generate a randomized (alphanumeric) ID for an upload with a specified
/// length.
pub fn generate_randomized_id(length: usize) -> String {
    Alphanumeric.sample_string(&mut rng(), length)
}

/// Generate alphanumeric delete key for an upload. Length is 32.
pub fn generate_delete_key() -> String {
    Alphanumeric.sample_string(&mut rng(), 32)
}
