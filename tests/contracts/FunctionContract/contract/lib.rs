#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod function_contract {
    use function_contract::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        AccountId,
        AccountIdExt,
        Storage,
        String,
        ZERO_ADDRESS,
    };
    use scale::{
        Decode,
        Encode,
    };


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FunctionContractContract {
        #[storage_field]
        data: impls::Data,
    }

    impl FunctionContract for FunctionContractContract {}

    impl function_contract::Internal for FunctionContractContract {}

    impl FunctionContractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().owner = instance.env().caller();
                self.x = 10;
            })
        }

    }
}
