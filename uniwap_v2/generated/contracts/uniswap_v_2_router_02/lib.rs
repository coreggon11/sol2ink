#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_router_02 {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct UniswapV2Router02Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Router02 for UniswapV2Router02Contract {}

    impl IUniswapV2Router02 for UniswapV2Router02Contract {}

    impl UniswapV2Router02Contract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, weth: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.factory = factory;
            instance.data.weth = weth;
            instance
        }

    }
}
