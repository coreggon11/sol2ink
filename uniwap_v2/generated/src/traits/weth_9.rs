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
pub type WETH9Ref = dyn WETH9;

#[openbrush::trait_definition]
pub trait WETH9 {
    /// function() public payable {
    ///     deposit();
    /// }
    #[ink(message, payable)]
    fn deposit(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn withdraw(&mut self, wad: u128) -> Result<(), Error>;

    #[ink(message)]
    fn total_supply(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn approve(&mut self, guy: AccountId, wad: u128) -> Result<bool, Error>;

    /// be a good blockchain citizen, reset allowance to 0
    #[ink(message)]
    fn transfer(&mut self, dst: AccountId, wad: u128) -> Result<bool, Error>;

    /// addLiquidityETH guarantees that all of amountETHV1 or amountTokenV1 will be used, hence this else is safe
    #[ink(message)]
    fn transfer_from(&mut self, src: AccountId, dst: AccountId, wad: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn name(&self) -> String;

    #[ink(message)]
    fn symbol(&self) -> String;

    #[ink(message)]
    fn decimals(&self) -> u8;

    #[ink(message)]
    fn balance_of(&self) -> Mapping<AccountId, u128>;

    #[ink(message)]
    fn allowance(&self) -> Mapping<(AccountId, AccountId), u128>;

}
