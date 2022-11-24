use hex_literal::hex;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

use crate::maki_types::HashedLeaf;

const MERKLE_TREE_MAX_DEPTH: u8 = 32;

pub const MERKLE_TREE_DEFAULT_DEPTH: u8 = 24;

#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, SpreadAllocate, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug, StorageLayout))]
pub struct MerkleTree {
    tree_depth: u8,
    next_leaf_index: u128,
    root: [u8; 32],
}

#[derive(Debug, PartialEq)]
pub enum MerkleTreeError {
    InvalidTreeDepth,
    TreeIsFull,
}

impl MerkleTree {
    pub fn new(tree_depth: u8) -> Result<Self, MerkleTreeError> {
        if tree_depth <= 0 || tree_depth > MERKLE_TREE_MAX_DEPTH {
            return Err(MerkleTreeError::InvalidTreeDepth);
        }

        let zeros: [[u8; 32]; MERKLE_TREE_MAX_DEPTH];

        //TODO : change this value
        zeros[0] = hex!("12633468165168489165651891165198498451781651121684981651891465318");

        //TODO
        Ok(MerkleTree {
            tree_depth,
            next_leaf_index: 0,
            root: zeros[0],
        })
    }

    pub fn insert_leaf(&mut self, leaf: &HashedLeaf) -> Result<u128, MerkleTreeError> {
        if self.next_leaf_index >= 1 << self.tree_depth {
            return Err(MerkleTreeError::TreeIsFull);
        }

        return Ok(self.next_leaf_index);
    }
}

#[cfg(test)]
mod tests {}
