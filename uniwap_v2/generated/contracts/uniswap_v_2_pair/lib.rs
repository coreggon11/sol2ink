#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_pair {
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

    pub const MINIMUM_LIQUIDITY: u128 = 10.pow(3);
    pub const SELECTOR: [u8; 4] =
        <[u8; 4]>::from(keccak_256(Vec::<u8>::from("transfer(address,uint256)"))?);

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        sender: AccountId,
        amount_0: u128,
        amount_1: u128,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        sender: AccountId,
        amount_0: u128,
        amount_1: u128,
        #[ink(topic)]
        to: AccountId,
    }

    #[ink(event)]
    pub struct Swap {
        #[ink(topic)]
        sender: AccountId,
        amount_0_in: u128,
        amount_1_in: u128,
        amount_0_out: u128,
        amount_1_out: u128,
        #[ink(topic)]
        to: AccountId,
    }

    #[ink(event)]
    pub struct Sync {
        reserve_0: u128,
        reserve_1: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct UniswapV2PairContract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Pair for UniswapV2PairContract {}

    impl uniswap_v_2_pair::Internal for UniswapV2PairContract {
        fn _emit_mint(&self, sender: AccountId, amount_0: u128, amount_1: u128) {
            self.env().emit_event(Mint {
                sender,
                amount_0,
                amount_1,
            });
        }

        fn _emit_burn(&self, sender: AccountId, amount_0: u128, amount_1: u128, to: AccountId) {
            self.env().emit_event(Burn {
                sender,
                amount_0,
                amount_1,
                to,
            });
        }

        fn _emit_swap(
            &self,
            sender: AccountId,
            amount_0_in: u128,
            amount_1_in: u128,
            amount_0_out: u128,
            amount_1_out: u128,
            to: AccountId,
        ) {
            self.env().emit_event(Swap {
                sender,
                amount_0_in,
                amount_1_in,
                amount_0_out,
                amount_1_out,
                to,
            });
        }

        fn _emit_sync(&self, reserve_0: u128, reserve_1: u128) {
            self.env().emit_event(Sync {
                reserve_0,
                reserve_1,
            });
        }

    }

    impl IUniswapV2Pair for UniswapV2PairContract {}

    impl UniswapV2ERC20 for UniswapV2PairContract {}

    impl UniswapV2PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().factory = instance.env().caller();
                self.unlocked = 1;
            })
        }

    }
}
