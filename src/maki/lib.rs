#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

mod maki_objects;
mod maki_types;
mod merkle_tree;

#[ink::contract]
mod maki {

    use crate::maki_objects::StateLeaf;
    use crate::maki_types::PublicKey;
    use crate::merkle_tree::{MerkleTree, MERKLE_TREE_DEFAULT_DEPTH};

    #[ink(storage)]
    pub struct Maki {
        // Use to determine if a user can still sign-up/vote
        contract_start_timestamp: Timestamp,
        signup_duration_seconds: u32,
        vote_duration_seconds: u32,

        coordinator_public_key: PublicKey,

        user_vote_credit: u16,

        // State
        message_tree: MerkleTree,
        state_tree: MerkleTree,
    }

    /// SignedUp event when a user signed up successfully
    #[ink(event)]
    pub struct SignedUp {
        user_public_key: PublicKey,
    }

    impl Maki {
        #[ink(constructor)]
        pub fn new(
            signup_duration_seconds: u32,
            vote_duration_seconds: u32,
            coordinator_public_key: PublicKey,
            user_vote_credit: u16,
        ) -> Self {
            Self {
                signup_duration_seconds,
                vote_duration_seconds,
                coordinator_public_key,
                user_vote_credit,
                contract_start_timestamp: Self::env().block_timestamp(),
                message_tree: MerkleTree::new(MERKLE_TREE_DEFAULT_DEPTH).unwrap(),
                state_tree: MerkleTree::new(MERKLE_TREE_DEFAULT_DEPTH).unwrap(),
            }
        }

        /// Sign Up can be called by any user whishing to cast a vote.
        /// /// # Arguments
        ///
        /// * `user_public_key` - User's public key that will be used by the coordinator to decrypt commands (encrypted using a shared key)
        #[ink(message)]
        pub fn sign_up(&mut self, user_public_key: PublicKey) {
            self.env().emit_event(SignedUp { user_public_key });

            let state_leaf = StateLeaf {
                public_key: user_public_key,
                voice_credit_balance: self.user_vote_credit,
                nounce: [0; 32],

            };
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        // Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        // We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let maki = Maki::default();
        //     assert_eq!(maki.get(), false);
        // }

        // We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut maki = Maki::new(false);
        //     assert_eq!(maki.get(), false);
        //     maki.flip();
        //     assert_eq!(maki.get(), true);
        // }
    }
}
