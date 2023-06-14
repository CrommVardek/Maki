use maki_shared::types::SerializedProof;
use plonk_prover::{verifyProcessMessage, verifyVoteTally};

pub fn verify_proof_process_message(proof: &SerializedProof, pp: &[u8]) -> bool {
    // TODO
    verifyProcessMessage(pp, proof, f)
        .map(|_| true)
        .unwrap_or(false)
}

pub fn verify_proof_vote_tally(proof: &SerializedProof, pp: &[u8]) -> bool {
    // TODO
    verifyVoteTally(pp, proof, f)
        .map(|_| true)
        .unwrap_or(false)
}
