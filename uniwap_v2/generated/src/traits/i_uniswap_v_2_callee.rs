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
