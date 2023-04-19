use maki_shared::types::{PublicKey as SharedTypePublicKey};

pub type PublicKey = SharedTypePublicKey;

pub type HashedLeaf = [u8; 32];

pub type VoteOptionTreeRoot = [u8; 32];
