// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

use ink_prelude::vec::Vec;
use openbrush::traits::{
    AccountId,
    String,
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

pub enum Status {
    Pending,
    Shipped,
    Accepted,
    Rejected,
    Canceled,
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Todo {
    text: String,
    completed: bool,
    priority: u8,
    comment: String,
}


#[openbrush::wrapper]
pub type StructContractRef = dyn StructContract;

#[openbrush::trait_definition]
pub trait StructContract {
    #[ink(message)]
    fn get(&self) -> Result<Status, Error>;

    #[ink(message)]
    fn set(&mut self, status: Status) -> Result<(), Error>;

    #[ink(message)]
    fn cancel(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn reset(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn create_events(&mut self) -> Result<(), Error>;

    #[ink(message)]
    fn create_todo(&mut self, text: String, priority: u8, comment: String) -> Result<(), Error>;

    #[ink(message)]
    fn status(&self) -> Status;

    #[ink(message)]
    fn todos(&self) -> Vec<Todo>;

}
