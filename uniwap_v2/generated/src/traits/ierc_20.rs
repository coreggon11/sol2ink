// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

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

#[openbrush::wrapper]
pub type IERC20Ref = dyn IERC20;

#[openbrush::trait_definition]
pub trait IERC20 {
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

}
