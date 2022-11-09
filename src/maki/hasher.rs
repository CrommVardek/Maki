pub mod hasher {
    use crate::maki_objects::StateLeaf;
    use crate::maki_types::HashedLeaf;

    //TODO
    pub fn hash_state_leaf(state_leaf: &StateLeaf) -> HashedLeaf {
        let mut plainLeafVoiceCredit: [u8; 32] = [0; 32];
        plainLeafVoiceCredit[0] = (state_leaf.voice_credit_balance >> 8) as u8;
        plainLeafVoiceCredit[1] = state_leaf.voice_credit_balance as u8;

        let plainLeaf: [[u8; 32]; 4] = [
            state_leaf.public_key,
            plainLeafVoiceCredit,
            state_leaf.vote_option_tree_root,
            state_leaf.nounce,
        ];

        [0; 32]
    }
}
