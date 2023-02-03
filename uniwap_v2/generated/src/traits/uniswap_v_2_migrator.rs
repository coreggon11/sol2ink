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
pub type UniswapV2MigratorRef = dyn UniswapV2Migrator;

#[openbrush::trait_definition]
pub trait UniswapV2Migrator {
    #[ink(message)]
    fn migrate(
        &mut self,
        token: AccountId,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn factory_v_1(&self) -> IUniswapV1Factory;

    #[ink(message)]
    fn router(&self) -> IUniswapV2Router01;

}
