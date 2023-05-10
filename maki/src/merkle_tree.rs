use crate::hasher::hasher::hash_left_right;
use hex_literal::hex;
#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;
use maki_shared::types::TreeRoot;

use crate::maki_types::HashedLeaf;

const MERKLE_TREE_MAX_DEPTH: usize = 32;

#[cfg(test)]
pub const MERKLE_TREE_DEFAULT_DEPTH: usize = 24;

const MERKLE_TREE_ZEROS: [[u8; 32]; MERKLE_TREE_MAX_DEPTH + 1] = [
    hex!("7b7fd692a95c21575ccf41a10c5fffa47e231e9b6503551fc397a2024e7d1b45"), // BLAKE2b-256 Hash of "maki"
    hex!("b6147a8cd8c6b3787b46234f3a2c7b97600079d9c61b00552c7bbba3e742fbb8"),
    hex!("eb2e77cbae50f3ee70f9cde778e8c425247f960b2ba4878e445ca67de0436296"),
    hex!("3e8ec789403271e32b46079812fc9ca4ac41028d7b71fa8c45655524a1e6b991"),
    hex!("725de40a39b430101bd9a6991b708fd1939f4d8c81dcf964417fc40505aad5d2"),
    hex!("840f5334075ea1407cb7bdc83c5a5181a5e6b09f4211d3253017212536ba2e78"),
    hex!("1cf104a27dc2700808404b54b37544b74890dd23135e315101771774c1edf206"),
    hex!("5696ffe94c052fb985d13c90b70e359673ac1c0e5968ead90b40ac0b08558022"),
    hex!("027013bd2beb5842480edbb208be94e05b82e071aedfa7c60e481662e5187e8e"),
    hex!("6274865509d2c90b8e0dc801a3774e7bd01780293671e79b1a783be1bc697778"),
    hex!("9ffa205b504ddf0d9bd4c61fc69fec9918116ad9e6e0561a6fb131b27a73fb51"),
    hex!("7ca83dd95c0119d9ebf97159a9f0c08250f4ae0720b0e12e365a3c78aabd7476"),
    hex!("1f36392825503141d49391e3059d88223cb6cc152743b58933ca60280a124ccd"),
    hex!("ededfc17fa461c1176d38e11a0a81bc99983722ac4c422741e599a34277189d7"),
    hex!("ae90d8bb755683f4f8a0efbf2766e2594a7d704bba660ed5535080d154ec397d"),
    hex!("05c8bc0a2b656015a1d8a313fcc868fc53a5affa0dd6d7b3449b14d8da6a3e09"),
    hex!("6acbd2fdf1f4494042c6ac1ae642ae9ecb70c8d4e39534235de8fe92d8406875"),
    hex!("18233d5e1babb0c8faa1bc48ef780657daa4caa0b0db469a19ae399c925d46af"),
    hex!("f5d5151bbeaf611f80c57afb51956224d5487d2d85c59cd62890cfea57991619"),
    hex!("b28bb4b2acc11e42e3245489a5badb867e4a8b8a01a048b00339736dc9a32dbf"),
    hex!("1640e5419648d7e1c73aac2878bcc7d959c9268c66d26a9b360db4976a4b72ee"),
    hex!("3ce95dd8a4931b136c78c2a9c7c985499197acd18c5dd19a589e72df6529fb84"),
    hex!("31c750a39f3cf8dffd1bacb42f0f094d664b4ca4a5d967c0581d1979502675a4"),
    hex!("d1066747a49c9e4ce0c5fb798d978e65d73f63bfd542ffe01941715ac46717d5"),
    hex!("bbfd8dbdc7c1775369473e139674cefae589abb2078500cf6f69db26c8b2a1b2"),
    hex!("bee3f9931bcddd77287493c45126df07b5f8b585593b9f516812af6c0e492092"),
    hex!("12b9b47543cb6da4406e7b28f3a0d14ec0f2393ffc8407280238d5bfb13c7cd0"),
    hex!("13cc72a87cf878ed7ada1f2f4da9fbb6b5a76451ccfbd294505f090c563037bb"),
    hex!("ae6db4aba7b5dd131e5c484c78a506299411edde24adfc0f06ecb48ffce8814c"),
    hex!("3f65cc50530d9e8270ea39293255e62d212fb5023ea885700c6d5e4c92c0fcb8"),
    hex!("78f8bbc4497fe45f495315ccd94e7b559f9dbd2bf81dfdae0787a54f6ea93022"),
    hex!("641bbc9668167e04bf60c515f84b8f61fd6cf60c60edeb9b0bab771843fe19fa"),
    hex!("ae91edaddca5e8e616541b5ea8809722a4aa3a65b748ed7a4b487229867b3dd0"),
];

#[derive(scale::Encode, scale::Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
pub struct MerkleTree {
    tree_depth: u8,
    next_leaf_index: u128,
    filled_subtrees: [[u8; 32]; MERKLE_TREE_MAX_DEPTH],
    root: TreeRoot,
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
        filled_subtrees.copy_from_slice(&zeros[0..32]);

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

        return Ok(self.next_leaf_index);
    }

    pub fn get_root(&self) -> [u8; 32] {
        self.root
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DEPTH: usize = 10;

    #[test]
    fn test_error_when_tree_is_full() {
        let mut tree = MerkleTree::new(TEST_DEPTH as u8).unwrap();

        for i in 0..2usize.pow(TEST_DEPTH as u32) {
            tree.insert_leaf([(i % (2usize.pow(8)))as u8; 32]).unwrap();
        }

        let err = tree.insert_leaf([2; 32]);

        assert_eq!(err, Err(MerkleTreeError::TreeIsFull));
    }

    #[test]
    fn test_error_when_tree_depth_exceeds_max() {
        let tree = MerkleTree::new((MERKLE_TREE_MAX_DEPTH+1) as u8);

        assert!(tree.is_err());
        assert_eq!(tree, Err(MerkleTreeError::InvalidTreeDepth));
    }

    #[test]
    fn test_empty_tree_root_is_first_zero_value() {
        let tree = MerkleTree::new(TEST_DEPTH as u8).unwrap();

        assert_eq!(tree.root, MERKLE_TREE_ZEROS[usize::from(TEST_DEPTH)]);
    }
}
