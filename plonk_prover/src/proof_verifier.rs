use dusk_bytes::Serializable;
use dusk_plonk::prelude::*;
use maki_shared::{types::{SerializedProof, TreeRoot, PublicKey}, functions_utils::bytes_to_scalar};

use crate::circuits::*;

// TODO : Add Public inputs and use accordingly
pub fn verifyVoteTally(
    public_parameters: &[u8],
    proof: &SerializedProof,
    f: JubJubAffine,
) -> Result<(), Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (_, verifier) = Compiler::compile::<MakiVoteTallyCircuit>(&pp, LABEL_TRANSCRIPT)
        .expect("failed to compile circuit");

    // Proof deserialization
    let proof = Proof::from_bytes(proof)?;

    // Create public inputs
    let public_inputs: Vec<BlsScalar> = vec![f.get_x(), f.get_y()];

    verifier.verify(&proof, &public_inputs)
}

pub fn verifyProcessMessage(
    public_parameters: &[u8],
    new_state_root: TreeRoot,
    public_key: PublicKey,
    ecdh_public_key: PublicKey,
    proof: &SerializedProof,
) -> Result<(), Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (_, verifier) = Compiler::compile::<MakiProcessMessageCircuit>(&pp, LABEL_TRANSCRIPT)
        .expect("failed to compile circuit");

    // Proof deserialization
    let proof = Proof::from_bytes(proof)?;

    // Create public inputs
    let public_inputs: Vec<BlsScalar> = vec![bytes_to_scalar(new_state_root), bytes_to_scalar(public_key), bytes_to_scalar(ecdh_public_key)];

    verifier.verify(&proof, &public_inputs)
}
