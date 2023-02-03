#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use openbrush::traits::AccountId;

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

/// library with helper methods for oracles that are concerned with computing average prices

pub enum Error {
    Custom(String),
}


/// helper function that returns the current block timestamp within the range of uint32, i.e. [0, 2**32 - 1]
pub fn current_block_timestamp(&self) -> Result<u32, Error> {
    return Ok(<u32>::from(block.timestamp % 2.pow(32)))
}

/// produces the cumulative price using counterfactuals to save gas and avoid a call to sync.
pub fn current_cumulative_prices(&self, pair: AccountId) -> Result<(u128, u128, u32), Error> {
    let mut price_0_cumulative = Default::default();
    let mut price_1_cumulative = Default::default();
    let mut block_timestamp = Default::default();
    block_timestamp = self._current_block_timestamp()?;
    price_0_cumulative = i_uniswap_v_2_pair(pair)?.price_0_cumulative_last()?;
    price_1_cumulative = i_uniswap_v_2_pair(pair)?.price_1_cumulative_last()?;
    (reserve_0, reserve_1, block_timestamp_last) = i_uniswap_v_2_pair(pair)?.get_reserves()?;
    if block_timestamp_last != block_timestamp {
        let mut time_elapsed: u32 = block_timestamp - block_timestamp_last;
        price_0_cumulative +=
            <u128>::from(fixed_point.fraction(reserve_1, reserve_0)?.x) * time_elapsed;
        price_1_cumulative +=
            <u128>::from(fixed_point.fraction(reserve_0, reserve_1)?.x) * time_elapsed;
    }
    Ok((price_0_cumulative, price_1_cumulative, block_timestamp))
}

