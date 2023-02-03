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
pub type ExampleSwapToPriceRef = dyn ExampleSwapToPrice;

#[openbrush::trait_definition]
pub trait ExampleSwapToPrice {
    /// swaps an amount of either token such that the trade is profit-maximizing, given an external true price
    /// true price is expressed in the ratio of token A to token B
    /// caller must approve this contract to spend whichever token is intended to be swapped
    #[ink(message)]
    fn swap_to_price(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
        max_spend_token_a: u128,
        max_spend_token_b: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn router(&self) -> IUniswapV2Router01;

    #[ink(message)]
    fn factory(&self) -> AccountId;

}
