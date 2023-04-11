// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::traits::AccountId;

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
