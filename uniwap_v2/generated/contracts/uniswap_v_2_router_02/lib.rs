#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_router_02 {
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
    pub struct UniswapV2Router02Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Router02 for UniswapV2Router02Contract {}

    impl uniswap_v_2_router_02::Internal for UniswapV2Router02Contract {}

    impl IUniswapV2Router02 for UniswapV2Router02Contract {}

    impl UniswapV2Router02Contract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, weth: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().factory = factory;
                instance.data().weth = weth;
            })
        }

    }
}
