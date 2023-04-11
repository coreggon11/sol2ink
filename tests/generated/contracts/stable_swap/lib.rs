#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod stable_swap {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;

    /// Number of tokens
    pub const N: u128 = 3;
    /// Amplification coefficient multiplied by N^(N - 1)
    /// Higher value makes the curve more flat
    /// Lower value makes the curve more like constant product AMM
    pub const A: u128 = 1000 * (N.pow((N - 1)));
    /// 0.03%
    pub const SWAP_FEE: u128 = 300;
    /// Liquidity fee is derived from 2 constraints
    /// 1. Fee is 0 for adding / removing liquidity that results in a balanced pool
    /// 2. Swapping in a balanced pool is like adding and then removing liquidity
    ///    from a balanced pool
    /// swap fee = add liquidity fee + remove liquidity fee
    pub const LIQUIDITY_FEE: u128 = (SWAP_FEE * N) / (4 * (N - 1));
    pub const FEE_DENOMINATOR: u128 = 1000000;
    /// 1 share = 1e18, 18 decimals
    pub const DECIMALS: u128 = 18;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StableSwapContract {
        #[storage_field]
        data: impls::Data,
    }

    impl StableSwap for StableSwapContract {}

    impl StableSwapContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            self.multipliers = vec![1, 1000000000000, 1000000000000];
            instance
        }

    }
}
