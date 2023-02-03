// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use ink_prelude::vec::*;
pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type IUniswapV2CalleeRef = dyn IUniswapV2Callee;

#[openbrush::trait_definition]
pub trait IUniswapV2Callee {
    #[ink(message)]
    fn uniswap_v_2_call(
        &mut self,
        sender: AccountId,
        amount_0: u128,
        amount_1: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

}
