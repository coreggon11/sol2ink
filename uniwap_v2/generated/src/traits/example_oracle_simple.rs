// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

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
pub type ExampleOracleSimpleRef = dyn ExampleOracleSimple;

#[openbrush::trait_definition]
pub trait ExampleOracleSimple {
    /// fetch the current accumulated price value (1 / 0)
    /// fetch the current accumulated price value (0 / 1)
    /// ensure that there's liquidity in the pair
    #[ink(message)]
    fn update(&mut self) -> Result<(), Error>;

    /// overflow is desired
    /// ensure that at least one full period has passed since the last update
    /// overflow is desired, casting never truncates
    /// cumulative price is in (uq112x112 price * seconds) units so we simply wrap it after division by time elapsed
    /// note this will always return 0 before update has been called successfully for the first time.
    #[ink(message)]
    fn consult(&self, token: AccountId, amount_in: u128) -> Result<u128, Error>;

}
