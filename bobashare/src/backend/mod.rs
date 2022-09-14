//! TODO: write description

pub mod storage;
pub mod serde;

// use chrono::prelude::*;

use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

pub fn generate_randomized_name(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}
