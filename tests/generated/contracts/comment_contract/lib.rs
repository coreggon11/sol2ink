#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod comment_contract {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::*;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        storage::Mapping,
        traits::{
            AccountId,
            AccountIdExt,
            Storage,
            String,
            ZERO_ADDRESS,
        },
    };
    use scale::{
        Decode,
        Encode,
    };


    ///sender comment
    /// message comment
    ///priority comment1
    ///priority comment2
    #[ink(event)]
    pub struct Log {
        #[ink(topic)]
        sender: AccountId,
        message: String,
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
