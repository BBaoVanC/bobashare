//! A simple ephemeral file sharing service. This crate contains the backend
//! API.

pub mod serde;
pub mod storage;

use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

/// Generate a randomized (alphanumeric) ID for an upload with a specified
/// length.
pub fn generate_randomized_id(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}
