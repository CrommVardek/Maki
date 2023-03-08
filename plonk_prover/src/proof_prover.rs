use dusk_bytes::Serializable;
use dusk_plonk::prelude::*;
use maki_shared::types::SerializedProof;
use rand_core::OsRng;

use crate::circuit::*;

pub fn prove(public_parameters: &[u8]) -> Result<SerializedProof, Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (prover, verifier) = Compiler::compile::<MakiCircuit>(&pp, LABEL_TRANSCRIPT)
        .expect("failed to compile circuit");

    // Generate the proof and its public inputs
    let (proof, public_inputs) = prover
        .prove(&mut OsRng, &MakiCircuit::default())
        .expect("Failed to prove");

    return Ok(proof.to_bytes());
}
