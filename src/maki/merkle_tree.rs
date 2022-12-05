use hex_literal::hex;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

use crate::maki_types::HashedLeaf;

const MERKLE_TREE_MAX_DEPTH: usize = 32;

pub const MERKLE_TREE_DEFAULT_DEPTH: usize = 24;

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
        if tree_depth <= 0 || usize::from(tree_depth) > MERKLE_TREE_MAX_DEPTH {
            return Err(MerkleTreeError::InvalidTreeDepth);
        }

        //TODO : populate zeros (Change 1 to 32 index values)
        let zeros: [[u8; 32]; MERKLE_TREE_MAX_DEPTH + 1] = [
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"), // BLAKE2b-256 Hash of "maki"
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
            hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"),
        ];

        Ok(MerkleTree {
            tree_depth,
            next_leaf_index: 0,
            root: zeros[usize::from(tree_depth)],
        })
    }

    pub fn insert_leaf(&mut self, leaf: &HashedLeaf) -> Result<u128, MerkleTreeError> {
        if self.next_leaf_index >= 1 << self.tree_depth {
            return Err(MerkleTreeError::TreeIsFull);
        }

        let current_leaf_index = self.next_leaf_index;

        for i in 0..self.tree_depth {
            let left;
            let right;

            if current_leaf_index % 2 == 0 {
                //TODO zeros outside of merkle_tree file
                right = zeros[i];
                left = leaf;

                //TODO subtree
            } else {
                //TODO subtree
                right = leaf;
            }

            //TODO change currenthash and use instead of leaf
            current_leaf_index /= 2;
        }
        // TODO

        return Ok(self.next_leaf_index);
    }
}

#[cfg(test)]
mod tests {}
