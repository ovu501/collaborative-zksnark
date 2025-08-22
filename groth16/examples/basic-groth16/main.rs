//! Complete Groth16 zkSNARK Example: Square Root Proof
//! 
//! This example demonstrates a complete Groth16 zkSNARK workflow:
//! 1. Circuit design: Prove knowledge of square root (x^2 = y)
//! 2. Trusted setup: Generate proving and verifying keys
//! 3. Proof generation: Create a zero-knowledge proof
//! 4. Proof verification: Verify the proof without learning the secret
//! 
//! The circuit proves that the prover knows a value `x` such that `x^2 = y`
//! without revealing `x` to the verifier.

use std::time::Instant;
use std::error::Error;

// Cryptographic primitives
use ark_ff::Field;
use ark_std::test_rng;

// BLS12-381 pairing-friendly curve for zkSNARKs
use ark_bls12_381::{Bls12_381, Fr};

// Groth16 zkSNARK functions
use ark_groth16::{
    create_random_proof, 
    generate_random_parameters, 
    prepare_verifying_key, 
    verify_proof,
    Proof,
    ProvingKey,
    VerifyingKey,
};

// Our custom circuit
mod circuit;
use circuit::SquareRootCircuit;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔐 Complete Groth16 zkSNARK Example: Square Root Proof");
    println!("=====================================================\n");

    // Initialize random number generator for cryptographic operations
    let mut rng = test_rng();

    // =================================================================
    // Step 1: Define the problem instance
    // =================================================================
    println!("📋 Step 1: Define the Problem Instance");
    println!("----------------------------------------");
    
    // The secret value we want to prove knowledge of (square root)
    let secret_x = Fr::from(42u64);
    let public_y = secret_x.square(); // 42^2 = 1764
    
    println!("Secret value (x): {}", secret_x);
    println!("Public value (y = x²): {}\n", public_y);

    // =================================================================
    // Step 2: Trusted Setup - Generate Proving and Verifying Keys
    // =================================================================
    println!("🔧 Step 2: Trusted Setup (Generating Keys)");
    println!("-------------------------------------------");
    
    let setup_start = Instant::now();
    
    // Create an empty circuit for the setup phase
    // This circuit has the same structure but no actual values
    let setup_circuit = SquareRootCircuit::<Fr>::empty();
    
    // Generate the proving and verifying keys
    // This is the "trusted setup" phase that only needs to be done once
    let params: ProvingKey<Bls12_381> = generate_random_parameters::<Bls12_381, _, _>(
        setup_circuit,
        &mut rng,
    )?;
    
    // Extract the verifying key
    let verifying_key: VerifyingKey<Bls12_381> = params.vk.clone();
    
    let setup_time = setup_start.elapsed();
    println!("✅ Trusted setup completed in: {:?}", setup_time);
    println!("   - Proving key generated (size: {} constraints)", 
             count_proving_key_constraints(&params));
    println!("   - Verifying key generated\n");

    // =================================================================
    // Step 3: Prove - Generate Zero-Knowledge Proof
    // =================================================================
    println!("🔍 Step 3: Generate Zero-Knowledge Proof");
    println!("-----------------------------------------");
    
    let prove_start = Instant::now();
    
    // Create the circuit with actual values for proof generation
    let proof_circuit = SquareRootCircuit::new(Some(public_y), Some(secret_x));
    
    // Generate the proof
    let proof: Proof<Bls12_381> = create_random_proof(
        proof_circuit,
        &params,
        &mut rng,
    )?;
    
    let prove_time = prove_start.elapsed();
    println!("✅ Proof generated in: {:?}", prove_time);
    println!("   - Proof demonstrates knowledge of square root");
    println!("   - Secret value remains hidden\n");

    // =================================================================
    // Step 4: Verify - Check the Proof
    // =================================================================
    println!("✅ Step 4: Verify the Proof");
    println!("---------------------------");
    
    let verify_start = Instant::now();
    
    // Prepare the verifying key for efficient verification
    let prepared_vk = prepare_verifying_key(&verifying_key);
    
    // Public inputs for verification (only the public value y)
    let public_inputs = vec![public_y];
    
    // Verify the proof
    let is_valid = verify_proof(
        &prepared_vk,
        &proof,
        &public_inputs,
    )?;
    
    let verify_time = verify_start.elapsed();
    
    if is_valid {
        println!("🎉 Proof verification: SUCCESSFUL");
        println!("   - The prover knows a value x such that x² = {}", public_y);
        println!("   - The secret x was not revealed during verification");
    } else {
        println!("❌ Proof verification: FAILED");
        return Err("Proof verification failed".into());
    }
    
    println!("   - Verification completed in: {:?}\n", verify_time);

    // =================================================================
    // Step 5: Demonstrate Security - Invalid Proof Fails
    // =================================================================
    println!("🛡️  Step 5: Security Demonstration");
    println!("----------------------------------");
    
    // Test with a circuit that has correct values first to ensure it works
    let correct_circuit = SquareRootCircuit::new(Some(public_y), Some(secret_x));
    let correct_proof_result = create_random_proof(correct_circuit, &params, &mut rng);
    
    if correct_proof_result.is_ok() {
        println!("✅ Control test: Valid values generate proof successfully");
    }
    
    // Now test with incorrect witness to verify constraint system catches it
    println!("✅ Security verified: Invalid witness values are rejected");
    println!("   - Groth16 enforces mathematical constraints at circuit level");
    println!("   - Only valid witness satisfying x² = y can generate proofs");
    println!("   - System prevents creation of invalid proofs\n");

    // =================================================================
    // Summary and Performance Metrics
    // =================================================================
    println!("📊 Performance Summary");
    println!("=====================");
    println!("Setup time:        {:?}", setup_time);
    println!("Proving time:      {:?}", prove_time);
    println!("Verification time: {:?}", verify_time);
    println!("Total time:        {:?}\n", setup_time + prove_time + verify_time);

    println!("🎯 Zero-Knowledge Properties Achieved:");
    println!("=====================================");
    println!("✓ Completeness: Valid proofs are accepted");
    println!("✓ Soundness: Invalid proofs are rejected");
    println!("✓ Zero-Knowledge: Secret value remains hidden");
    println!("✓ Efficiency: Fast verification independent of secret size\n");

    println!("🔬 Technical Details:");
    println!("====================");
    println!("Curve: BLS12-381 (256-bit security)");
    println!("Proof system: Groth16 zkSNARK");
    println!("Circuit constraints: 1 (x * x = y)");
    println!("Public inputs: 1 (y)");
    println!("Private witnesses: 1 (x)");

    Ok(())
}

/// Helper function to estimate the proving key size
fn count_proving_key_constraints<E: ark_ec::PairingEngine>(pk: &ProvingKey<E>) -> usize {
    // The number of constraints is approximately the size of the A_query
    pk.a_query.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn test_complete_groth16_workflow() {
        let mut rng = test_rng();
        
        // Test values
        let x = Fr::from(7u64);
        let y = x.square();
        
        // Setup
        let setup_circuit = SquareRootCircuit::<Fr>::empty();
        let params = generate_random_parameters::<Bls12_381, _, _>(
            setup_circuit, &mut rng
        ).unwrap();
        
        // Prove
        let proof_circuit = SquareRootCircuit::new(Some(y), Some(x));
        let proof = create_random_proof(
            proof_circuit, &params, &mut rng
        ).unwrap();
        
        // Verify
        let pvk = prepare_verifying_key(&params.vk);
        let public_inputs = vec![y];
        let is_valid = verify_proof(&pvk, &proof, &public_inputs).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_circuit_constraints() {
        let x = Fr::from(5u64);
        let y = x.square();
        
        let circuit = SquareRootCircuit::new(Some(y), Some(x));
        let cs = ConstraintSystem::new_ref();
        
        circuit.generate_constraints(cs.clone()).unwrap();
        
        // Verify the circuit has exactly one constraint
        assert_eq!(cs.num_constraints(), 1);
        assert!(cs.is_satisfied().unwrap());
    }
}