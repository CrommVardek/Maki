use maki_shared::types::SerializedProof;
use plonk_prover::verify;



pub fn verify_proof(proof: &SerializedProof,  pp: &[[u8; 32]]) -> bool {
    verify(pp, proof, f).map(|_| true)
    .unwrap_or(false)
}