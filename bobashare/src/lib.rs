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

#[cfg(test)]
mod tests {
    // https://crates.io/crates/cargo-public-api
    // See readme for this code
    #[test]
    fn public_api() {
        // Install a compatible nightly toolchain if it is missing
        rustup_toolchain::install(public_api::MINIMUM_NIGHTLY_RUST_VERSION).unwrap();

        // Build rustdoc JSON
        let rustdoc_json = rustdoc_json::Builder::default()
            .toolchain(public_api::MINIMUM_NIGHTLY_RUST_VERSION)
            .build()
            .unwrap();

        // Derive the public API from the rustdoc JSON
        let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json)
            .build()
            .unwrap();

        // Assert that the public API looks correct
        expect_test::expect_file!["../public-api.txt"].assert_eq(&public_api.to_string());
    }
}
