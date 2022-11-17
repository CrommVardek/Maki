pub mod hasher {
    use crate::maki_objects::StateLeaf;
    use crate::maki_types::HashedLeaf;

    use dusk_bls12_381::BlsScalar;

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

        let bls_scalars: Vec<BlsScalar> = plain_leaf.iter().map(|i| bytes_to_scalar(*i)).collect();

        let result = dusk_poseidon::sponge::hash(&bls_scalars);

        let hashed_leaf = scalar_to_bytes(result);

        hashed_leaf
    }

    fn bytes_to_scalar(bytes: [u8; 32]) -> BlsScalar {
        BlsScalar(bytes_to_u64(bytes))
    }

    fn bytes_to_u64(bytes: [u8; 32]) -> [u64; 4] {
        let mut result = [0; 4];
    
        for i in 0..4 {
            let bytes_array = <&[u8; 8]>::try_from(&bytes[i * 8..(i + 1) * 8]).unwrap();
            result[i] = u64::from_be_bytes(*bytes_array);
        }
    
        result
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
}
