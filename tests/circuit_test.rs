#[cfg(test)]
mod tests {
    use ark_bls12_381::Fr;
    use ark_relations::r1cs::ConstraintSynthesizer;
    use grothprotocol::circuit::Circuit; // use a specific finite field

    #[test]
    fn test_circuit_constraints() {
        let a = Fr::from(3u64);
        let b = Fr::from(4u64);
        let c = Fr::from(12u64);

        let circuit = Circuit {
            a: Some(a),
            b: Some(b),
            c: Some(c),
        };

        let cs = ark_relations::r1cs::ConstraintSystem::<Fr>::new_ref();

        circuit.generate_constraints(cs.clone()).unwrap();

        assert!(cs.is_satisfied().unwrap());
    }
}
