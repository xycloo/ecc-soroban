#![no_std]
use crate::verify_utils::{prepare_vk, verify};
use ark_bls12_377::Bls12_377;
use ark_bls12_377::Fr;
use ark_serialize::CanonicalDeserialize;
use soroban_sdk::{contractimpl, symbol, Bytes, BytesN, Env, Vec};

extern crate alloc;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct SorobanGroth16Verifier {
    pub vk_hash: BytesN<32>,
}

impl SorobanGroth16Verifier {
    pub fn load_with_vk_hash(hash: BytesN<32>) -> Self {
        Self { vk_hash: hash }
    }

    pub fn verify(
        &self,
        env: &Env,
        key_bytes: Bytes,
        proof_bytes: Bytes,
        image_vbytes: Vec<Bytes>,
    ) -> bool {
        let mut hash_slice = [0; 32];
        self.vk_hash.copy_into_slice(&mut hash_slice);

        if env.crypto().sha256(&key_bytes).to_array() != hash_slice {
            panic!("invalid verifing key")
        }

        // deserialize proof
        let len = proof_bytes.len();
        let mut bvec = alloc::vec![0u8;len as usize];
        proof_bytes.copy_into_slice(bvec.as_mut_slice());
        let proof = types::Proof::deserialize_uncompressed(bvec.as_slice()).unwrap();

        // deserialize key
        let k_len = key_bytes.len();
        let mut k_bvec = alloc::vec![0u8;k_len as usize];
        key_bytes.copy_into_slice(k_bvec.as_mut_slice());
        let vk =
            types::VerifyingKey::<Bls12_377>::deserialize_uncompressed(k_bvec.as_slice()).unwrap();

        let prep_vk = prepare_vk(&vk);

        let mut vimage = alloc::vec![];

        for image_bytes in image_vbytes.iter() {
            let len = image_bytes.as_ref().unwrap().len();
            let mut i_bvec = alloc::vec![0u8; len as usize];
            image_bytes.unwrap().copy_into_slice(&mut i_bvec);

            let fr = Fr::deserialize_uncompressed(i_bvec.as_slice()).unwrap();
            vimage.push(fr)
        }

        verify(proof, prep_vk, vimage.as_slice())
    }
}

mod key_wrap;
mod proof_wrap;
#[cfg(test)]
mod test;
mod types;
mod verify_utils;
