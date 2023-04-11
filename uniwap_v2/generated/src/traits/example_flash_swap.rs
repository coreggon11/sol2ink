// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use ink::prelude::vec::*;
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

}
