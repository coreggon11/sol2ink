// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::traits::AccountId;

#[ink(event)]
pub struct PairCreated {
    #[ink(topic)]
    token_0: AccountId,
    #[ink(topic)]
    token_1: AccountId,
    pair: AccountId,
    anonymous: u128,
}

#[openbrush::wrapper]
pub type IUniswapV2FactoryRef = dyn IUniswapV2Factory;

#[openbrush::trait_definition]
pub trait IUniswapV2Factory {
    #[ink(message)]
    fn fee_to(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn fee_to_setter(&self) -> Result<AccountId, Error>;

    #[ink(message)]
    fn get_pair(&self, token_a: AccountId, token_b: AccountId) -> Result<AccountId, Error>;

    #[ink(message)]
    fn all_pairs(&self, _: u128) -> Result<AccountId, Error>;

    #[ink(message)]
    fn all_pairs_length(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> Result<AccountId, Error>;

    #[ink(message)]
    fn set_fee_to(&mut self, _: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn set_fee_to_setter(&mut self, _: AccountId) -> Result<(), Error>;

}
