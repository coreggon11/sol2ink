#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use openbrush::traits::String;

// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.6.0) (utils/math/SafeMath.sol)
/// CAUTION
/// This version of SafeMath should only be used with Solidity 0.8 or later,
/// because it relies on the compiler's built in overflow checks.
/// @dev Wrappers over Solidity's arithmetic operations.
///
/// NOTE: `SafeMath` is generally not needed starting with Solidity 0.8, since the compiler
/// now has built in overflow checking.

pub enum Error {
    Custom(String),
}


/// @dev Returns the addition of two unsigned integers, with an overflow flag.
///
/// _Available since v3.4._
pub fn try_add(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    let mut c: u128 = a + b;
    if c < a {
        return Ok((_, _))
    }
    return Ok((_, _))
}

/// @dev Returns the subtraction of two unsigned integers, with an overflow flag.
///
/// _Available since v3.4._
pub fn try_sub(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if b > a {
        return Ok((_, _))
    }
    return Ok((_, _))
}

/// @dev Returns the multiplication of two unsigned integers, with an overflow flag.
///
/// _Available since v3.4._
pub fn try_mul(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if a == 0 {
        return Ok((_, _))
    }
    let mut c: u128 = a * b;
    if c / a != b {
        return Ok((_, _))
    }
    return Ok((_, _))
}

/// Gas optimization: this is cheaper than requiring 'a' not being zero, but the
/// benefit is lost if 'b' is also tested.
/// See: https://github.com/OpenZeppelin/openzeppelin-contracts/pull/522
/// @dev Returns the division of two unsigned integers, with a division by zero flag.
///
/// _Available since v3.4._
pub fn try_div(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if b == 0 {
        return Ok((_, _))
    }
    return Ok((_, _))
}

/// @dev Returns the remainder of dividing two unsigned integers, with a division by zero flag.
///
/// _Available since v3.4._
pub fn try_mod(&self, a: u128, b: u128) -> Result<(bool, u128), Error> {
    if b == 0 {
        return Ok((_, _))
    }
    return Ok((_, _))
}

/// @dev Returns the addition of two unsigned integers, reverting on
/// overflow.
///
/// Counterpart to Solidity's `+` operator.
///
/// Requirements:
///
/// - Addition cannot overflow.
pub fn add(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a + b)
}

/// @dev Returns the subtraction of two unsigned integers, reverting on
/// overflow (when the result is negative).
///
/// Counterpart to Solidity's `-` operator.
///
/// Requirements:
///
/// - Subtraction cannot overflow.
pub fn sub(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a - b)
}

/// @dev Returns the multiplication of two unsigned integers, reverting on
/// overflow.
///
/// Counterpart to Solidity's `*` operator.
///
/// Requirements:
///
/// - Multiplication cannot overflow.
pub fn mul(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a * b)
}

/// @dev Returns the integer division of two unsigned integers, reverting on
/// division by zero. The result is rounded towards zero.
///
/// Counterpart to Solidity's `/` operator.
///
/// Requirements:
///
/// - The divisor cannot be zero.
pub fn div(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a / b)
}

/// @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
/// reverting when dividing by zero.
///
/// Counterpart to Solidity's `%` operator. This function uses a `revert`
/// opcode (which leaves remaining gas untouched) while Solidity uses an
/// invalid opcode to revert (consuming all remaining gas).
///
/// Requirements:
///
/// - The divisor cannot be zero.
pub fn mod_is_rust_keyword(&self, a: u128, b: u128) -> Result<u128, Error> {
    return Ok(a % b)
}

/// @dev Returns the subtraction of two unsigned integers, reverting with custom message on
/// overflow (when the result is negative).
///
/// CAUTION: This function is deprecated because it requires allocating memory for the error
/// message unnecessarily. For custom revert reasons use {trySub}.
///
/// Counterpart to Solidity's `-` operator.
///
/// Requirements:
///
/// - Subtraction cannot overflow.
pub fn sub(&self, a: u128, b: u128, error_message: String) -> Result<u128, Error> {
    if !(b <= a) {
        return Err(Error::Custom(error_message))
    };
    return Ok(a - b)
}

/// @dev Returns the integer division of two unsigned integers, reverting with custom message on
/// division by zero. The result is rounded towards zero.
///
/// Counterpart to Solidity's `/` operator. Note: this function uses a
/// `revert` opcode (which leaves remaining gas untouched) while Solidity
/// uses an invalid opcode to revert (consuming all remaining gas).
///
/// Requirements:
///
/// - The divisor cannot be zero.
pub fn div(&self, a: u128, b: u128, error_message: String) -> Result<u128, Error> {
    if !(b > 0) {
        return Err(Error::Custom(error_message))
    };
    return Ok(a / b)
}

/// @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
/// reverting with custom message when dividing by zero.
///
/// CAUTION: This function is deprecated because it requires allocating memory for the error
/// message unnecessarily. For custom revert reasons use {tryMod}.
///
/// Counterpart to Solidity's `%` operator. This function uses a `revert`
/// opcode (which leaves remaining gas untouched) while Solidity uses an
/// invalid opcode to revert (consuming all remaining gas).
///
/// Requirements:
///
/// - The divisor cannot be zero.
pub fn mod_is_rust_keyword(&self, a: u128, b: u128, error_message: String) -> Result<u128, Error> {
    if !(b > 0) {
        return Err(Error::Custom(error_message))
    };
    return Ok(a % b)
}

