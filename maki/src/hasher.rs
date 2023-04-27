pub mod hasher {
    use crate::maki_objects::{Message, StateLeaf};
    use crate::maki_types::HashedLeaf;

    use dusk_bls12_381::BlsScalar;

    use ink_prelude::vec::Vec;
    use maki_shared::functions_utils::bytes_to_scalar;

    pub fn hash_state_leaf(state_leaf: &StateLeaf) -> HashedLeaf {
        let mut plain_leaf_voice_credit: [u8; 32] = [0; 32];
        plain_leaf_voice_credit[0] = (state_leaf.voice_credit_balance >> 8) as u8;
        plain_leaf_voice_credit[1] = state_leaf.voice_credit_balance as u8;

        let plain_leaf: [[u8; 32]; 4] = [
            state_leaf.public_key,
            plain_leaf_voice_credit,
            state_leaf.vote_option_tree_root,
            state_leaf.nounce,
        ];

        poseidon_hash(&plain_leaf)
    }

    pub fn hash_left_right(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        poseidon_hash(&[*left, *right])
    }

    pub fn hash_message(message: &Message) -> HashedLeaf {
        poseidon_hash(&[message.data])
    }

    fn poseidon_hash(elements_to_hash: &[[u8; 32]]) -> [u8; 32] {
        let bls_scalars: Vec<BlsScalar> = elements_to_hash
            .iter()
            .map(|i| bytes_to_scalar(*i))
            .collect();

        let result = dusk_poseidon::sponge::hash(&bls_scalars);

        let hash_bytes = scalar_to_bytes(result);

        hash_bytes
    }

    fn scalar_to_bytes(scalar: BlsScalar) -> [u8; 32] {
        u64_to_bytes(*scalar.internal_repr())
    }

    fn u64_to_bytes(array: [u64; 4]) -> [u8; 32] {
        let mut result = [0; 32];

        for i in 0..array.len() {
            let bytes_array = array[i].to_be_bytes();
            for j in 0..bytes_array.len() {
                result[i * 8 + j] = bytes_array[j];
            }
        }

        result
    }

    #[test]
    fn u64_to_bytes_works_under_256() {
        let array: [u64; 4] = [10, 255, 0, 3];

        let result = u64_to_bytes(array);

        assert_eq!(
            [
                0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 3
            ],
            result
        )
    }

    #[test]
    fn u64_to_bytes_works_over_256() {
        let array: [u64; 4] = [196710, 257, 0, 899];

        let result = u64_to_bytes(array);

        assert_eq!(
            [
                0, 0, 0, 0, 0, 3, 0, 102, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 3, 131
            ],
            result
        )
    }
}
