use ark_bls12_377::Bls12_377;
use ark_ec::PairingEngine;

use crate::{key_wrap::af_g1, key_wrap::af_g2, types::Proof};

/// Builds proof for given points as strings
pub fn build_proof<E>(a_b: &[&str], b_b: ([&str; 2], [&str; 2]), c_b: &[&str]) -> Proof<Bls12_377>
where
    E: PairingEngine,
{
    let a = af_g1::<E>(a_b[0], a_b[1]);
    let b = af_g2::<E>(b_b.0[0], b_b.0[1], b_b.1[0], b_b.1[1]);
    let c = af_g1::<E>(c_b[0], c_b[1]);

    Proof { a, b, c }
}
