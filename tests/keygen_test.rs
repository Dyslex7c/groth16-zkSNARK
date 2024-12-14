#[cfg(test)]
mod tests {
    use ark_bls12_381::{G1Affine, G2Affine};
    use grothprotocol::keygen::{
        generate_toxic_waste, setup_phase1, setup_phase2, ProvingKey, VerificationKey,
    };
    use rand_chacha::ChaCha20Rng;
    use rand_core::SeedableRng;

    #[test]
    fn test_generate_toxic_waste() {
        let seed = [42u8; 32];
        let mut rng = ChaCha20Rng::from_seed(seed);
        let (a, b, c, d) = generate_toxic_waste(&mut rng);

        assert!(
            a != b && a != c && a != d && b != c && b != d && c != d,
            "Generated toxic waste elements should be distinct."
        );
    }

    #[test]
    fn test_setup_phase1() {
        let num_inputs = 3;
        let num_variables = 5;
        let constraints = vec![(0, 1, 2)];

        let result = setup_phase1(num_inputs, num_variables, constraints);

        assert!(result.is_ok(), "Setup phase 1 should succeed.");
        let (proving_key, verification_key) = result.unwrap();

        assert!(
            proving_key.a1 != G1Affine::identity(),
            "ProvingKey.a1 should not be identity."
        );
        assert!(
            proving_key.b2 != G2Affine::identity(),
            "ProvingKey.b2 should not be identity."
        );
        assert!(
            proving_key.alpha_g1 != G1Affine::identity(),
            "ProvingKey.alpha_g1 should not be identity."
        );

        assert!(
            verification_key.alpha_g2 != G2Affine::identity(),
            "VerificationKey.alpha_g2 should not be identity."
        );
        assert_eq!(
            verification_key.ic.len(),
            num_inputs + 1,
            "VerificationKey.ic length should match num_inputs + 1."
        );
    }

    #[test]
    fn test_setup_phase2() {
        let num_inputs = 1;
        let num_variables = 3;
        let constraints = vec![(0, 1, 2)];

        let mut proving_key = ProvingKey {
            a1: G1Affine::identity(),
            b2: G2Affine::identity(),
            c1: G1Affine::identity(),
            alpha_g1: G1Affine::identity(),
            beta_g1: G1Affine::identity(),
            delta_g1: G1Affine::identity(),
            gamma_g1: G1Affine::identity(),
        };

        let mut verification_key = VerificationKey {
            alpha_g2: G2Affine::identity(),
            beta_g1: G1Affine::identity(),
            beta_g2: G2Affine::identity(),
            gamma_g2: G2Affine::identity(),
            delta_g1: G1Affine::identity(),
            delta_g2: G2Affine::identity(),
            ic: Vec::new(),
        };

        let result = setup_phase2(
            &mut proving_key,
            &mut verification_key,
            num_inputs,
            num_variables,
            constraints,
        );

        assert!(result.is_ok(), "Setup phase 2 should succeed.");

        assert!(
            proving_key.a1 != G1Affine::identity(),
            "ProvingKey.a1 should be updated."
        );
        assert!(
            proving_key.b2 != G2Affine::identity(),
            "ProvingKey.b2 should be updated."
        );
        assert!(
            proving_key.c1 != G1Affine::identity(),
            "ProvingKey.c1 should be updated."
        );

        assert_eq!(
            verification_key.ic.len(),
            num_inputs + 1,
            "VerificationKey.ic length should match num_inputs + 1."
        );
    }

    #[test]
    fn test_serialization() {
        let proving_key = ProvingKey {
            a1: G1Affine::identity(),
            b2: G2Affine::identity(),
            c1: G1Affine::identity(),
            alpha_g1: G1Affine::identity(),
            beta_g1: G1Affine::identity(),
            delta_g1: G1Affine::identity(),
            gamma_g1: G1Affine::identity(),
        };

        let serialized = proving_key.serialize();
        assert!(
            serialized.is_empty(),
            "Serialization not yet implemented but should return an empty Vec."
        );

        let deserialized = ProvingKey::deserialize(&serialized);
        assert!(
            deserialized.is_err(),
            "Deserialization should return an error since it's not implemented."
        );
    }
}
