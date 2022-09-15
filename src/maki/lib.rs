#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

mod merkle_tree;

#[ink::contract]
mod maki {
    
    use super::*;
    use crate::merkle_tree::MerkleTree;

    #[ink(storage)]
    pub struct Maki {
        messageTree: MerkleTree,
        stateTree: MerkleTree,
    }

    impl Maki {        
        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                messageTree: MerkleTree::new(), 
                stateTree: MerkleTree::new(),
            }
        }

        /// Sign Up can be called by any user whishing to cast a vote.
        #[ink(message)]
        pub fn sign_up(&mut self) {
            self.value = !self.value;
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
