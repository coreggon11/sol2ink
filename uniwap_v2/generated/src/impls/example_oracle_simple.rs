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
    pub pair: IUniswapV2Pair,
    pub token_0: AccountId,
    pub token_1: AccountId,
    pub price_0_cumulative_last: u128,
    pub price_1_cumulative_last: u128,
    pub block_timestamp_last: u32,
    pub price_0_average: fixed_point::uq_112_x_112,
    pub price_1_average: fixed_point::uq_112_x_112,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ExampleOracleSimple for T {
    /// fetch the current accumulated price value (1 / 0)
    /// scope for token{0,1}, avoids stack too deep errors
    /// fetch the current accumulated price value (0 / 1)
    /// ensure that msg.sender is actually a V2 pair
    /// this strategy is unidirectional
    /// ensure that there's liquidity in the pair
    fn update(&mut self) -> Result<(), Error> {
        (price_0_cumulative, price_1_cumulative, block_timestamp) = uniswap_v_2_oracle_library
            .current_cumulative_prices(AccountId::from(self.data().pair))?;
        let mut time_elapsed: u32 = block_timestamp - self.data().block_timestamp_last;
        if !(time_elapsed >= PERIOD) {
            return Err(Error::Custom(String::from(
                "ExampleOracleSimple: PERIOD_NOT_ELAPSED",
            )))
        };
        self.data().price_0_average = fixed_point.uq_112_x_112(<u128>::from(
            (price_0_cumulative - self.data().price_0_cumulative_last) / time_elapsed,
        ))?;
        self.data().price_1_average = fixed_point.uq_112_x_112(<u128>::from(
            (price_1_cumulative - self.data().price_1_cumulative_last) / time_elapsed,
        ))?;
        self.data().price_0_cumulative_last = price_0_cumulative;
        self.data().price_1_cumulative_last = price_1_cumulative;
        self.data().block_timestamp_last = block_timestamp;
        Ok(())
    }

    /// overflow is desired
    /// this strategy only works with a V2 WETH pair
    /// ensure that at least one full period has passed since the last update
    /// overflow is desired, casting never truncates
    /// get V1 exchange
    /// cumulative price is in (uq112x112 price * seconds) units so we simply wrap it after division by time elapsed
    /// slippage parameter for V1, passed in by caller
    /// fail if we didn't get enough ETH back to repay our flash loan
    /// note this will always return 0 before update has been called successfully for the first time.
    fn consult(&self, token: AccountId, amount_in: u128) -> Result<u128, Error> {
        let mut amount_out = Default::default();
        if token == self.data().token_0 {
            amount_out = self.data().price_0_average.mul(amount_in)?.decode_144()?;
        } else {
            if !(token == self.data().token_1) {
                return Err(Error::Custom(String::from(
                    "ExampleOracleSimple: INVALID_TOKEN",
                )))
            };
            amount_out = self.data().price_1_average.mul(amount_in)?.decode_144()?;
        }
        Ok(amount_out)
    }

    fn token_0(&self) -> AccountId {
        self.data().token_0
    }

    fn token_1(&self) -> AccountId {
        self.data().token_1
    }

    fn price_0_cumulative_last(&self) -> u128 {
        self.data().price_0_cumulative_last
    }

    fn price_1_cumulative_last(&self) -> u128 {
        self.data().price_1_cumulative_last
    }

    fn block_timestamp_last(&self) -> u32 {
        self.data().block_timestamp_last
    }

    fn price_0_average(&self) -> fixed_point::uq_112_x_112 {
        self.data().price_0_average
    }

    fn price_1_average(&self) -> fixed_point::uq_112_x_112 {
        self.data().price_1_average
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
