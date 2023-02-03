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

#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: u128,
}

#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    value: u128,
}

#[openbrush::wrapper]
pub type IUniswapV2ERC20Ref = dyn IUniswapV2ERC20;

#[openbrush::trait_definition]
pub trait IUniswapV2ERC20 {
    #[ink(message)]
    fn name(&self) -> Result<String, Error>;

    #[ink(message)]
    fn symbol(&self) -> Result<String, Error>;

    #[ink(message)]
    fn decimals(&self) -> Result<u8, Error>;

    #[ink(message)]
    fn total_supply(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128)
        -> Result<bool, Error>;

    #[ink(message)]
    fn domain_separator(&self) -> Result<[u8; 32], Error>;

    #[ink(message)]
    fn permit_typehash(&self) -> Result<[u8; 32], Error>;

    #[ink(message)]
    fn nonces(&self, owner: AccountId) -> Result<u128, Error>;

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

}
