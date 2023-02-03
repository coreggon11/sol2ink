#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

use ink_prelude::vec::*;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
        ZERO_ADDRESS,
    },
};

pub enum Error {
    Custom(String),
}


/// returns sorted token addresses, used to handle return values from pairs sorted in this order
pub fn sort_tokens(
    &self,
    token_a: AccountId,
    token_b: AccountId,
) -> Result<(AccountId, AccountId), Error> {
    let mut token_0 = Default::default();
    let mut token_1 = Default::default();
    if !(token_a != token_b) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: IDENTICAL_ADDRESSES",
        )))
    };
    (_, _) = if token_a < token_b { (_, _) } else { (_, _) };
    if !(self.data().token_0 != ZERO_ADDRESS.into()) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: ZERO_ADDRESS",
        )))
    };
    Ok((token_0, token_1))
}

/// calculates the CREATE2 address for a pair without making any external calls
pub fn pair_for(
    &self,
    factory: AccountId,
    token_a: AccountId,
    token_b: AccountId,
) -> Result<AccountId, Error> {
    let mut pair = Default::default();
    (token_0, token_1) = self._sort_tokens(token_a, token_b)?;
    pair = AccountId::from(<u128>::from(keccak_256(abi.encode_packed(
        &hex::decode("ff"),
        self.data().factory,
        keccak_256(abi.encode_packed(self.data().token_0, self.data().token_1)?)?,
        &hex::decode("96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f"),
    )?)?));
    Ok(pair)
}

/// if time has elapsed since the last update on the pair, mock the accumulated price values
/// init code hash
/// fetches and sorts the reserves for a pair
/// subtraction overflow is desired
pub fn get_reserves(
    &self,
    factory: AccountId,
    token_a: AccountId,
    token_b: AccountId,
) -> Result<(u128, u128), Error> {
    let mut reserve_a = Default::default();
    let mut reserve_b = Default::default();
    (token_0, _) = self._sort_tokens(token_a, token_b)?;
    (reserve_0, reserve_1, _) =
        i_uniswap_v_2_pair(self._pair_for(self.data().factory, token_a, token_b)?)?
            .get_reserves()?;
    (_, _) = if token_a == self.data().token_0 {
        (_, _)
    } else {
        (_, _)
    };
    Ok((reserve_a, reserve_b))
}

/// addition overflow is desired
/// counterfactual
/// counterfactual
/// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
pub fn quote(&self, amount_a: u128, reserve_a: u128, reserve_b: u128) -> Result<u128, Error> {
    let mut amount_b = Default::default();
    if !(amount_a > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INSUFFICIENT_AMOUNT",
        )))
    };
    if !(reserve_a > 0 && reserve_b > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INSUFFICIENT_LIQUIDITY",
        )))
    };
    amount_b = amount_a.mul(reserve_b)? / reserve_a;
    Ok(amount_b)
}

/// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
pub fn get_amount_out(
    &self,
    amount_in: u128,
    reserve_in: u128,
    reserve_out: u128,
) -> Result<u128, Error> {
    let mut amount_out = Default::default();
    if !(amount_in > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INSUFFICIENT_INPUT_AMOUNT",
        )))
    };
    if !(reserve_in > 0 && reserve_out > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INSUFFICIENT_LIQUIDITY",
        )))
    };
    let mut amount_in_with_fee: u128 = amount_in.mul(997)?;
    let mut numerator: u128 = amount_in_with_fee.mul(reserve_out)?;
    let mut denominator: u128 = reserve_in.mul(1000)?.add(amount_in_with_fee)?;
    amount_out = numerator / denominator;
    Ok(amount_out)
}

/// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
pub fn get_amount_in(
    &self,
    amount_out: u128,
    reserve_in: u128,
    reserve_out: u128,
) -> Result<u128, Error> {
    let mut amount_in = Default::default();
    if !(amount_out > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT",
        )))
    };
    if !(reserve_in > 0 && reserve_out > 0) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INSUFFICIENT_LIQUIDITY",
        )))
    };
    let mut numerator: u128 = reserve_in.mul(amount_out)?.mul(1000)?;
    let mut denominator: u128 = reserve_out.sub(amount_out)?.mul(997)?;
    amount_in = (numerator / denominator).add(1)?;
    Ok(amount_in)
}

/// performs chained getAmountOut calculations on any number of pairs
pub fn get_amounts_out(
    &self,
    factory: AccountId,
    amount_in: u128,
    path: Vec<AccountId>,
) -> Result<Vec<u128>, Error> {
    let mut amounts = Default::default();
    if !(path.length >= 2) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INVALID_PATH",
        )))
    };
    amounts = vec![u128::default(); path.length];
    amounts[0] = amount_in;
    while i < path.length - 1 {
        (reserve_in, reserve_out) =
            self._get_reserves(self.data().factory, path[i], path[i + 1])?;
        amounts[i + 1] = self._get_amount_out(amounts[i], reserve_in, reserve_out)?;
        i += 1;
    }
    Ok(amounts)
}

/// performs chained getAmountIn calculations on any number of pairs
pub fn get_amounts_in(
    &self,
    factory: AccountId,
    amount_out: u128,
    path: Vec<AccountId>,
) -> Result<Vec<u128>, Error> {
    let mut amounts = Default::default();
    if !(path.length >= 2) {
        return Err(Error::Custom(String::from(
            "UniswapV2Library: INVALID_PATH",
        )))
    };
    amounts = vec![u128::default(); path.length];
    amounts[amounts.length - 1] = amount_out;
    let mut i: u128 = path.length - 1;
    while i > 0 {
        (reserve_in, reserve_out) =
            self._get_reserves(self.data().factory, path[i - 1], path[i])?;
        amounts[i - 1] = self._get_amount_in(amounts[i], reserve_in, reserve_out)?;
        i -= 1;
    }
    Ok(amounts)
}

