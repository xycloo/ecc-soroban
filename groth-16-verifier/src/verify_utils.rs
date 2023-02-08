use core::ops::{AddAssign, MulAssign, Neg};

use ark_bls12_377::{Bls12_377, Fq12Parameters, Fr};
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::{Fp12ParamsWrapper, PrimeField, QuadExtField};

use crate::types::{PreparedVK, Proof, VerifyingKey};

/// Prepare proof inputs for use with [`verify_proof_with_prepared_inputs`], wrt the prepared
/// verification key `pvk` and instance public inputs.
// froom ark_groth16
pub fn aggregate_inputs(
    prep_vk: &PreparedVK<Bls12_377>,
    pub_inputs: &[<Bls12_377 as PairingEngine>::Fr],
) -> <Bls12_377 as PairingEngine>::G1Projective {
    if (pub_inputs.len() + 1) != prep_vk.vk.gamma_abc_g1.len() {
        panic!("Malformed key");
    }

    let mut g_ic = prep_vk.vk.gamma_abc_g1[0].into_projective();
    for (i, b) in pub_inputs
        .iter()
        .zip(prep_vk.vk.gamma_abc_g1.iter().skip(1))
    {
        g_ic.add_assign(&b.mul(i.into_repr()));
    }

    g_ic
}

/// precompute params to be used in the verifying key
pub fn prepare_vk<E>(vk: &VerifyingKey<E>) -> PreparedVK<E>
where
    E: PairingEngine,
{
    PreparedVK {
        vk: vk.clone(),
        e_alpha_beta: E::pairing(vk.alpha_g1, vk.beta_g2),
        gamma_neg: vk.gamma_g2.neg(),
        delta_neg: vk.delta_g2.neg(),
    }
}

/// groth16 equation
pub fn verify_eq(
    e_a_b: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    e_l_ngamma: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    e_c_ndelta: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
    e_alpha_beta: QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>,
) -> bool {
    let mut lhs = e_a_b;
    lhs.mul_assign(e_l_ngamma);
    lhs.mul_assign(e_c_ndelta);

    lhs.eq(&e_alpha_beta)
}

/// compute pairings and verify a proof
pub fn verify(proof: Proof<Bls12_377>, prep_vk: PreparedVK<Bls12_377>, pub_inputs: &[Fr]) -> bool {
    let l = aggregate_inputs(&prep_vk, pub_inputs);
    let e_a_b = Bls12_377::pairing(proof.a, proof.b);
    let e_l_ngamma = Bls12_377::pairing(l, prep_vk.gamma_neg);
    let e_c_ndelta = Bls12_377::pairing(proof.c, prep_vk.delta_neg);

    verify_eq(e_a_b, e_l_ngamma, e_c_ndelta, prep_vk.e_alpha_beta)
}
