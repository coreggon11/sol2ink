// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::traits::AccountId;
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
pub type ExampleComputeLiquidityValueRef = dyn ExampleComputeLiquidityValue;

#[openbrush::trait_definition]
pub trait ExampleComputeLiquidityValue {
    /// see UniswapV2LiquidityMathLibrary#getReservesAfterArbitrage
    #[ink(message)]
    fn get_reserves_after_arbitrage(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
    ) -> Result<(u128, u128), Error>;

    /// see UniswapV2LiquidityMathLibrary#getLiquidityValue
    #[ink(message)]
    fn get_liquidity_value(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity_amount: u128,
    ) -> Result<(u128, u128), Error>;

    /// see UniswapV2LiquidityMathLibrary#getLiquidityValueAfterArbitrageToPrice
    #[ink(message)]
    fn get_liquidity_value_after_arbitrage_to_price(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
        liquidity_amount: u128,
    ) -> Result<(u128, u128), Error>;

    /// test function to measure the gas cost of the above function
    #[ink(message)]
    fn get_gas_cost_of_get_liquidity_value_after_arbitrage_to_price(
        &self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
        liquidity_amount: u128,
    ) -> Result<u128, Error>;

}
