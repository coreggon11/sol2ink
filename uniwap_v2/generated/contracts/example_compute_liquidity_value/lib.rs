#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod example_compute_liquidity_value {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ExampleComputeLiquidityValueContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleComputeLiquidityValue for ExampleComputeLiquidityValueContract {}

    impl generated::impls::example_compute_liquidity_value::Internal
        for ExampleComputeLiquidityValueContract
    {
    }

    impl ExampleComputeLiquidityValueContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.factory = factory;
            })
        }

    }
}
