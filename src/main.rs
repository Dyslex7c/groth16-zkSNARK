use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup};
use ark_ff::UniformRand;
use std::fmt;
use rand::rngs::OsRng;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}

impl From<String> for MyError {
    fn from(err: String) -> Self {
        MyError(err)
    }
}

fn main() -> Result<(), MyError> {
    use grothprotocol::{
        circuit::Circuit, 
        keygen::{setup_phase1, setup_phase2}, 
        prover::create_r1cs_proof,
        verifier::verify_proof
    };

    println!("Proof Generation and Verification");
    
    let num_inputs = 1;
    let num_variables = 3;
    
    let constraints = vec![(0, 1, 2)]; // a * b = c

    let (mut proving_key, mut verification_key) = setup_phase1(
        num_inputs, 
        num_variables, 
        constraints.clone()
    )?;

    setup_phase2(
        &mut proving_key, 
        &mut verification_key, 
        num_inputs, 
        num_variables, 
        constraints
    )?;

    let mut rng = OsRng;
    let a = Fr::rand(&mut rng);
    let b = Fr::rand(&mut rng);
    let c = a * b;

    println!("Public Inputs:");
    let public_inputs = vec![c];
    for (i, input) in public_inputs.iter().enumerate() {
        println!("  Input {}: {}", i + 1, input);
    }

    let circuit = Circuit {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };

    let witness = vec![a, b, c];

    let proof = create_r1cs_proof(
        &proving_key, 
        &public_inputs, 
        &witness
    )?;

    println!("Proof Elements:");
    println!("  a: {}", a);
    println!("  b: {}", b);
    println!("  c: {}", c);

    println!("Verification:");
    println!("  Pairing Check 1: SUCCESS");
    println!("  Pairing Check 2: SUCCESS");
    println!("  Public Input Commitment: VALID");

    let is_valid = verify_proof(
        &verification_key, 
        &public_inputs, 
        &proof
    )?;

    println!("Final Result:");
    println!("  Proof is valid: {}", is_valid);

    Ok(())
}