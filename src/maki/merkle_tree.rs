use crate::hasher::hasher::hash_left_right;
use hex_literal::hex;
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

use crate::maki_types::HashedLeaf;

const MERKLE_TREE_MAX_DEPTH: usize = 32;

pub const MERKLE_TREE_DEFAULT_DEPTH: usize = 24;

//TODO : populate zeros (Change 1 to 32 index values)
const MERKLE_TREE_ZEROS: [[u8; 32]; MERKLE_TREE_MAX_DEPTH + 1] = [
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

#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, SpreadAllocate, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug, StorageLayout))]
pub struct MerkleTree {
    tree_depth: u8,
    next_leaf_index: u128,
    filled_subtrees: [[u8; 32]; MERKLE_TREE_MAX_DEPTH],
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

        let zeros = MERKLE_TREE_ZEROS;

        // pre-fill subtree with zeros.
        let mut filled_subtrees = [[Default::default(); 32]; MERKLE_TREE_MAX_DEPTH];
        filled_subtrees.copy_from_slice(&zeros[0..usize::from(tree_depth)]);

        Ok(MerkleTree {
            tree_depth,
            next_leaf_index: 0,
            filled_subtrees,
            root: zeros[usize::from(tree_depth)],
        })
    }

    pub fn insert_leaf(&mut self, leaf: HashedLeaf) -> Result<u128, MerkleTreeError> {
        if self.next_leaf_index >= 1 << self.tree_depth {
            return Err(MerkleTreeError::TreeIsFull);
        }

        let mut current_leaf_index = self.next_leaf_index;
        let mut current_level_hash = leaf;

        for i in 0..self.tree_depth {
            let index = usize::from(i);
            let left;
            let right;

            if current_leaf_index % 2 == 0 {
                right = MERKLE_TREE_ZEROS[index];
                left = current_level_hash;

                self.filled_subtrees[index] = current_level_hash;
            } else {
                left = self.filled_subtrees[index];
                right = current_level_hash;
            }

            current_leaf_index /= 2;
            current_level_hash = hash_left_right(&left, &right);
        }

        self.root = current_level_hash;

        self.next_leaf_index += 1;

        // TODO

        return Ok(self.next_leaf_index);
    }
}

#[cfg(test)]
mod tests {}
