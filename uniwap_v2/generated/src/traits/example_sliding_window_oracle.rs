// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::{
    storage::Mapping,
    traits::AccountId,
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
    /// returns the index of the observation corresponding to the given timestamp
    #[ink(message)]
    fn observation_index_of(&self, timestamp: u128) -> Result<u8, Error>;

    /// no overflow issue. if observationIndex + 1 overflows, result is still zero.
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

}
