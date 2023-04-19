use dusk_bytes::Serializable;
use dusk_plonk::prelude::*;
use dusk_poseidon::sponge;
use maki_shared::{types::{SerializedProof, PublicKey, TreeRoot}, functions_utils::bytes_to_scalar};
use rand_core::OsRng;

use crate::circuit::*;

pub fn prove(
    public_parameters: &[u8],
    // private inputs
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    private_key: [u8; 32],
    // public inputs
    new_state_root: TreeRoot,
    public_key: PublicKey,
) -> Result<SerializedProof, Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (prover, _) =
        Compiler::compile::<MakiCircuit>(&pp, LABEL_TRANSCRIPT).expect("failed to compile circuit");

    let circuit = MakiCircuit { a, b, c, d, hashed_private_key: sponge::hash(&[bytes_to_scalar(private_key)]), public_key, new_state_root };

    // Generate the proof and its public inputs
    let (proof, _) = prover.prove(&mut OsRng, &circuit).expect("Failed to prove");

    return Ok(proof.to_bytes());
}
