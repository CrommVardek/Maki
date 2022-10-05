#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

use merkle_tree::MERKLE_TREE_DEFAULT_DEPTH;

mod merkle_tree;
mod maki_types;

#[ink::contract]
mod maki {
    
    use crate::merkle_tree::MerkleTree;
    use crate::maki_types::PublicKey;

    #[ink(storage)]
    pub struct Maki {

        // Use to determine if a user can still sign-up/vote
        contract_start_timestamp: Timestamp,
        signup_duration_seconds: u32,
        vote_duration_seconds: u32,

        // State
        message_tree: MerkleTree,
        state_tree: MerkleTree,
    }

    /// SignedUp event when a user signed up successfully
    #[ink(event)]
    pub struct SignedUp {

    }

    impl Maki {       
        
        #[ink(constructor)]
        pub fn new(signup_duration_seconds: u32, vote_duration_seconds: u32) -> Self {
            Self { 
                    signup_duration_seconds,
                    vote_duration_seconds,
                    contract_start_timestamp: Self::env().block_timestamp(),
                    message_tree: MerkleTree::new(MERKLE_TREE_DEFAULT_DEPTH), 
                    state_tree: MerkleTree::new(MERKLE_TREE_DEFAULT_DEPTH),
            }            
        }

        /// Sign Up can be called by any user whishing to cast a vote.
        /// /// # Arguments
        ///
        /// * `user_public_key` - User's public key that will be used by the coordinator to decrypt commands (encrypted using a shared key)
        #[ink(message)]
        pub fn sign_up(&mut self, user_public_key : PublicKey) {
            
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
