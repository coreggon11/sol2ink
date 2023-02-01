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



#[openbrush::wrapper]
pub type ExampleOracleSimpleRef = dyn ExampleOracleSimple;

#[openbrush::trait_definition]
pub trait ExampleOracleSimple {
    /// fetch the current accumulated price value (1 / 0)
    /// scope for token{0,1}, avoids stack too deep errors
    /// fetch the current accumulated price value (0 / 1)
    /// ensure that msg.sender is actually a V2 pair
    /// this strategy is unidirectional
    /// ensure that there's liquidity in the pair
    #[ink(message)]
    fn update(&mut self) -> Result<(), Error>;

    /// overflow is desired
    /// this strategy only works with a V2 WETH pair
    /// ensure that at least one full period has passed since the last update
    /// overflow is desired, casting never truncates
    /// get V1 exchange
    /// cumulative price is in (uq112x112 price * seconds) units so we simply wrap it after division by time elapsed
    /// slippage parameter for V1, passed in by caller
    /// fail if we didn't get enough ETH back to repay our flash loan
    /// note this will always return 0 before update has been called successfully for the first time.
    #[ink(message)]
    fn consult(&self, token: AccountId, amount_in: u128) -> Result<u128, Error>;

    #[ink(message)]
    fn pair(&self) -> IUniswapV2Pair;

    #[ink(message)]
    fn token_0(&self) -> AccountId;

    #[ink(message)]
    fn token_1(&self) -> AccountId;

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> u128;

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> u128;

    #[ink(message)]
    fn block_timestamp_last(&self) -> u32;

    #[ink(message)]
    fn price_0_average(&self) -> fixed_point::uq_112_x_112;

    #[ink(message)]
    fn price_1_average(&self) -> fixed_point::uq_112_x_112;

}
