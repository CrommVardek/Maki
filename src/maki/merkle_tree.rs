
#[cfg(feature = "std")]
use ink_storage::traits::{StorageLayout};
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout };

const MERKLE_TREE_MAX_DEPTH: u8 = 32;

pub const MERKLE_TREE_DEFAULT_DEPTH: u8 = 24;

#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, SpreadAllocate)]
#[cfg_attr(feature = "std", derive(Debug, StorageLayout))]
pub struct MerkleTree {
    tree_depth: u8,
}    

pub enum MerkleTreeError {
    InvalidTreeDepth,
}

impl MerkleTree {

    pub fn new(tree_depth: u8) -> Result<Self, MerkleTreeError> {
        if(tree_depth <= 0 || tree_depth > MERKLE_TREE_MAX_DEPTH) {
            return Err(MerkleTreeError::InvalidTreeDepth); 
        }
        Ok(MerkleTree { tree_depth })
    }

}