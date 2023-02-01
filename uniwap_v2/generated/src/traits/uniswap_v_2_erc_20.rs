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
pub type UniswapV2ERC20Ref = dyn UniswapV2ERC20;

#[openbrush::trait_definition]
pub trait UniswapV2ERC20 {
    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128)
        -> Result<bool, Error>;

    #[ink(message)]
    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: u128,
        deadline: u128,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(), Error>;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn balance_of(&self) -> Mapping<AccountId, u128>;

    #[ink(message)]
    fn allowance(&self) -> Mapping<(AccountId, AccountId), u128>;

    #[ink(message)]
    fn domain_separator(&self) -> [u8; 32];

    #[ink(message)]
    fn nonces(&self) -> Mapping<AccountId, u128>;

}
