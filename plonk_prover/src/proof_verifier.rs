
use dusk_bytes::Serializable;
use dusk_plonk::prelude::*;
use maki_shared::types::SerializedProof;

use crate::circuit::*;

// TODO : Add Public parameters and use accordingly
pub fn verify(
    public_parameters: &[u8],
    proof: &SerializedProof,
    f: JubJubAffine,
) -> Result<(), Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (_, verifier) =
        Compiler::compile::<MakiCircuit>(&pp, LABEL_TRANSCRIPT).expect("failed to compile circuit");

    // Proof deserialization
    let proof = Proof::from_bytes(proof)?;

    // Create public inputs
    let public_inputs: Vec<BlsScalar> = vec![f.get_x(), f.get_y()];

    verifier.verify(&proof, &public_inputs)
}
