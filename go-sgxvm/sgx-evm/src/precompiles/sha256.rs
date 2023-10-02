extern crate alloc;

use alloc::vec::Vec;
use core::cmp::min;

use k256::sha2::{Sha256 as kSha256, Digest};
use sha3::{Keccak256};
use k256::{
    ecdsa::recoverable,
    elliptic_curve::{sec1::ToEncodedPoint, IsHigh},
};
use crate::precompiles::{ExitSucceed, LinearCostPrecompile, PrecompileFailure};

/// The sha256 precompile.
pub struct Sha256;

impl LinearCostPrecompile for Sha256 {
    const BASE: u64 = 60;
    const WORD: u64 = 12;

    fn raw_execute(input: &[u8], _cost: u64) -> Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
        let mut hasher = kSha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        Ok((ExitSucceed::Returned, result.to_vec()))
    }
}