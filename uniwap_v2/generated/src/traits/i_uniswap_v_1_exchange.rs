// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type IUniswapV1ExchangeRef = dyn IUniswapV1Exchange;

#[openbrush::trait_definition]
pub trait IUniswapV1Exchange {
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Result<u128, Error>;

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128)
        -> Result<bool, Error>;

    #[ink(message)]
    fn remove_liquidity(
        &mut self,
        _: u128,
        _: u128,
        _: u128,
        _: u128,
    ) -> Result<(u128, u128), Error>;

    #[ink(message)]
    fn token_to_eth_swap_input(&mut self, _: u128, _: u128, _: u128) -> Result<u128, Error>;

    #[ink(message, payable)]
    fn eth_to_token_swap_input(&mut self, _: u128, _: u128) -> Result<u128, Error>;

}
