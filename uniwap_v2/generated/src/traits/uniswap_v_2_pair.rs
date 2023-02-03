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
pub type UniswapV2PairRef = dyn UniswapV2Pair;

#[openbrush::trait_definition]
pub trait UniswapV2Pair {
    #[ink(message)]
    fn get_reserves(&self) -> Result<(u128, u128, u32), Error>;

    /// called once by the factory at time of deployment
    #[ink(message)]
    fn initialize(&mut self, token_0: AccountId, token_1: AccountId) -> Result<(), Error>;

    /// gas savings
    /// this low-level function should be called from a contract which performs important safety checks
    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> Result<u128, Error>;

    /// gas savings
    /// gas savings, must be defined here since totalSupply can update in _mintFee
    /// permanently lock the first MINIMUM_LIQUIDITY tokens
    /// reserve0 and reserve1 are up-to-date
    /// this low-level function should be called from a contract which performs important safety checks
    #[ink(message)]
    fn burn(&mut self, to: AccountId) -> Result<(u128, u128), Error>;

    /// gas savings
    /// gas savings
    /// gas savings
    /// gas savings, must be defined here since totalSupply can update in _mintFee
    /// using balances ensures pro-rata distribution
    /// using balances ensures pro-rata distribution
    /// reserve0 and reserve1 are up-to-date
    /// this low-level function should be called from a contract which performs important safety checks
    #[ink(message)]
    fn swap(
        &mut self,
        amount_0_out: u128,
        amount_1_out: u128,
        to: AccountId,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// gas savings
    /// scope for _token{0,1}, avoids stack too deep errors
    /// optimistically transfer tokens
    /// optimistically transfer tokens
    /// scope for reserve{0,1}Adjusted, avoids stack too deep errors
    /// force balances to match reserves
    #[ink(message)]
    fn skim(&mut self, to: AccountId) -> Result<(), Error>;

    /// gas savings
    /// gas savings
    /// force reserves to match balances
    #[ink(message)]
    fn sync(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn token_0(&self) -> AccountId;

    #[ink(message)]
    fn token_1(&self) -> AccountId;

    #[ink(message)]
    fn reserve_0(&self) -> u128;

    #[ink(message)]
    fn reserve_1(&self) -> u128;

    #[ink(message)]
    fn block_timestamp_last(&self) -> u32;

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> u128;

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> u128;

    #[ink(message)]
    fn k_last(&self) -> u128;

    #[ink(message)]
    fn unlocked(&self) -> u128;

}
