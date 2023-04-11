#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]


// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

/// a library for performing overflow-safe math, courtesy of DappHub (https://github.com/dapphub/ds-math)

pub enum Error {
    Custom(String),
}


pub fn add(&self, x: u128, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    if !((z = x + y) >= x) {
        return Err(Error::Custom(String::from("ds-math-add-overflow")))
    };
    Ok(z)
}

pub fn sub(&self, x: u128, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    if !((z = x - y) <= x) {
        return Err(Error::Custom(String::from("ds-math-sub-underflow")))
    };
    Ok(z)
}

pub fn mul(&self, x: u128, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    if !(y == 0 || (z = x * y) / y == x) {
        return Err(Error::Custom(String::from("ds-math-mul-overflow")))
    };
    Ok(z)
}

