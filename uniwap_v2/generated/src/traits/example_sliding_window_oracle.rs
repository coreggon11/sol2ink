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
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Observation {
    timestamp: u128,
    price_0_cumulative: u128,
    price_1_cumulative: u128,
}


#[openbrush::wrapper]
pub type ExampleSlidingWindowOracleRef = dyn ExampleSlidingWindowOracle;

#[openbrush::trait_definition]
pub trait ExampleSlidingWindowOracle {
    /// spend up to the allowance of the token in
    /// returns the index of the observation corresponding to the given timestamp
    #[ink(message)]
    fn observation_index_of(&self, timestamp: u128) -> Result<u8, Error>;

    /// keep the rest! (ETH)
    /// no overflow issue. if observationIndex + 1 overflows, result is still zero.
    /// slippage parameter for V1, passed in by caller
    /// update the cumulative price for the observation at the current timestamp. each observation is updated at most
    /// once per epoch period.
    #[ink(message)]
    fn update(&mut self, token_a: AccountId, token_b: AccountId) -> Result<(), Error>;

    /// overflow is desired.
    /// returns the amount out corresponding to the amount in for a given token using the moving average over the time
    /// range [now - [windowSize, windowSize - periodSize * 2], now]
    /// update must have been called for the bucket corresponding to timestamp `now - windowSize`
    #[ink(message)]
    fn consult(
        &self,
        token_in: AccountId,
        amount_in: u128,
        token_out: AccountId,
    ) -> Result<u128, Error>;

    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn window_size(&self) -> u128;

    #[ink(message)]
    fn granularity(&self) -> u8;

    #[ink(message)]
    fn period_size(&self) -> u128;

    #[ink(message)]
    fn pair_observations(&self) -> Mapping<AccountId, Vec<Observation>>;

}
