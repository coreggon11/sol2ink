// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

use ink_prelude::vec::*;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
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
pub type UniswapV2FactoryRef = dyn UniswapV2Factory;

#[openbrush::trait_definition]
pub trait UniswapV2Factory {
    #[ink(message)]
    fn all_pairs_length(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> Result<AccountId, Error>;

    /// single check is sufficient
    /// populate mapping in the reverse direction
    #[ink(message)]
    fn set_fee_to(&mut self, fee_to: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn fee_to(&self) -> AccountId;

    #[ink(message)]
    fn fee_to_setter(&self) -> AccountId;

    #[ink(message)]
    fn get_pair(&self) -> Mapping<(AccountId, AccountId), AccountId>;

    #[ink(message)]
    fn all_pairs(&self) -> Vec<AccountId>;

}
