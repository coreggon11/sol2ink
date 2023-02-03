// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type IUniswapV2Router02Ref = dyn IUniswapV2Router02;

#[openbrush::trait_definition]
pub trait IUniswapV2Router02 {
    #[ink(message)]
    fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<u128, Error>;

    #[ink(message)]
    fn remove_liquidity_eth_with_permit_supporting_fee_on_transfer_tokens(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
        approve_max: bool,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<u128, Error>;

    #[ink(message)]
    fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message, payable)]
    fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
        &mut self,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error>;

}
