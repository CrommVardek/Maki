#![cfg_attr(not(feature = "std"), no_std)]

mod hasher;
mod maki_objects;
mod maki_types;
mod merkle_tree;
mod snark_verifier;

#[ink::contract]
pub mod maki {

    use maki_shared::functions_utils::generate_public_parameters;
    use maki_shared::types::SerializedProof;

    use crate::hasher::hasher::{hash_message, hash_state_leaf};
    use crate::maki_objects::{Message, StateLeaf};
    use crate::maki_types::PublicKey;
    use crate::merkle_tree::MerkleTree;
    use crate::snark_verifier::verify_proof_process_message;

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

        // Merkle Root of the state_tree
        state_root: [u8; 32],

        tree_depth: u8,

        number_messages: u32,
    }

    /// Errors which may be returned from the smart contract
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        SignUpPeriodEnded,
        VotingPeriodEnded,
        MessageLimitReached,
        VotingPeriodNotEnded,
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
        ecdh_public_key: PublicKey,
    }

    impl Maki {
        #[ink(constructor)]
        pub fn new(
            signup_duration_seconds: u32,
            vote_duration_seconds: u32,
            coordinator_public_key: PublicKey,
            user_vote_credit: u16,
            tree_depth: u8,
        ) -> Self {
            let state_merkle_tree = MerkleTree::new(tree_depth).unwrap();
            Self {
                signup_duration_seconds,
                vote_duration_seconds,
                coordinator_public_key,
                user_vote_credit,
                contract_start_timestamp: Self::env().block_timestamp(),
                message_tree: MerkleTree::new(tree_depth).unwrap(),
                state_root: state_merkle_tree.get_root(),
                state_tree: state_merkle_tree,
                number_messages: 0,
                tree_depth,
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
                < block_timestamp
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
        /// * `ecdh_public_key` - An ephemeral public key that can be combined 
        /// with the coordinator's private key to generate an ECDH shared key which was used to encrypt the message.
        ///
        /// * `message` - User's (encrypted) message containing the command(s)
        ///
        /// ## Returns
        pub fn publish_message(
            &mut self,
            message: Message,
            ecdh_public_key: PublicKey,
        ) -> Result<()> {
            if self.number_messages >= 2u32.pow(self.tree_depth as u32) - 1 {
                return Err(Error::MessageLimitReached);
            }

            let block_timestamp = self.env().block_timestamp();

            if self.contract_start_timestamp
                + u64::from(self.signup_duration_seconds) * 1000
                + u64::from(self.vote_duration_seconds) * 1000
                < block_timestamp
            {
                return Err(Error::VotingPeriodEnded);
            }

            let leaf = hash_message(&message);

            let result = self.message_tree.insert_leaf(leaf);

            if result.is_ok() {
                self.number_messages += 1;

                self.env().emit_event(MessagePublished {
                    message,
                    ecdh_public_key,
                });
            }

            Ok(())
        }

        /// Process messages can be called by any user, however, only the coordinator is supposed to be able to decrypt the message.
        /// Therefore the coordinator is the only user that will be able to provide a verified proof.
        /// ## Arguments
        ///
        /// * `proof` - The zk-SNARK proof
        ///
        /// ## Returns
        pub fn process_messages(
            &mut self,
            proof: SerializedProof,
            public_key: PublicKey,
        ) -> Result<()> {
            let block_timestamp = self.env().block_timestamp();

            if self.contract_start_timestamp
                + u64::from(self.signup_duration_seconds) * 1000
                + u64::from(self.vote_duration_seconds) * 1000
                >= block_timestamp
            {
                return Err(Error::VotingPeriodNotEnded);
            }

            // TODO

            let public_parameters = generate_public_parameters(
                &self.state_tree.get_root(),
                &public_key,
                &self.coordinator_public_key,
            );

            let proved = verify_proof_process_message(&proof, &public_parameters);

            if proved {
                self.state_root = self.state_tree.get_root();
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        // Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink_env;

        use crate::merkle_tree::MERKLE_TREE_DEFAULT_DEPTH;

        type Event = <Maki as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn sign_up_emits_sign_up_event() {
            let mut maki = Maki::new(10000, 10000, [0; 32], 100, MERKLE_TREE_DEFAULT_DEPTH as u8);

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
        fn sign_up_after_end_of_sign_up_period_returns_error() {
            let signup_duration: u32 = 60;
            let vote_duration: u32 = 10000;
            let mut maki = Maki::new(
                signup_duration,
                vote_duration,
                [0; 32],
                100,
                MERKLE_TREE_DEFAULT_DEPTH as u8,
            );

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(u64::from(
                signup_duration * 1000 + 1,
            ));

            let upk = [1; 32];

            let result = maki.sign_up(upk);

            assert!(result.is_err());
            assert_eq!(result, Err(Error::SignUpPeriodEnded));
        }

        #[ink::test]
        fn publish_message_returns_error_on_number_of_message_limit_reached() {
            let test_tree_depth: u8 = 8;

            let mut maki = Maki::new(60, 10000, [0; 32], 100, test_tree_depth);

            let msg = Message::new([2; 32]);
            let upk = [1; 32];
            for _ in 0..2usize.pow(test_tree_depth as u32) - 1 {
                maki.publish_message(msg, upk).unwrap();
            }

            let err = maki.publish_message(msg, upk);

            assert!(err.is_err());
            assert_eq!(err, Err(Error::MessageLimitReached));
        }

        #[ink::test]
        fn publish_message_emits_publish_message_event() {
            let mut maki = Maki::new(10000, 10000, [0; 32], 100, MERKLE_TREE_DEFAULT_DEPTH as u8);

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
                ecdh_public_key,
            }) = decoded_event
            {
                assert_eq!(
                    ecdh_public_key, upk,
                    "encountered invalid MessagePublished.user_public_key"
                );
                assert_eq!(message, msg, "encountered invalid MessagePublished.message");
            } else {
                panic!("encountered unexpected event kind: expected a MessagePublished event")
            }
        }

        #[ink::test]
        fn publish_message_after_end_of_voting_period_returns_error() {
            let signup_duration: u32 = 60;
            let vote_duration: u32 = 60;
            let mut maki = Maki::new(
                signup_duration,
                vote_duration,
                [0; 32],
                100,
                MERKLE_TREE_DEFAULT_DEPTH as u8,
            );

            ink_env::test::set_block_timestamp::<ink_env::DefaultEnvironment>(u64::from(
                (signup_duration + vote_duration) * 1000 + 1,
            ));

            let msg = Message::new([2; 32]);
            let upk = [1; 32];

            let result = maki.publish_message(msg, upk);

            assert!(result.is_err());
            assert_eq!(result, Err(Error::VotingPeriodEnded));
        }

        #[ink::test]
        fn process_messages_before_voting_period_ends_returns_error() {
            let mut maki = Maki::new(60, 60, [0; 32], 100, MERKLE_TREE_DEFAULT_DEPTH as u8);

            let proof = [123; 1040];
            let pk: PublicKey = [2;32];
            let result = maki.process_messages(proof, pk);

            assert!(result.is_err());
            assert_eq!(result, Err(Error::VotingPeriodNotEnded));
        }
    }
}
