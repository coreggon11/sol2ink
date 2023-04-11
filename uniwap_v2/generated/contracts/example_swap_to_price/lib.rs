#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod example_swap_to_price {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ExampleSwapToPriceContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleSwapToPrice for ExampleSwapToPriceContract {}

    impl ExampleSwapToPriceContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, router: IUniswapV2Router01) -> Self {
            let mut instance = Self::default();
            instance.data.factory = factory;
            instance.data.router = router;
            instance
        }

    }
}
