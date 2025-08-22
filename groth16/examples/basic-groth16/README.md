# Basic Groth16 zkSNARK Example

This example demonstrates a complete Groth16 zkSNARK workflow implementing a **square root proof circuit**. The circuit allows a prover to demonstrate knowledge of a square root without revealing the actual value.

## Circuit Description

The circuit implements a simple but meaningful constraint:
- **Public input**: `y` (the square of the secret)
- **Private witness**: `x` (the secret square root)
- **Constraint**: `x * x = y`

This proves that the prover knows a value `x` such that `x² = y` without revealing `x` to the verifier.

## Features Demonstrated

1. **Circuit Design**: Creation of a custom R1CS constraint system
2. **Trusted Setup**: Generation of proving and verifying keys using BLS12-381 curve
3. **Zero-Knowledge Proof**: Generation of cryptographic proofs with private inputs
4. **Proof Verification**: Verification of proofs using only public inputs
5. **Security Properties**: Demonstration that invalid witnesses cannot generate valid proofs
6. **Performance Metrics**: Timing information for all phases

## Running the Example

From the `groth16` directory, run:

```bash
cargo run --example basic-groth16 --features="std"
```

## Expected Output

The example will show:

1. **Problem Definition**: Display the secret value and its square
2. **Trusted Setup**: Generate cryptographic parameters (≈100ms)
3. **Proof Generation**: Create zero-knowledge proof (≈30ms)
4. **Proof Verification**: Verify the proof (≈50ms)
5. **Security Demonstration**: Show that invalid inputs are rejected
6. **Performance Summary**: Complete timing breakdown
7. **Technical Details**: Cryptographic specifications

## Sample Output

```
🔐 Complete Groth16 zkSNARK Example: Square Root Proof
=====================================================

📋 Step 1: Define the Problem Instance
----------------------------------------
Secret value (x): 42
Public value (y = x²): 1764

🔧 Step 2: Trusted Setup (Generating Keys)
-------------------------------------------
✅ Trusted setup completed in: 108.326984ms
   - Proving key generated (size: 3 constraints)
   - Verifying key generated

🔍 Step 3: Generate Zero-Knowledge Proof
-----------------------------------------
✅ Proof generated in: 34.212181ms
   - Proof demonstrates knowledge of square root
   - Secret value remains hidden

✅ Step 4: Verify the Proof
---------------------------
🎉 Proof verification: SUCCESSFUL
   - The prover knows a value x such that x² = 1764
   - The secret x was not revealed during verification
   - Verification completed in: 56.134965ms

🛡️  Step 5: Security Demonstration
----------------------------------
✅ Security verified: Invalid witness values are rejected
   - Groth16 enforces mathematical constraints at circuit level
   - Only valid witness satisfying x² = y can generate proofs
   - System prevents creation of invalid proofs

📊 Performance Summary
=====================
Setup time:        108.326984ms
Proving time:      34.212181ms
Verification time: 56.134965ms
Total time:        198.674130ms

🎯 Zero-Knowledge Properties Achieved:
=====================================
✓ Completeness: Valid proofs are accepted
✓ Soundness: Invalid proofs are rejected
✓ Zero-Knowledge: Secret value remains hidden
✓ Efficiency: Fast verification independent of secret size

🔬 Technical Details:
====================
Curve: BLS12-381 (256-bit security)
Proof system: Groth16 zkSNARK
Circuit constraints: 1 (x * x = y)
Public inputs: 1 (y)
Private witnesses: 1 (x)
```

## Code Structure

- **`main.rs`**: Complete workflow implementation with error handling and timing
- **`circuit.rs`**: Square root circuit definition with constraint generation

## Technical Specifications

- **Cryptographic Curve**: BLS12-381 (128-bit security level)
- **Proving System**: Groth16 zkSNARK
- **Circuit Complexity**: 1 constraint (x * x = y)
- **Setup**: Universal (circuit-specific)
- **Proof Size**: Constant (3 group elements)
- **Verification**: Fast and constant time

## Educational Value

This example teaches:

1. **R1CS Constraint Systems**: How to build arithmetic circuits
2. **zkSNARK Workflow**: Complete setup → prove → verify process
3. **Zero-Knowledge Properties**: Completeness, soundness, and privacy
4. **Performance Characteristics**: Real-world timing measurements
5. **Security Model**: How invalid proofs are prevented

## Dependencies

- `ark-ff`: Finite field arithmetic
- `ark-bls12-381`: BLS12-381 elliptic curve implementation
- `ark-groth16`: Groth16 zkSNARK implementation
- `ark-relations`: R1CS constraint system
- `ark-std`: Standard library utilities

## Next Steps

To extend this example:

1. **More Complex Circuits**: Add multiple constraints and variables
2. **Different Curves**: Try other pairing-friendly curves
3. **Batch Verification**: Verify multiple proofs simultaneously
4. **Circuit Optimizations**: Minimize constraints for better performance
5. **Real Applications**: Implement practical use cases like range proofs