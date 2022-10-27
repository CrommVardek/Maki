#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env;

mod maki_objects;
mod maki_types;
mod merkle_tree;
mod hasher;

#[ink::contract]
mod maki {

    use crate::maki_objects::StateLeaf;
    use crate::maki_types::PublicKey;
    use crate::merkle_tree::{MerkleTree, MERKLE_TREE_DEFAULT_DEPTH};
    use crate::hasher::hasher::hash_state_leaf;

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

            let state_leaf =
                StateLeaf::new(user_public_key, self.user_vote_credit, [0; 32], [0; 32]);

            let hashed_leaf = hash_state_leaf(state_leaf);
        }
    }

    #[cfg(test)]
    mod tests {
        // Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        use ink_env;
        
        type Event = <Maki as ::ink_lang::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn sign_up_emits_sign_up_event() {
            let mut maki = Maki::new(1000,1000, [0; 32], 100);

            let upk = [1; 32];
            maki.sign_up(upk);

            let events = ink_env::test::recorded_events().collect::<Vec<_>>();

            let events_length = &events.len();

            assert_eq!(*events_length, 1);
            let sign_up_event = &events[0];
            let decoded_event = <Event as scale::Decode>::decode(&mut &sign_up_event.data[..]).expect("encountered invalid contract event data buffer");
            if let Event::SignedUp(SignedUp { user_public_key }) = decoded_event {
                assert_eq!(user_public_key, upk, "encountered invalid SignedUp.user_public_key");
            } else {
                panic!("encountered unexpected event kind: expected a SignedUp event")
            }
        }
    }
}
