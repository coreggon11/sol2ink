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
pub type IUniswapV1FactoryRef = dyn IUniswapV1Factory;

#[openbrush::trait_definition]
pub trait IUniswapV1Factory {
    #[ink(message)]
    fn get_exchange(&self, _: AccountId) -> Result<AccountId, Error>;

}
