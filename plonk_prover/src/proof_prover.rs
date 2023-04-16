use dusk_bytes::Serializable;
use dusk_plonk::prelude::*;
use maki_shared::types::SerializedProof;
use rand_core::OsRng;

use crate::circuit::*;

pub fn prove(
    public_parameters: &[u8],
    // private inputs
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    e: JubJubScalar, 
    // public inputs
    f: JubJubAffine,
) -> Result<SerializedProof, Error> {
    //Read public parameters
    let pp = PublicParameters::from_slice(public_parameters)?;

    let (prover, _) =
        Compiler::compile::<MakiCircuit>(&pp, LABEL_TRANSCRIPT).expect("failed to compile circuit");

    let circuit = MakiCircuit { a, b, c, d, e, f };

    // Generate the proof and its public inputs
    let (proof, _) = prover.prove(&mut OsRng, &circuit).expect("Failed to prove");

    return Ok(proof.to_bytes());
}
