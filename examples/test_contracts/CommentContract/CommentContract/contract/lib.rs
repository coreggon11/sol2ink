#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

#[openbrush::contract]
pub mod comment_contract {
    use comment_contract::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        storage::Mapping,
        traits::{
            AccountId,
            Storage,
            String,
        },
    };
    use scale::{
        Decode,
        Encode,
    };


    #[ink(event)]
    pub struct Log {
        /// sender comment
        #[ink(topic)]
        sender: AccountId,
        /// message comment
        message: String,
        /// priority comment1
        /// priority comment2
        priority: u8,
        status: Status,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct CommentContractContract {
        #[storage_field]
        data: impls::Data,
    }

    impl CommentContract for CommentContractContract {}

    impl comment_contract::Internal for CommentContractContract {
        fn _emit_log(&self, sender: AccountId, message: String, priority: u8, status: Status) {
            self.env().emit_event(Log {
                sender,
                message,
                priority,
                status,
            });
        }

    }

    impl CommentContractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

    }
}
