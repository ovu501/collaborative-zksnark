use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use std::marker::PhantomData;

/// A circuit that proves knowledge of a square root.
/// 
/// The circuit takes:
/// - Public input: `y` (the square of the secret)
/// - Private witness: `x` (the secret square root)
/// 
/// The circuit enforces the constraint: `x * x = y`
/// This proves that the prover knows a value `x` such that `x^2 = y`
/// without revealing `x`.
pub struct SquareRootCircuit<F: Field> {
    /// The public input y (x^2)
    pub y: Option<F>,
    /// The private witness x (square root)
    pub x: Option<F>,
    /// Phantom data for the field type
    _field: PhantomData<F>,
}

impl<F: Field> SquareRootCircuit<F> {
    /// Create a new circuit with the given public input and private witness
    pub fn new(y: Option<F>, x: Option<F>) -> Self {
        Self {
            y,
            x,
            _field: PhantomData,
        }
    }

    /// Create a circuit for setup phase (without actual values)
    pub fn empty() -> Self {
        Self {
            y: None,
            x: None,
            _field: PhantomData,
        }
    }
}

impl<F: Field> ConstraintSynthesizer<F> for SquareRootCircuit<F> {
    /// Generate the R1CS constraints for the square root circuit
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate the public input variable y
        let y_var = cs.new_input_variable(|| {
            self.y.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate the private witness variable x
        let x_var = cs.new_witness_variable(|| {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce the constraint: x * x = y
        // This creates the constraint equation: x * x - y = 0
        // In R1CS form: (x) * (x) = (y)
        cs.enforce_constraint(
            lc!() + x_var,     // Left: x
            lc!() + x_var,     // Right: x  
            lc!() + y_var,     // Output: y
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_ff::Zero;

    #[test]
    fn test_square_root_circuit() {
        // Test with x = 3, y = 9
        let x = Fr::from(3u64);
        let y = Fr::from(9u64);
        
        let circuit = SquareRootCircuit::new(Some(y), Some(x));
        let cs = ConstraintSystem::new_ref();
        
        circuit.generate_constraints(cs.clone()).unwrap();
        
        // Check that the circuit is satisfied
        assert!(cs.is_satisfied().unwrap());
        
        // Check that we have the expected number of constraints
        assert_eq!(cs.num_constraints(), 1);
    }

    #[test]
    fn test_square_root_circuit_fails_with_wrong_input() {
        // Test with x = 3, but y = 10 (should be 9)
        let x = Fr::from(3u64);
        let y = Fr::from(10u64);
        
        let circuit = SquareRootCircuit::new(Some(y), Some(x));
        let cs = ConstraintSystem::new_ref();
        
        circuit.generate_constraints(cs.clone()).unwrap();
        
        // Check that the circuit is NOT satisfied
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_square_root_circuit_zero() {
        // Test with x = 0, y = 0
        let x = Fr::zero();
        let y = Fr::zero();
        
        let circuit = SquareRootCircuit::new(Some(y), Some(x));
        let cs = ConstraintSystem::new_ref();
        
        circuit.generate_constraints(cs.clone()).unwrap();
        
        // Check that the circuit is satisfied
        assert!(cs.is_satisfied().unwrap());
    }
}