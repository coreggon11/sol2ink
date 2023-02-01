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
pub type RouterEventEmitterRef = dyn RouterEventEmitter;

#[openbrush::trait_definition]
pub trait RouterEventEmitter {
    #[ink(message)]
    fn swap_exact_tokens_for_tokens(
        &mut self,
        router: AccountId,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn swap_tokens_for_exact_tokens(
        &mut self,
        router: AccountId,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message, payable)]
    fn swap_exact_eth_for_tokens(
        &mut self,
        router: AccountId,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn swap_tokens_for_exact_eth(
        &mut self,
        router: AccountId,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn swap_exact_tokens_for_eth(
        &mut self,
        router: AccountId,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message, payable)]
    fn swap_eth_for_exact_tokens(
        &mut self,
        router: AccountId,
        amount_out: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

}
