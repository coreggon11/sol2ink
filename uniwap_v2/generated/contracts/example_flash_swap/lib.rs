#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod example_flash_swap {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ExampleFlashSwapContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleFlashSwap for ExampleFlashSwapContract {}

    impl generated::impls::example_flash_swap::Internal for ExampleFlashSwapContract {}

    impl IUniswapV2Callee for ExampleFlashSwapContract {}

    impl ExampleFlashSwapContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, factory_v_1: AccountId, router: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.factory_v_1 = i_uniswap_v_1_factory(factory_v_1)?;
                instance.data.factory = factory;
                instance.data.weth = iweth(i_uniswap_v_2_router_01(router)?.weth()?)?;
            })
        }

    }
}
