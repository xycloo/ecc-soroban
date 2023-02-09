use crate::*;
use soroban_sdk::Env;
extern crate std;

#[test]
fn verify_test() {
    let env = Env::default();
    let proof_bytes = TestVerifier::prove(env.clone());
    let key_bytes = TestVerifier::get_key(env.clone());

    std::println!("{:?} \n\n", proof_bytes);
    std::println!("{:?}", key_bytes);

    let res = TestVerifier::verify(env, key_bytes, proof_bytes);

    assert!(res);
}
