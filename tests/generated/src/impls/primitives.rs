// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
pub use openbrush::traits::AccountId;
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> primitives for T {
    fn is_mul(&self, op: Oper) -> Result<bool, Error> {
        return Ok(op == oper.mul)
    }

    fn return_div(&self) -> Result<Oper, Error> {
        return Ok(oper.div)
    }

    fn op_i_64(&self, op: Oper, a: i64, b: i64) -> Result<i64, Error> {
        if op == oper.add {
            return Ok(a + b)
        } else if op == oper.sub {
            return Ok(a - b)
        } else if op == oper.mul {
            return Ok(a * b)
        } else if op == oper.div {
            return Ok(a / b)
        } else if op == oper.modulo {
            return Ok(a % b)
        } else if op == oper.shl {
            return Ok(a << b)
        } else if op == oper.shr {
            return Ok(a >> b)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn op_u_64(&self, op: Oper, a: u64, b: u64) -> Result<u64, Error> {
        if op == oper.add {
            return Ok(a + b)
        } else if op == oper.sub {
            return Ok(a - b)
        } else if op == oper.mul {
            return Ok(a * b)
        } else if op == oper.div {
            return Ok(a / b)
        } else if op == oper.modulo {
            return Ok(a % b)
        } else if op == oper.pow {
            return Ok(a.pow(b))
        } else if op == oper.shl {
            return Ok(a << b)
        } else if op == oper.shr {
            return Ok(a >> b)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn op_u_256(&self, op: Oper, a: u128, b: u128) -> Result<u128, Error> {
        if op == oper.add {
            return Ok(a + b)
        } else if op == oper.sub {
            return Ok(a - b)
        } else if op == oper.mul {
            return Ok(a * b)
        } else if op == oper.div {
            return Ok(a / b)
        } else if op == oper.modulo {
            return Ok(a % b)
        } else if op == oper.pow {
            return Ok(a.pow(<u128>::from(b)))
        } else if op == oper.shl {
            return Ok(a << b)
        } else if op == oper.shr {
            return Ok(a >> b)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn op_i_256(&self, op: Oper, a: i128, b: i128) -> Result<i128, Error> {
        if op == oper.add {
            return Ok(a + b)
        } else if op == oper.sub {
            return Ok(a - b)
        } else if op == oper.mul {
            return Ok(a * b)
        } else if op == oper.div {
            return Ok(a / b)
        } else if op == oper.modulo {
            return Ok(a % b)
        } else if op == oper.shl {
            return Ok(a << b)
        } else if op == oper.shr {
            return Ok(a >> b)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn return_u_8_6(&self) -> Result<[u8; 6], Error> {
        return Ok("ABCDEF")
    }

    fn op_u_8_5_shift(&self, op: Oper, a: [u8; 5], r: u64) -> Result<[u8; 5], Error> {
        if op == oper.shl {
            return Ok(a << r)
        } else if op == oper.shr {
            return Ok(a >> r)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn op_u_8_5(&self, op: Oper, a: [u8; 5], b: [u8; 5]) -> Result<[u8; 5], Error> {
        if op == oper.or {
            return Ok(a | b)
        } else if op == oper.and {
            return Ok(a & b)
        } else if op == oper.xor {
            return Ok(a ^ b)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn op_u_8_14_shift(&self, op: Oper, a: [u8; 14], r: u64) -> Result<[u8; 14], Error> {
        if op == oper.shl {
            return Ok(a << r)
        } else if op == oper.shr {
            return Ok(a >> r)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn op_u_8_14(&self, op: Oper, a: [u8; 14], b: [u8; 14]) -> Result<[u8; 14], Error> {
        if op == oper.or {
            return Ok(a | b)
        } else if op == oper.and {
            return Ok(a & b)
        } else if op == oper.xor {
            return Ok(a ^ b)
        } else {
            return Err(Error::Custom(String::from("_")))
        }
    }

    fn address_passthrough(&self, a: AccountId) -> Result<AccountId, Error> {
        return Ok(a)
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
