// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::vec::Vec;
use openbrush::{
    modifier_definition,
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Storage,
        String,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub owner: AccountId,
    pub x: u128,
    pub locked: bool,
    ///Can use array for output
    pub arr: Vec<u128>,
    pub _reserved: Option<()>,
}

#[modifier_definition]
pub fn only_owner<T, F, R>(instance: &mut T, body: F) -> Result<R, Error>
where
    T: FunctionContract,
    F: FnOnce(&mut T) -> Result<R, Error>,
{
    if Self::env().caller() != self.data().owner {
        return Err(Error::Custom(String::from("Not owner")))
    }
    body(instance);
}

///Modifiers can take inputs
#[modifier_definition]
pub fn valid_address<T, F, R>(instance: &mut T, body: F, addr: AccountId) -> Result<R, Error>
where
    T: FunctionContract,
    F: FnOnce(&mut T) -> Result<R, Error>,
{
    if addr.is_zero() {
        return Err(Error::Custom(String::from("Not valid address")))
    }
    body(instance);
}

///Modifiers can be called before and / or after a function
#[modifier_definition]
pub fn no_reentrancy<T, F, R>(instance: &mut T, body: F) -> Result<R, Error>
where
    T: FunctionContract,
    F: FnOnce(&mut T) -> Result<R, Error>,
{
    if self.data().locked {
        return Err(Error::Custom(String::from("No reentrancy")))
    }
    self.data().locked = true;
    body(instance);
    self.data().locked = false;
}


impl<T: Storage<Data>> FunctionContract for T {
    # [modifiers (valid_address (new_owner) ?)]
    fn change_owner(&mut self, new_owner: AccountId) -> Result<(), Error> {
        self.data().owner = new_owner;
        Ok(())
    }

    fn decrement(&mut self, i: u128) -> Result<(), Error> {
        self.data().x -= i;
        if i > 1 {
            self.decrement(i - 1)?;
        }
        Ok(())
    }

    ///Functions can return multiple values.
    fn return_many(&self) -> Result<(u128, bool, u128), Error> {
        return Ok((1, _true, _2))
    }

    ///Return values can be named.
    fn named(&self) -> Result<(u128, bool, u128), Error> {
        let mut x = Default::default();
        let mut b = Default::default();
        let mut y = Default::default();
        return Ok((1, _true, _2))
    }

    ///Return values can be assigned to their name.
    ///In this case the return statement can be omitted.
    fn assigned(&self) -> Result<(u128, bool, u128), Error> {
        let mut x = Default::default();
        let mut b = Default::default();
        let mut y = Default::default();
        self.data().x = 1;
        b = true;
        y = 2;
        Ok((x, b, y))
    }

    ///Use destructuring assignment when calling another
    ///function that returns multiple values.
    fn destructuring_assignments(&self) -> Result<(u128, bool, u128, u128, u128), Error> {
        // Sol2Ink Not Implemented yet: (uint i, bool b, uint j) = returnMany();
        // Values can be left out.
        // Sol2Ink Not Implemented yet: (uint x, , uint y) = (4, 5, 6);
        return Ok((i, _b, _j, _x, _y))
    }

    ///Cannot use map for either input or output
    ///Can use array for input
    fn array_input(&mut self, arr: Vec<u128>) -> Result<(), Error> {
        Ok(())
    }

    fn array_output(&self) -> Result<Vec<u128>, Error> {
        return Ok(self.data().arr)
    }

    ///Call function with key-value inputs
    fn some_func_with_many_inputs(
        &self,
        x: u128,
        y: u128,
        z: u128,
        a: AccountId,
        b: bool,
        c: String,
    ) -> Result<u128, Error> {
    }

    fn call_func(&self) -> Result<u128, Error> {
        return Ok(self.some_func_with_many_inputs(1, 2, 3, ZERO_ADDRESS.into(), true, "c")?)
    }

    fn call_func_with_key_value(&self) -> Result<u128, Error> {
        return Ok(SomeFuncWithManyInputs {
            a: ZERO_ADDRESS.into(),
            b: true,
            c: "c",
            x: 1,
            y: 2,
            z: 3,
        })
    }

    fn owner(&self) -> AccountId {
        self.data().owner
    }

    fn x(&self) -> u128 {
        self.data().x
    }

    fn locked(&self) -> bool {
        self.data().locked
    }

    fn arr(&self) -> Vec<u128> {
        self.data().arr
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
