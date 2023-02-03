// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
pub use ink_prelude::vec::*;
use openbrush::traits::Storage;
pub use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// Variables in contract storage
    pub state: State,
    pub pid: i32,
    pub reaped: u32,
    pub card_1: Card,
    pub card_2: Card,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> example for T {
    /// Set contract storage
    /// Reading but not writing contract storage means function
    /// can be declared view
    fn is_zombie_reaper(&self) -> Result<bool, Error> {
        return Ok((self.data().pid == FIRST_PID && self.data().state != state.zombie))
    }

    ///must be pid 1 and not zombie ourselves
    /// Returning a constant does not access storage at all, so
    /// fn parameters
    /// function can be declared pure
    fn systemd_pid(&self) -> Result<u32, Error> {
        return Ok(<u32>::from(FIRST_PID))
    }

    /// declaration
    /// Note that cast is required to change sign from
    /// int32 to uint32
    ///Convert celcius to fahrenheit
    fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error> {
        let mut fahrenheit: i32 = celcius * 9 / 5 + 32;
        return Ok(fahrenheit)
    }

    /// assign value
    ///Convert fahrenheit to celcius
    /// assign array type
    fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error> {
        return Ok((fahrenheit - 32) * 5 / 9)
    }

    ///is this number a power-of-two
    fn is_power_of_2(&self, n: u128) -> Result<bool, Error> {
        return Ok(n != 0 && (n & (n - 1)) == 0)
    }

    /// nested array
    ///calculate the population count (number of set bits) using Brian Kerningham's way
    fn population_count(&self, n: u128) -> Result<u128, Error> {
        let mut count = Default::default();
        count = 0;
        while n != 0 {
            n &= (n - 1);
            count += 1;
        }
        Ok(count)
    }

    ///calculate the power of base to exp
    fn power(&self, base: u128, exp: u128) -> Result<u128, Error> {
        return Ok(base.pow(exp))
    }

    ///returns true if the address is 0
    fn is_address_zero(&self, a: AccountId) -> Result<bool, Error> {
        return Ok(a == ZERO_ADDRESS.into())
    }

    /// struct fields
    /// reverse the bytes in an array of 8 (endian swap)
    fn byte_8_reverse(&self, input: [u8; 8]) -> Result<[u8; 8], Error> {
        let mut out = Default::default();
        out = ((input << 56) & &hex::decode("ff00000000000000"))
            | ((input << 40) & &hex::decode("00ff000000000000"))
            | ((input << 24) & &hex::decode("0000ff0000000000"))
            | ((input << 8) & &hex::decode("000000ff00000000"))
            | ((input >> 8) & &hex::decode("00000000ff000000"))
            | ((input >> 24) & &hex::decode("0000000000ff0000"))
            | ((input >> 40) & &hex::decode("000000000000ff00"))
            | ((input >> 56) & &hex::decode("00000000000000ff"));
        Ok(out)
    }

    /// pop
    fn reap_processes(&mut self) -> Result<(), Error> {
        let mut n: u32 = 0;
        while n < 100 {
            if self._get_pid_state(n)? == state.zombie {
                self.data().reaped += 1;
            }
            n += 1;
        }
        Ok(())
    }

    /// reap!
    /// delete
    fn run_queue(&self) -> Result<u16, Error> {
        let mut count: u16 = 0;
        let mut n: u32 = 0;
        loop {
            if self._get_pid_state(n)? == state.waiting {
                count += 1;
            }
            if !n += 1 < 1000 {
                break
            }
        }
        return Ok(count)
    }

    /// This function does a lot of copying
    fn set_card_1(&mut self, c: Card) -> Result<Card, Error> {
        let mut previous = Default::default();
        previous = self.data().card_1;
        self.data().card_1 = c;
        Ok(previous)
    }

    ///return the ace of spades
    fn ace_of_spaces(&self) -> Result<Card, Error> {
        return Ok(card {
            s: suit.spades,
            v: self.data().value.ace,
        })
    }

    ///score card
    fn score_card(&self, c: Card) -> Result<u32, Error> {
        let mut score = Default::default();
        if c.s == suit.hearts {
            if c.v == self.data().value.ace {
                score = 14;
            }
            if c.v == self.data().value.king {
                score = 13;
            }
            if c.v == self.data().value.queen {
                score = 12;
            }
            if c.v == self.data().value.jack {
                score = 11;
            }
        }
        Ok(score)
    }

}

pub trait Internal {
    /// nested struct fields
    /// assign struct field
    /// This mocks a pid state
    fn _get_pid_state(&self, pid: u64) -> Result<State, Error>;

    /// push
    ///Overloaded function with different return value!
    fn _get_pid_state(&self) -> Result<u32, Error>;

}

impl<T: Storage<Data>> Internal for T {
    /// nested struct fields
    /// assign struct field
    /// This mocks a pid state
    default fn _get_pid_state(&self, pid: u64) -> Result<State, Error> {
        let mut n: u64 = 8;
        let mut i: u16 = 1;
        while i < 10 {
            if (i % 3) == 0 {
                n *= pid / <u64>::from(i);
            } else {
                n /= 3;
            }
            i += 1;
        }
        return Ok(state(n % <u64>::from(state.state_count))?)
    }

    /// push
    ///Overloaded function with different return value!
    default fn _get_pid_state(&self) -> Result<u32, Error> {
        return Ok(self.data().reaped)
    }

}
