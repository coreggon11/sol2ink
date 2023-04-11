#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod example_flash_swap {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ExampleFlashSwapContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleFlashSwap for ExampleFlashSwapContract {}

    impl IUniswapV2Callee for ExampleFlashSwapContract {}

    impl ExampleFlashSwapContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, factory_v_1: AccountId, router: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.factory_v_1 = i_uniswap_v_1_factory(factory_v_1)?;
            instance.data.factory = factory;
            instance.data.weth = iweth(i_uniswap_v_2_router_01(router)?.weth()?)?;
            instance
        }

    }
}
