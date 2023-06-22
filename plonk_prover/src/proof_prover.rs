use dusk_bytes::Serializable;
use dusk_plonk::prelude::*;
use dusk_poseidon::sponge;
use maki_shared::{
    functions_utils::bytes_to_scalar,
    types::{PublicKey, SerializedProof, TreeRoot, PrivateKey},
};
use rand_core::OsRng;

use crate::circuits::*;

pub fn proveVoteTally(
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

    let (prover, _) = Compiler::compile::<MakiVoteTallyCircuit>(&pp, LABEL_TRANSCRIPT)
        .expect("failed to compile circuit");

    let circuit = MakiVoteTallyCircuit {
        a,
        b,
        c,
        d,
        hashed_private_key: sponge::hash(&[bytes_to_scalar(private_key)]),
        new_state_root: sponge::hash(&[bytes_to_scalar(new_state_root)]),
        public_key: sponge::hash(&[bytes_to_scalar(public_key)]),
    };

    // Generate the proof and its public inputs
    let (proof, _) = prover.prove(&mut OsRng, &circuit).expect("Failed to prove");

    return Ok(proof.to_bytes());
}

//TODO
pub fn proveProcessMessage(
    public_parameters: &[u8],
    // private inputs
    ecdh_private_key: PrivateKey,
    private_key: PrivateKey,
    // public inputs
    new_state_root: TreeRoot,
    public_key: PublicKey,
    ecdh_public_key: PublicKey,
) -> Result<SerializedProof, Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (prover, _) = Compiler::compile::<MakiProcessMessageCircuit>(&pp, LABEL_TRANSCRIPT)
        .expect("failed to compile circuit");

    let circuit: MakiProcessMessageCircuit = MakiProcessMessageCircuit {
        hashed_private_key: sponge::hash(&[bytes_to_scalar(private_key)]),
        new_state_root: bytes_to_scalar(new_state_root),
        public_key: bytes_to_scalar(public_key),
        ecdh_public_key: bytes_to_scalar(ecdh_public_key),
        ecdh_private_key: bytes_to_scalar(ecdh_private_key),
    };

    // Generate the proof and its public inputs
    let (proof, _) = prover.prove(&mut OsRng, &circuit).expect("Failed to prove");

    return Ok(proof.to_bytes());
}
