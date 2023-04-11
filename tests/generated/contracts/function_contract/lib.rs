#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod function_contract {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct FunctionContractContract {
        #[storage_field]
        data: impls::Data,
    }

    impl FunctionContract for FunctionContractContract {}

    impl FunctionContractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance.data.owner = instance.env().caller();
            self.x = 10;
            instance
        }

    }
}
