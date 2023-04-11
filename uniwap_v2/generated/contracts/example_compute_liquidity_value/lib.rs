#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod example_compute_liquidity_value {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ExampleComputeLiquidityValueContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleComputeLiquidityValue for ExampleComputeLiquidityValueContract {}

    impl ExampleComputeLiquidityValueContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.factory = factory;
            instance
        }

    }
}
