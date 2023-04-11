// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::traits::AccountId;
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

}
