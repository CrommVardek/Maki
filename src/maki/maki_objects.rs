#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};

use crate::maki_types::{PublicKey, VoteOptionTreeRoot};

#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, SpreadAllocate)]
#[cfg_attr(feature = "std", derive(Debug, StorageLayout))]
pub struct StateLeaf {
    pub public_key: PublicKey,
    pub voice_credit_balance: u16,
    pub vote_option_tree_root: VoteOptionTreeRoot,
    pub nounce: [u8; 32],
}

impl StateLeaf {
    pub fn new(
        public_key: PublicKey,
        voice_credit_balance: u16,
        vote_option_tree_root: VoteOptionTreeRoot,
        nounce: [u8; 32],
    ) -> StateLeaf {
        StateLeaf {
            public_key,
            voice_credit_balance,
            vote_option_tree_root,
            nounce,
        }
    }
}
