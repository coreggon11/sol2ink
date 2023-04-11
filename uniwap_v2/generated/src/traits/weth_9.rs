// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        String,
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

    #[ink(message)]
    fn transfer(&mut self, dst: AccountId, wad: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn transfer_from(&mut self, src: AccountId, dst: AccountId, wad: u128) -> Result<bool, Error>;

}
