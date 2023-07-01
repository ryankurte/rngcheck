//! A helper crate for testing the operation of cryptographic random number generators
//!
//! This provides a subset of tests from NIST 800-22 and the Diehard / Dieharder suites
//! suitable for runtime testing of RNGs.
//!
//! NOTE: This is a an incomplete and broadly untested implementation, to be extended as is useful / required.
//! If we're missing a test that would be useful, please feel free to open an issue or PR!

#![no_std]

pub mod helpers;
pub mod nist;

/// Test errors
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    /// RNG Failed
    RngFailed,

    /// Insufficient sample size
    InsufficientSampleSize(usize),

    /// P-value outside required bounds
    BadPValue(f32),
}
