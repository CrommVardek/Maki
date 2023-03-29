#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;
use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
use ink::storage_item;

use crate::maki_types::{PublicKey, VoteOptionTreeRoot};

#[derive(scale::Encode, scale::Decode)]
#[ink::storage_item]
#[cfg_attr(feature = "std", derive(Debug))]
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
    ) -> Self {
        StateLeaf {
            public_key,
            voice_credit_balance,
            vote_option_tree_root,
            nounce,
        }
    }
}


// TODO see : https://github.com/727-Ventures/ink/tree/feature/storage-docs/examples/complex-storage-structures
#[derive(
    scale::Encode,
    scale::Decode,
    PartialEq,
    Clone,
    Copy,
)]
#[ink::storage_item]
#[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct Message {
    pub data: [u8; 32],
}

impl Message {
    pub fn new(data: [u8; 32]) -> Self {
        Message { data }
    }
}
