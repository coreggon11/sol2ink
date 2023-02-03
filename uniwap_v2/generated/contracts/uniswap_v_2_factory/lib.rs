#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_factory {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(event)]
    pub struct PairCreated {
        #[ink(topic)]
        token_0: AccountId,
        #[ink(topic)]
        token_1: AccountId,
        pair: AccountId,
        anonymous: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct UniswapV2FactoryContract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Factory for UniswapV2FactoryContract {}

    impl generated::impls::uniswap_v_2_factory::Internal for UniswapV2FactoryContract {
        fn _emit_pair_created(
            &self,
            token_0: AccountId,
            token_1: AccountId,
            pair: AccountId,
            anonymous: u128,
        ) {
            self.env().emit_event(PairCreated {
                token_0,
                token_1,
                pair,
                anonymous,
            });
        }

    }

    impl IUniswapV2Factory for UniswapV2FactoryContract {}

    impl UniswapV2FactoryContract {
        #[ink(constructor)]
        pub fn new(fee_to_setter: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.fee_to_setter = fee_to_setter;
            })
        }

    }
}
