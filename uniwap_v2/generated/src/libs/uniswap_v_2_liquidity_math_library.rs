#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

use ink_prelude::vec::*;
/// library containing some math for dealing with the liquidity shares of a pair, e.g. computing their exact value
/// in terms of the underlying tokens
use openbrush::traits::ZERO_ADDRESS;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
    },
};

pub enum Error {
    Custom(String),
}


/// computes the direction and magnitude of the profit-maximizing trade
pub fn compute_profit_maximizing_trade(
    &self,
    true_price_token_a: u128,
    true_price_token_b: u128,
    reserve_a: u128,
    reserve_b: u128,
) -> Result<(bool, u128), Error> {
    let mut a_to_b = Default::default();
    let mut amount_in = Default::default();
    a_to_b = full_math.mul_div(reserve_a, true_price_token_b, reserve_b)? < true_price_token_a;
    let mut invariant: u128 = reserve_a.mul(reserve_b)?;
    let mut left_side: u128 = babylonian.sqrt(
        full_math.mul_div(
            invariant.mul(1000)?,
            if a_to_b {
                true_price_token_a
            } else {
                true_price_token_b
            },
            (if a_to_b {
                true_price_token_b
            } else {
                true_price_token_a
            })
            .mul(997)?,
        )?,
    )?;
    let mut right_side: u128 = (if a_to_b {
        reserve_a.mul(1000)?
    } else {
        reserve_b.mul(1000)?
    }) / 997;
    if left_side < right_side {
        return Ok((_, _))
    }
    amount_in = left_side.sub(right_side)?;
    Ok((a_to_b, amount_in))
}

/// compute the amount that must be sent to move the price to the profit-maximizing price
/// gets the reserves after an arbitrage moves the price to the profit-maximizing ratio given an externally observed true price
pub fn get_reserves_after_arbitrage(
    &self,
    factory: AccountId,
    token_a: AccountId,
    token_b: AccountId,
    true_price_token_a: u128,
    true_price_token_b: u128,
) -> Result<(u128, u128), Error> {
    let mut reserve_a = Default::default();
    let mut reserve_b = Default::default();
    (_, _) = uniswap_v_2_library.get_reserves(self.data().factory, token_a, token_b)?;
    if !(reserve_a > 0 && reserve_b > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2ArbitrageLibrary: ZERO_PAIR_RESERVES",
        )))
    };
    (a_to_b, amount_in) = self._compute_profit_maximizing_trade(
        true_price_token_a,
        true_price_token_b,
        reserve_a,
        reserve_b,
    )?;
    if amount_in == 0 {
        return Ok((_, _))
    }
    if a_to_b {
        let mut amount_out: u128 =
            uniswap_v_2_library.get_amount_out(amount_in, reserve_a, reserve_b)?;
        reserve_a += amount_in;
        reserve_b -= amount_out;
    } else {
        let mut amount_out: u128 =
            uniswap_v_2_library.get_amount_out(amount_in, reserve_b, reserve_a)?;
        reserve_b += amount_in;
        reserve_a -= amount_out;
    }
    Ok((reserve_a, reserve_b))
}

/// first get reserves before the swap
/// then compute how much to swap to arb to the true price
/// now affect the trade to the reserves
/// computes liquidity value given all the parameters of the pair
pub fn compute_liquidity_value(
    &self,
    reserves_a: u128,
    reserves_b: u128,
    total_supply: u128,
    liquidity_amount: u128,
    fee_on: bool,
    k_last: u128,
) -> Result<(u128, u128), Error> {
    let mut token_a_amount = Default::default();
    let mut token_b_amount = Default::default();
    if fee_on && self.data().k_last > 0 {
        let mut root_k: u128 = babylonian.sqrt(reserves_a.mul(reserves_b)?)?;
        let mut root_k_last: u128 = babylonian.sqrt(self.data().k_last)?;
        if root_k > root_k_last {
            let mut numerator_1: u128 = self.data().total_supply;
            let mut numerator_2: u128 = root_k.sub(root_k_last)?;
            let mut denominator: u128 = root_k.mul(5)?.add(root_k_last)?;
            let mut fee_liquidity: u128 =
                full_math.mul_div(numerator_1, numerator_2, denominator)?;
            self.data().total_supply = self.data().total_supply.add(fee_liquidity)?;
        }
    }
    return Ok((_, _))
}

/// get all current parameters from the pair and compute value of a liquidity amount
/// **note this is subject to manipulation, e.g. sandwich attacks**. prefer passing a manipulation resistant price to
/// #getLiquidityValueAfterArbitrageToPrice
pub fn get_liquidity_value(
    &self,
    factory: AccountId,
    token_a: AccountId,
    token_b: AccountId,
    liquidity_amount: u128,
) -> Result<(u128, u128), Error> {
    let mut token_a_amount = Default::default();
    let mut token_b_amount = Default::default();
    (reserves_a, reserves_b) =
        uniswap_v_2_library.get_reserves(self.data().factory, token_a, token_b)?;
    let mut pair: IUniswapV2Pair =
        i_uniswap_v_2_pair(uniswap_v_2_library.pair_for(self.data().factory, token_a, token_b)?)?;
    let mut fee_on: bool =
        i_uniswap_v_2_factory(self.data().factory)?.fee_to()? != ZERO_ADDRESS.into();
    let mut k_last: u128 = if fee_on { pair.k_last()? } else { 0 };
    let mut total_supply: u128 = pair.total_supply()?;
    return Ok(self._compute_liquidity_value(
        reserves_a,
        reserves_b,
        self.data().total_supply,
        liquidity_amount,
        fee_on,
        self.data().k_last,
    )?)
}

/// given two tokens, tokenA and tokenB, and their "true price", i.e. the observed ratio of value of token A to token B,
/// and a liquidity amount, returns the value of the liquidity in terms of tokenA and tokenB
pub fn get_liquidity_value_after_arbitrage_to_price(
    &self,
    factory: AccountId,
    token_a: AccountId,
    token_b: AccountId,
    true_price_token_a: u128,
    true_price_token_b: u128,
    liquidity_amount: u128,
) -> Result<(u128, u128), Error> {
    let mut token_a_amount = Default::default();
    let mut token_b_amount = Default::default();
    let mut fee_on: bool =
        i_uniswap_v_2_factory(self.data().factory)?.fee_to()? != ZERO_ADDRESS.into();
    let mut pair: IUniswapV2Pair =
        i_uniswap_v_2_pair(uniswap_v_2_library.pair_for(self.data().factory, token_a, token_b)?)?;
    let mut k_last: u128 = if fee_on { pair.k_last()? } else { 0 };
    let mut total_supply: u128 = pair.total_supply()?;
    if !(self.data().total_supply >= liquidity_amount && liquidity_amount > 0) {
        return Err(Error::Custom(String::from(
            "ComputeLiquidityValue: LIQUIDITY_AMOUNT",
        )))
    };
    (reserves_a, reserves_b) = self._get_reserves_after_arbitrage(
        self.data().factory,
        token_a,
        token_b,
        true_price_token_a,
        true_price_token_b,
    )?;
    return Ok(self._compute_liquidity_value(
        reserves_a,
        reserves_b,
        self.data().total_supply,
        liquidity_amount,
        fee_on,
        self.data().k_last,
    )?)
}

