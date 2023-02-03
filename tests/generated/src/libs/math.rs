#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

use ink_prelude::vec::*;
/// SPDX-License-Identifier: MIT
/// Simplified version of Curve's stable swap AMM
///Invariant - price of trade and amount of liquidity are determined by this equation
///
///An^n sum(x_i) + D = ADn^n + D^(n + 1) / (n^n prod(x_i))
///
///Topics
///0. Newton's method x_(n + 1) = x_n - f(x_n) / f'(x_n)
///1. Invariant
///2. Swap
///   - Calculate Y
///   - Calculate D
///3. Get virtual price
///4. Add liquidity
///   - Imbalance fee
///5. Remove liquidity
///6. Remove liquidity one token
///   - Calculate withdraw one token
///   - getYD
use openbrush::traits::ZERO_ADDRESS;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
    },
};

pub enum Error {
    Custom(String),
}


pub fn abs(&self, x: u128, y: u128) -> Result<u128, Error> {
    return Ok(if self.data().x >= y {
        self.data().x - y
    } else {
        y - self.data().x
    })
}

