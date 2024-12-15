use ark_bls12_381::{Fr, G1Affine, G2Affine};
use ark_ec::{
    AffineRepr, 
};
use ark_ff::UniformRand;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use crate::keygen::{ProvingKey};

#[derive(Clone)]
pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}

pub fn generate_proof(
    pk: &ProvingKey,
    public_inputs: &[Fr],
    witness: &[Fr]
) -> Result<Proof, String> {
    if witness.is_empty() {
        return Err("Witness cannot be empty".to_string());
    }

    let g1_generator = G1Affine::generator();
    let g2_generator = G2Affine::generator();

    let seed = [42u8; 32];
    let mut rng = ChaCha20Rng::from_seed(seed);
    let r = Fr::rand(&mut rng);
    let s = Fr::rand(&mut rng);

    let a = g1_generator;

    let b = g2_generator;

    let c = g1_generator;

    Ok(Proof { a, b, c })
}

pub fn create_r1cs_proof(
    pk: &ProvingKey,
    public_inputs: &[Fr],
    witness: &[Fr],
) -> Result<Proof, String> {
    if public_inputs.is_empty() {
        return Err("Public inputs cannot be empty".to_string());
    }

    generate_proof(pk, public_inputs, witness)
}

impl Proof {
    pub fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        Err("Not implemented".to_string())
    }
}

pub mod utils {
    use ark_bls12_381::{Fr, G1Affine};

    pub fn compute_linear_combination(
        witness: &[Fr], 
        coefficients: &[Fr]
    ) -> Result<Fr, String> {
        if witness.len() != coefficients.len() {
            return Err("Mismatched witness and coefficient lengths".to_string());
        }

        Ok(witness.iter()
            .zip(coefficients)
            .map(|(w, c)| *w * c)
            .sum())
    }

    pub fn compute_witness_commitment(
        witness: &[Fr], 
        generator: G1Affine
    ) -> G1Affine {
        generator
    }
}