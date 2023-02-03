#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_migrator {
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
    pub struct UniswapV2MigratorContract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Migrator for UniswapV2MigratorContract {}

    impl uniswap_v_2_migrator::Internal for UniswapV2MigratorContract {}

    impl IUniswapV2Migrator for UniswapV2MigratorContract {}

    impl UniswapV2MigratorContract {
        #[ink(constructor)]
        pub fn new(factory_v_1: AccountId, router: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().factory_v_1 = i_uniswap_v_1_factory(factory_v_1)?;
                instance.data().router = i_uniswap_v_2_router_01(router)?;
            })
        }

    }
}
