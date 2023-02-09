# Elliptic Curve Cryptography on Soroban

Contract examples and reusable primitives.

## [Groth 16 verifier](https://github.com/xycloo/ecc-soroban/tree/main/groth-16-verifier).

This crate provides a `SorobanGroth16Verifier` object that can verify zk-SNARKs with the groth16 algorithm:

$$( P_a \times P_b) \cdot (L_i \times -VK_{\gamma}) \cdot (P_c \times -VK_{\delta}) = VK_{\alpha} \times VK_{\beta}$$

Where:
- P is the proof.
- L is the aggregated inputs.
- VK is the verifiyng key.

### Contract example

```rust
pub struct TestVerifier;

#[contractimpl]
impl TestVerifier {
    pub fn init(env: Env, vk_hash: BytesN<32>) {
        env.storage().set(symbol!("VK"), vk_hash)
    }

    pub fn verify(env: Env, key: Bytes, proof: Bytes, image: Vec<Bytes>) -> bool {
        let vk_hash = env.storage().get(symbol!("VK")).unwrap().unwrap();
        let verifier = SorobanGroth16Verifier::load_with_vk_hash(vk_hash);

        verifier.verify(&env, key, proof, image)
    }
}
```

The implementation uses memory allocation on the contract's side, which is inefficient in Soroban (as it should), so contracts that use this verifier on chain will most likely trap as they exceed the CPU limits. However, you can try it out in your tests or in a local vm with the `--unlimited-budget`, for example:

```
soroban contract invoke --id 1 --fn verify --wasm ../../../ecc-soroban/target/wasm32-unknown-unknown/release/soroban_groth16_verifier_test.wasm --unlimited-budget -- --key_bytes "4f14e0df744c920c5b7a03159b24edfc8f799fa14ffd67614e0acb56cbacd7b82fcba2423a1f4a2f1fe8094ae3e36300b49fa586c9d9d85a750eb4b920136ae53713e74955a6dff390a629d56b7c04ed3c6f1c912296977751c25f7e4467ab01d0040877903290ded0da42926bca732d8b61fb1b92ab67621d570f68a05ec5d6b1c32abab79ad4487f6fba7f55442a01a9dcb92c6c7dc11136f693332b7681488cc7376ab2f7b80dd8a6c4972dddd50ec91caf8e8213a501d2adca45f277ed0066fa44125c0d74b9870d65abd77f3fd2582efa2476407ca3c821d72acefb4ee0daab44d7e56afcc02713c405112d60008d06a90021fddc33802698512795c313ba5b86eadf63e3eef495da92a5b9e387741fb22f5289322d3f065b910330a80088ac820b4cd95f5d15afa18421ae03b7ab8704035d9eed5ea44db48775bf21c78c9f620d07718dbcdd92b0c3db521300b64f8d5d25481798d62b96af0c086f47b6172c9a3885ad204ec4e8c3cb5f0a75ff439a2dafa4d0ebf8bbdbbb7e1fa50037de4449717351307e61b275d4d63ec2c3ee0b6759f20e72dd7bf66bcfbfe5b839aae7d243231d3416f61e55d3322f00dc7dd775d7fda1ad657644d6e5db502ddbea6421a8f25eb1a47f4cd5b04416ea012a72be4c3ed1a9e543db536e1f430045f1727ea7acac8213ab78942571a3b0f9f87542ecc36ae1853c8b085433c465bd69d7d1397285834862ef428c6a46009cfc55dcdb4d5cf2ffd7f364ec6bda92eaf053771e44b0c35be6359ac041dd56505d48992d7c5f69cde0c3bdc1aa1600438e0f1f09bdf95e769317dc56670668e06233c6401591ef79d5a42807dc035205c46b99d6a050e19285c76494f5fb00df033c206baa6e05e299287ca5bea07b511884640bd0ddeadd2a2ba83a900ff16fab1da8b08b5c3689b573bbbd77b10002000000000000000bbdce2efbd939093b9f09d1ced755b7601fc8e96f1323fc5f8a0921bb294ea81ab5488432edfe77ca5ab3a9714c5200dc3bad2f2064b9493b6657a523509c93c5770509fde8a3725cbd36f3b53355282606101fda7173e752667e2bcf547c0072c498765656533e8fe7f7011335838aa1dd33cd4df1e79548d9bbb2f318951758e7ef202138a8ea439cae191d914c0017287394cbd31937e6e2c666e04378ba6c89fa13822ab2257eb6869a6dbe65f4b163d705f8d1ed24ce912623879b6900" --proof_bytes "7b9ae6b72e7fd138c9182f75983c25a96bb0cf9540d5d91593ea3ad57c118d631028f663b96c38cff59a4bb534a5190064bb193be871e4bfb1f9e5d0254a064bf0891158c9a112ba2eb2ffc3d724c752148d720e81eb5d0b3067de78adc71501be353a6d79ed1d162b80959f9f1255f090d17a500273f7c18ee01b552af46cdfd587e33e70e7f8d510c30c4ce80f15016565aeab62368d58ab0448a7afcf360a91eb5ae86e4c8e53fed2ab114c497dc879ef192f53ae9237566ceddf4f345701fe095edf6a77e65852d95d7624e9c50189ed95a5d94693d1f0e8eebdc3831b79d9df0388dc973fb834f571642da97001185f3cb7e16937e60f18bfec512dcacc48e6b3297f7dca8986f36c8da5c9d5a06e8e80bb4ba99dea45a564e3553ca3007e4b94803458570e128db231db378466011d0421fe86f314faf357ea1a71306879d3fcad23b8980edd30a7d601349601b025cd4ac375e90e07aeb4e48d78c1d8ca7369117ff4bb8423da870a143fc6616e8411d4c10c947592bd48814caf5000"

result: true

```


### Credits
Parts of the code in this repo depend on:
- [arkworks ecosystem](https://arkworks.rs/)
