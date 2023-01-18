#![cfg_attr(not(feature = "std"), no_std)]

use ink_env;
use ink_lang as ink;

mod hasher;
mod maki_objects;
mod maki_types;
mod merkle_tree;

#[ink::contract]
mod maki {

    use crate::hasher::hasher::{hash_message, hash_state_leaf};
    use crate::maki_objects::{Message, StateLeaf};
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

        number_messages: u32,
    }

    /// Errors which may be returned from the smart contract
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        SignUpPeriodEnded,
        VotingPeriodEnded,
        MessageLimitReached,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// SignedUp event when a user signed up successfully
    #[ink(event)]
    pub struct SignedUp {
        user_public_key: PublicKey,
    }

    /// MessagePublished event when a user published a message successfully
    #[ink(event)]
    pub struct MessagePublished {
        message: Message,
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
                message_tree: MerkleTree::new(MERKLE_TREE_DEFAULT_DEPTH as u8).unwrap(),
                state_tree: MerkleTree::new(MERKLE_TREE_DEFAULT_DEPTH as u8).unwrap(),
                number_messages: 0,
            }
        }

        /// Sign Up can be called by any user whishing to cast a vote.
        /// ## Arguments
        ///
        /// * `user_public_key` - User's public key that will be used by the coordinator to decrypt commands (encrypted using a shared key)
        ///
        /// ## Returns
        #[ink(message)]
        pub fn sign_up(&mut self, user_public_key: PublicKey) -> Result<()> {
            let block_timestamp = self.env().block_timestamp();

            if self.contract_start_timestamp + u64::from(self.signup_duration_seconds) * 1000
                > block_timestamp
            {
                return Err(Error::SignUpPeriodEnded);
            }

            let state_leaf =
                StateLeaf::new(user_public_key, self.user_vote_credit, [0; 32], [0; 32]);

            let hashed_leaf = hash_state_leaf(&state_leaf);

            let result = self.state_tree.insert_leaf(hashed_leaf);

            if result.is_ok() {
                self.env().emit_event(SignedUp { user_public_key });
            }

            Ok(())
        }

        /// Publish message can be called by any user who signed up to cast a vote or change its public key.
        /// ## Arguments
        ///
        /// * `user_public_key` - User's public key (encrypted using a shared key)
        ///
        /// * `message` - User's (encrypted) message containing the command(s)
        ///
        /// ## Returns
        pub fn publish_message(
            &mut self,
            message: Message,
            user_public_key: PublicKey,
        ) -> Result<()> {
            if self.number_messages >= 2u32.pow(MERKLE_TREE_DEFAULT_DEPTH as u32) - 1 {
                return Err(Error::MessageLimitReached);
            }

            let block_timestamp = self.env().block_timestamp();

            if self.contract_start_timestamp
                + u64::from(self.signup_duration_seconds) * 1000
                + u64::from(self.vote_duration_seconds) * 1000
                > block_timestamp
            {
                return Err(Error::VotingPeriodEnded);
            }

            let leaf = hash_message(&message);

            let result = self.message_tree.insert_leaf(leaf);

            if result.is_ok() {
                self.number_messages += 1;

                self.env().emit_event(MessagePublished {
                    message,
                    user_public_key,
                });
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        // Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_env;
        use ink_lang as ink;

        type Event = <Maki as ::ink_lang::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn sign_up_emits_sign_up_event() {
            let mut maki = Maki::new(10000, 10000, [0; 32], 100);

            let upk = [1; 32];

            let result = maki.sign_up(upk);

            assert!(result.is_ok());

            let events = ink_env::test::recorded_events().collect::<Vec<_>>();

            let events_length = &events.len();

            assert_eq!(*events_length, 1);
            let sign_up_event = &events[0];
            let decoded_event = <Event as scale::Decode>::decode(&mut &sign_up_event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::SignedUp(SignedUp { user_public_key }) = decoded_event {
                assert_eq!(
                    user_public_key, upk,
                    "encountered invalid SignedUp.user_public_key"
                );
            } else {
                panic!("encountered unexpected event kind: expected a SignedUp event")
            }
        }

        #[ink::test]
        fn publish_message_emits_publish_message_event() {
            let mut maki = Maki::new(10000, 10000, [0; 32], 100);
            self.exec_context.block_timestamp += 10;

            let msg = Message::new([2; 32]);
            let upk = [1; 32];
            let result = maki.publish_message(msg, upk);

            assert!(result.is_ok());

            let events = ink_env::test::recorded_events().collect::<Vec<_>>();

            let events_length = &events.len();

            assert_eq!(*events_length, 1);
            let publish_message_event = &events[0];
            let decoded_event =
                <Event as scale::Decode>::decode(&mut &publish_message_event.data[..])
                    .expect("encountered invalid contract event data buffer");
            if let Event::MessagePublished(MessagePublished {
                message,
                user_public_key,
            }) = decoded_event
            {
                assert_eq!(
                    user_public_key, upk,
                    "encountered invalid MessagePublished.user_public_key"
                );
                assert_eq!(message, msg, "encountered invalid MessagePublished.message");
            } else {
                panic!("encountered unexpected event kind: expected a MessagePublished event")
            }
        }
    }
}
