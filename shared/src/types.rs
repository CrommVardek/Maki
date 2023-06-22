use dusk_bytes::Serializable;
pub use dusk_plonk::prelude::Proof;

pub type PoseidonHash = [u8; 32];
pub type SerializedProof = [u8; Proof::SIZE];

//TODO : change this to struct that is composed of two fields X and Y
pub type PublicKey = [u8; 32];

pub type PrivateKey = [u8; 32];

pub type TreeRoot = [u8; 32];