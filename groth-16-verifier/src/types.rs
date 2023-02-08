use ark_ec::PairingEngine;
use ark_serialize::*;
use ark_std::vec::Vec as ArkVec;

/// A proof in the Groth16 SNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof<E: PairingEngine> {
    /// The `A` element in `G1`.
    pub a: E::G1Affine,
    /// The `B` element in `G2`.
    pub b: E::G2Affine,
    /// The `C` element in `G1`.
    pub c: E::G1Affine,
}

/// A verification key in the Groth16 SNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct VerifyingKey<E: PairingEngine> {
    /// The `alpha * G`, where `G` is the generator of `E::G1`.
    pub alpha_g1: E::G1Affine,
    /// The `alpha * H`, where `H` is the generator of `E::G2`.
    pub beta_g2: E::G2Affine,
    /// The `gamma * H`, where `H` is the generator of `E::G2`.
    pub gamma_g2: E::G2Affine,
    /// The `delta * H`, where `H` is the generator of `E::G2`.
    pub delta_g2: E::G2Affine,
    /// The `gamma^{-1} * (beta * a_i + alpha * b_i + c_i) * H`, where `H` is the generator of `E::G1`.
    pub gamma_abc_g1: ArkVec<E::G1Affine>,
}

/// Preprocessed verifying key
pub struct PreparedVK<E: PairingEngine> {
    /// verifying key
    pub vk: VerifyingKey<E>,
    /// e(VK_\aplha, VK_\beta)
    pub e_alpha_beta: E::Fqk,
    /// -VK_\gamma
    pub gamma_neg: E::G2Affine,
    /// -VK_\delta
    pub delta_neg: E::G2Affine,
}
