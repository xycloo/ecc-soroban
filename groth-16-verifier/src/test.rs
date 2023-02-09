use soroban_sdk::Env;
use crate::*;

#[test]
fn verify_test()
{
    let env = Env::default();
    let proof_bytes = TestVerifier::prove(env.clone());
    TestVerifier::verify(env, proof_bytes);  
}