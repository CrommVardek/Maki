#![cfg_attr(not(feature = "std"), no_std)]

mod circuit;
mod proof_verifier;
mod proof_prover;

pub use proof_prover::{prove};
pub use proof_verifier::{verify};