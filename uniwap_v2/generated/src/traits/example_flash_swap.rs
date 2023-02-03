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
pub type ExampleFlashSwapRef = dyn ExampleFlashSwap;

#[openbrush::trait_definition]
pub trait ExampleFlashSwap {
    /// gets tokens/WETH via a V2 flash swap, swaps for the ETH/tokens on V1, repays V2, and keeps the rest!
    #[ink(message)]
    fn uniswap_v_2_call(
        &mut self,
        sender: AccountId,
        amount_0: u128,
        amount_1: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn factory_v_1(&self) -> IUniswapV1Factory;

    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn weth(&self) -> Iweth;

}
