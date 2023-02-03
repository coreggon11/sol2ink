#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]


// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

/// a library for handling binary fixed point numbers (https://en.wikipedia.org/wiki/Q_(number_format))
/// range: [0, 2**112 - 1]
/// resolution: 1 / 2**112

pub enum Error {
    Custom(String),
}

pub const Q_112: u128 = 2.pow(112);

/// encode a uint112 as a UQ112x112
pub fn encode(&self, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    z = <u128>::from(y) * self.data().q_112;
    Ok(z)
}

/// never overflows
/// divide a UQ112x112 by a uint112, returning a UQ112x112
pub fn uqdiv(&self, x: u128, y: u128) -> Result<u128, Error> {
    let mut z = Default::default();
    z = x / <u128>::from(y);
    Ok(z)
}

