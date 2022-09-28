
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout, StorageLayout};

#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, SpreadAllocate, StorageLayout)]
pub struct MerkleTree {

}    

impl MerkleTree {

    pub fn new() -> Self {
        MerkleTree {  }
    }

}