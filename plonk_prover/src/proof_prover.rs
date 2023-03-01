
use dusk_plonk::prelude::Compiler;
use rand_core::OsRng;

use crate::circuit::*;

pub fn prove(
    public_parameters: &[u8]) -> Result<SerializedProof, Error> {

    let (prover, verifier) = Compiler::compile::<MakiCircuit>(public_parameters, LABEL_TRANSCRIPT).expect("failed to compile circuit");

    // Generate the proof and its public inputs
    let (proof, public_inputs) = prover
        .prove(&mut OsRng, &MakiCircuit::default())
        .expect("failed to prove");

    return proof.to_bytes();    
}