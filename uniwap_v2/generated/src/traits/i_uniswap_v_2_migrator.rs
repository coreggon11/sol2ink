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
pub type IUniswapV2MigratorRef = dyn IUniswapV2Migrator;

#[openbrush::trait_definition]
pub trait IUniswapV2Migrator {
    #[ink(message)]
    fn migrate(
        &mut self,
        token: AccountId,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

}
