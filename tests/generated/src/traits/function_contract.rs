// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::traits::{
    AccountId,
    AccountIdExt,
    String,
    ZERO_ADDRESS,
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
pub type FunctionContractRef = dyn FunctionContract;

#[openbrush::trait_definition]
pub trait FunctionContract {
    #[ink(message)]
    fn change_owner(&mut self, new_owner: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn decrement(&mut self, i: u128) -> Result<(), Error>;

    /// Functions can return multiple values.
    #[ink(message)]
    fn return_many(&self) -> Result<(u128, bool, u128), Error>;

    /// Return values can be named.
    #[ink(message)]
    fn named(&self) -> Result<(u128, bool, u128), Error>;

    /// Return values can be assigned to their name.
    /// In this case the return statement can be omitted.
    #[ink(message)]
    fn assigned(&self) -> Result<(u128, bool, u128), Error>;

    /// Use destructuring assignment when calling another
    /// function that returns multiple values.
    #[ink(message)]
    fn destructuring_assignments(&self) -> Result<(u128, bool, u128, u128, u128), Error>;

    /// Values can be left out.
    /// Cannot use map for either input or output
    /// Can use array for input
    #[ink(message)]
    fn array_input(&mut self, arr: Vec<u128>) -> Result<(), Error>;

    #[ink(message)]
    fn array_output(&self) -> Result<Vec<u128>, Error>;

    /// Call function with key-value inputs
    #[ink(message)]
    fn some_func_with_many_inputs(
        &self,
        x: u128,
        y: u128,
        z: u128,
        a: AccountId,
        b: bool,
        c: String,
    ) -> Result<u128, Error>;

    #[ink(message)]
    fn call_func(&self) -> Result<u128, Error>;

    #[ink(message)]
    fn call_func_with_key_value(&self) -> Result<u128, Error>;

}
