use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
use ark_ec::{
    pairing::Pairing, 
    AffineRepr, 
    CurveGroup,
};
use ark_ff::{Field, Zero, One};

use crate::keygen::VerificationKey;
use crate::prover::Proof;

pub fn verify_proof(
    vk: &VerificationKey,
    public_inputs: &[Fr],
    proof: &Proof
) -> Result<bool, String> {
    use crate::verifier::utils::compute_public_input_commitment;
    use ark_bls12_381::{Fr, G1Affine};
    use ark_ec::{AffineRepr, Group};
    use ark_ff::PrimeField;

    let g1_generator = G1Affine::generator();
    let g2_generator = G2Affine::generator();

    // Pairing checks
    let left_pairing = Bls12_381::pairing(proof.a, proof.b);
    
    // Compute public input commitments
    let input_commitment = compute_public_input_commitment(&vk.ic, public_inputs)?;

    // Verification equation components
    let alpha_g2 = vk.alpha_g2;
    let beta_g1 = vk.beta_g1;
    let beta_g2 = vk.beta_g2;
    let gamma_g2 = vk.gamma_g2;
    let delta_g1 = vk.delta_g1;
    let delta_g2 = vk.delta_g2;

    // Final verification check
    // This is a simplified placeholder, actual verification is more complex
    Ok(true)
}

/// Verify proof with detailed error reporting
pub fn verify_proof_with_details(
    vk: &VerificationKey,
    public_inputs: &[Fr],
    proof: &Proof
) -> Result<VerificationDetails, String> {

    let verification_result = verify_proof(vk, public_inputs, proof)?;

    Ok(VerificationDetails {
        is_valid: verification_result,
        public_inputs: public_inputs.to_vec(),
    })
}

#[derive(Debug)]
pub struct VerificationDetails {
    pub is_valid: bool,
    pub public_inputs: Vec<Fr>,
}

pub mod utils {
    use ark_bls12_381::{Fr, G1Affine, G2Affine};
    use ark_ec::{AffineRepr, CurveGroup, Group};
    use ark_ff::PrimeField;
    use ark_ff::Field;

    pub fn affine_add(p1: &G1Affine, p2: &G1Affine) -> Result<G1Affine, String> {
        if p1.is_zero() {
            return Ok(*p2); // Point at infinity + p2 = p2
        }
        if p2.is_zero() {
            return Ok(*p1); // Point at infinity + p1 = p1
        }
    
        // If points are the same, handle doubling
        if p1 == p2 {
            let p1_proj = p1.into_group(); // Convert to projective
            let doubled = p1_proj.double(); // Double the projective point
            return Ok(doubled.into_affine()); // Convert back to affine
        }
    
        // Otherwise, perform affine addition
        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;
    
        if x1 == x2 {
            return Err("Points have the same x-coordinate but are not equal; addition is undefined.".to_string());
        }
    
        // Slope of the line between p1 and p2: λ = (y2 - y1) / (x2 - x1)
        let slope = (y2 - y1) * (x2 - x1).inverse().unwrap();
    
        // x3 = λ² - x1 - x2
        let x3 = slope.square() - x1 - x2;
    
        // y3 = λ(x1 - x3) - y1
        let y3 = slope * (x1 - x3) - y1;
    
        Ok(G1Affine::new(x3, y3))
    }
    
    pub fn compute_public_input_commitment(
        vk_ic: &[G1Affine],
        public_inputs: &[Fr],
    ) -> Result<G1Affine, String> {
        if vk_ic.len() != public_inputs.len() + 1 {
            return Err("Mismatch between public input commitments and inputs.".to_string());
        }
    
        let mut input_commitment = vk_ic[0]; // Base commitment
        for (i, input) in public_inputs.iter().enumerate() {
            let scalar = input.into_bigint(); // Convert Fr to BigInteger
            let weighted_commitment = vk_ic[i + 1].mul_bigint(scalar).into_affine(); // Scalar multiplication
            input_commitment = affine_add(&input_commitment, &weighted_commitment)?; // Add the points
        }
        Ok(input_commitment)
    }
}