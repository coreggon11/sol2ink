// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::vec::*;
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

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory: AccountId,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ExampleComputeLiquidityValue for T {
    /// see UniswapV2LiquidityMathLibrary#getReservesAfterArbitrage
    fn get_reserves_after_arbitrage(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
    ) -> Result<(u128, u128), Error> {
        let mut reserve_a = Default::default();
        let mut reserve_b = Default::default();
        return Ok(
            uniswap_v_2_liquidity_math_library.get_reserves_after_arbitrage(
                self.data().factory,
                token_a,
                token_b,
                true_price_token_a,
                true_price_token_b,
            )?,
        )
    }

    /// see UniswapV2LiquidityMathLibrary#getLiquidityValue
    fn get_liquidity_value(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity_amount: u128,
    ) -> Result<(u128, u128), Error> {
        let mut token_a_amount = Default::default();
        let mut token_b_amount = Default::default();
        return Ok(uniswap_v_2_liquidity_math_library.get_liquidity_value(
            self.data().factory,
            token_a,
            token_b,
            liquidity_amount,
        )?)
    }

    /// see UniswapV2LiquidityMathLibrary#getLiquidityValueAfterArbitrageToPrice
    fn get_liquidity_value_after_arbitrage_to_price(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
        liquidity_amount: u128,
    ) -> Result<(u128, u128), Error> {
        let mut token_a_amount = Default::default();
        let mut token_b_amount = Default::default();
        return Ok(
            uniswap_v_2_liquidity_math_library.get_liquidity_value_after_arbitrage_to_price(
                self.data().factory,
                token_a,
                token_b,
                true_price_token_a,
                true_price_token_b,
                liquidity_amount,
            )?,
        )
    }

    /// test function to measure the gas cost of the above function
    fn get_gas_cost_of_get_liquidity_value_after_arbitrage_to_price(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
        liquidity_amount: u128,
    ) -> Result<u128, Error> {
        let mut gas_before: u128 = gasleft()?;
        uniswap_v_2_liquidity_math_library.get_liquidity_value_after_arbitrage_to_price(
            self.data().factory,
            token_a,
            token_b,
            true_price_token_a,
            true_price_token_b,
            liquidity_amount,
        )?;
        let mut gas_after: u128 = gasleft()?;
        return Ok(gas_before - gas_after)
    }

    fn factory(&self) -> AccountId {
        self.data().factory
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
