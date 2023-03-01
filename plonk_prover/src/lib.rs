#![cfg_attr(not(feature = "std"), no_std)]

mod circuit;
mod proof_verifier;
mod proof_prover;

pub mod proof_prover::{prove};