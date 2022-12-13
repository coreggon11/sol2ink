#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

#[openbrush::contract]
pub mod primitives {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        AccountId,
        Storage,
    };
    use primitives::*;
    use scale::{
        Decode,
        Encode,
    };


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct primitivesContract {
        #[storage_field]
        data: impls::Data,
    }

    impl primitives for primitivesContract {}

    impl primitives::Internal for primitivesContract {}

    impl primitivesContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

    }
}
