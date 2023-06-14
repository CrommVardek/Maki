#![cfg_attr(not(feature = "std"), no_std)]

mod circuits;
mod proof_verifier;
mod proof_prover;

pub use proof_prover::{proveVoteTally, proveProcessMessage};
pub use proof_verifier::{verifyVoteTally, verifyProcessMessage};

#[cfg(test)]
mod tests {
    #[test]
    fn verify_proof_process_message_success(){
        
    }
}