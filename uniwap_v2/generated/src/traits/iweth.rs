// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type IWETHRef = dyn IWETH;

#[openbrush::trait_definition]
pub trait IWETH {
    #[ink(message, payable)]
    fn deposit(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u128) -> Result<bool, Error>;

    #[ink(message)]
    fn withdraw(&mut self, _: u128) -> Result<(), Error>;

}
