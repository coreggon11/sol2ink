#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_migrator {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct UniswapV2MigratorContract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Migrator for UniswapV2MigratorContract {}

    impl IUniswapV2Migrator for UniswapV2MigratorContract {}

    impl UniswapV2MigratorContract {
        #[ink(constructor)]
        pub fn new(factory_v_1: AccountId, router: AccountId) -> Self {
            let mut instance = Self::default();
            instance.data.factory_v_1 = i_uniswap_v_1_factory(factory_v_1)?;
            instance.data.router = i_uniswap_v_2_router_01(router)?;
            instance
        }

    }
}
