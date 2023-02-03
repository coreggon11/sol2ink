// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use ink_prelude::vec::*;
pub use openbrush::traits::{
    AccountId,
    String,
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

#[ink(event)]
pub struct Mint {
    #[ink(topic)]
    sender: AccountId,
    amount_0: u128,
    amount_1: u128,
}

#[ink(event)]
pub struct Burn {
    #[ink(topic)]
    sender: AccountId,
    amount_0: u128,
    amount_1: u128,
    #[ink(topic)]
    to: AccountId,
}

#[ink(event)]
pub struct Swap {
    #[ink(topic)]
    sender: AccountId,
    amount_0_in: u128,
    amount_1_in: u128,
    amount_0_out: u128,
    amount_1_out: u128,
    #[ink(topic)]
    to: AccountId,
}

#[ink(event)]
pub struct Sync {
    reserve_0: u128,
    reserve_1: u128,
}

#[openbrush::wrapper]
pub type IUniswapV2PairRef = dyn IUniswapV2Pair;

#[openbrush::trait_definition]
pub trait IUniswapV2Pair {
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

    #[ink(message)]
    fn minimum_liquidity(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn factory(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn token_0(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn token_1(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn get_reserves(&self) -> Result<(u128, u128, u32), Error>;

    #[ink(message)]
    fn price_0_cumulative_last(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn price_1_cumulative_last(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn k_last(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn mint(&mut self, to: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn burn(&mut self, to: AccountId) -> Result<(u128, u128), Error>;

    #[ink(message)]
    fn swap(
        &mut self,
        amount_0_out: u128,
        amount_1_out: u128,
        to: AccountId,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn skim(&mut self, to: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn sync(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn initialize(&mut self, _: AccountId, _: AccountId) -> Result<(), Error>;

}
