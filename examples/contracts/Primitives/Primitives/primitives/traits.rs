// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

use openbrush::traits::AccountId;
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}

pub enum Oper {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Pow,
    Shl,
    Shr,
    Or,
    And,
    Xor,
}



#[openbrush::wrapper]
pub type primitivesRef = dyn primitives;

#[openbrush::trait_definition]
pub trait primitives {
    #[ink(message)]
    fn is_mul(&self, op: oper) -> Result<bool, Error>;

    #[ink(message)]
    fn return_div(&self) -> Result<oper, Error>;

    #[ink(message)]
    fn op_i_64(&self, op: oper, a: i64, b: i64) -> Result<i64, Error>;

    #[ink(message)]
    fn op_u_64(&self, op: oper, a: u64, b: u64) -> Result<u64, Error>;

    #[ink(message)]
    fn op_u_256(&self, op: oper, a: u128, b: u128) -> Result<u128, Error>;

    #[ink(message)]
    fn op_i_256(&self, op: oper, a: i128, b: i128) -> Result<i128, Error>;

    #[ink(message)]
    fn return_u_8_6(&self) -> Result<[u8; 6], Error>;

    #[ink(message)]
    fn op_u_8_5_shift(&self, op: oper, a: [u8; 5], r: u64) -> Result<[u8; 5], Error>;

    #[ink(message)]
    fn op_u_8_5(&self, op: oper, a: [u8; 5], b: [u8; 5]) -> Result<[u8; 5], Error>;

    #[ink(message)]
    fn op_u_8_14_shift(&self, op: oper, a: [u8; 14], r: u64) -> Result<[u8; 14], Error>;

    #[ink(message)]
    fn op_u_8_14(&self, op: oper, a: [u8; 14], b: [u8; 14]) -> Result<[u8; 14], Error>;

    #[ink(message)]
    fn address_passthrough(&self, a: AccountId) -> Result<AccountId, Error>;

}
