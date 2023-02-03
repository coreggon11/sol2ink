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

#[openbrush::wrapper]
pub type IWETHRef = dyn IWETH;

#[openbrush::trait_definition]
pub trait IWETH {
    #[ink(message, payable)]
    fn deposit(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn withdraw(&mut self, _: u128) -> Result<(), Error>;

}
