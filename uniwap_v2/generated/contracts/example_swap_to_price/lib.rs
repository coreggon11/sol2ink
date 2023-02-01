#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod example_swap_to_price {
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


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ExampleSwapToPriceContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleSwapToPrice for ExampleSwapToPriceContract {}

    impl example_swap_to_price::Internal for ExampleSwapToPriceContract {}

    impl ExampleSwapToPriceContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, router: IUniswapV2Router01) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().factory = factory;
                instance.data().router = router;
            })
        }

    }
}
