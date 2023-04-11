// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    amount: u128,
}

#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    amount: u128,
}

#[openbrush::wrapper]
pub type IERC20Ref = dyn IERC20;

#[openbrush::trait_definition]
pub trait IERC20 {
    #[ink(message)]
    fn total_supply(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn balance_of(&self, account: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn transfer(&mut self, recipient: AccountId, amount: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        sender: AccountId,
        recipient: AccountId,
        amount: u128,
    ) -> Result<bool, Error>;

}
