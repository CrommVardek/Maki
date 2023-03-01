use dusk_bytes::Serializable;
pub use dusk_plonk::prelude::Proof;

pub type SerializedProof = [u8; Proof::SIZE];