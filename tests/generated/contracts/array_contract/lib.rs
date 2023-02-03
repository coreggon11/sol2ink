#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.7.0) (token/ERC1155/ERC1155.sol)
#[openbrush::contract]
pub mod array_contract {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ArrayContractContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ArrayContract for ArrayContractContract {}

    impl generated::impls::array_contract::Internal for ArrayContractContract {}

    impl ArrayContractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

    }
}
