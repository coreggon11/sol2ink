// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    impls::Internal as _,
    traits::*,
};
use openbrush::traits::{
    AccountId,
    AccountIdExt,
    Storage,
    ZERO_ADDRESS,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    ///Variables in contract storage
    pub state: State,
    pub pid: i32,
    pub reaped: u32,
    pub card_1: card,
    pub card_2: card,
    pub _reserved: Option<()>,
}
impl<T: Storage<Data>> example for T {
    ///Reading but not writing contract storage means function
    ///can be declared view
    fn is_zombie_reaper(&self) -> Result<bool, Error> {
        // must be pid 1 and not zombie ourselves *
        return Ok((self.data().pid == FIRST_PID && self.data().state != state.zombie))
    }

    ///Returning a constant does not access storage at all, so
    ///function can be declared pure
    fn systemd_pid(&self) -> Result<u32, Error> {
        // Note that cast is required to change sign from
        // int32 to uint32
        return Ok((FIRST_PID as u32))
    }

    ///Convert celcius to fahrenheit
    fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error> {
        let mut fahrenheit: i32 = celcius * 9 / 5 + 32;
        return Ok(fahrenheit)
    }

    ///Convert fahrenheit to celcius
    fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error> {
        return Ok((fahrenheit - 32) * 5 / 9)
    }

    ///is this number a power-of-two
    fn is_power_of_2(&self, n: u128) -> Result<bool, Error> {
        return Ok(n != 0 && (n & (n - 1)) == 0)
    }

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
        return Ok(base.pow(exp as u32))
    }

    ///returns true if the address is 0
    fn is_address_zero(&self, a: AccountId) -> Result<bool, Error> {
        return Ok(a.is_zero())
    }

    ///reverse the bytes in an array of 8 (endian swap)
    fn byte_8_reverse(&self, input: [u8; 8]) -> Result<[u8; 8], Error> {
        let mut out = Default::default();
        out = ((input << 56) & &hex::decode("ff00_0000_0000_0000"))
            | ((input << 40) & &hex::decode("00ff_0000_0000_0000"))
            | ((input << 24) & &hex::decode("0000_ff00_0000_0000"))
            | ((input << 8) & &hex::decode("0000_00ff_0000_0000"))
            | ((input >> 8) & &hex::decode("0000_0000_ff00_0000"))
            | ((input >> 24) & &hex::decode("0000_0000_00ff_0000"))
            | ((input >> 40) & &hex::decode("0000_0000_0000_ff00"))
            | ((input >> 56) & &hex::decode("0000_0000_0000_00ff"));
        Ok(out)
    }

    fn reap_processes(&mut self) -> Result<(), Error> {
        let mut n: u32 = 0;
        while n < 100 {
            if self._get_pid_state(n)? == state.zombie {
                // reap!
                self.data().reaped += 1;
            }
            n += 1;
        }
        Ok(())
    }

    fn run_queue(&self) -> Result<u16, Error> {
        let mut count: u16 = 0;
        // no initializer means its 0.
        let mut n: u32 = 0;
        loop {
            if self._get_pid_state(n)? == state.waiting {
                count += 1;
            }
            if n < 1000 {
                break
            }
        }
        return Ok(count)
    }

    ///This function does a lot of copying
    fn set_card_1(&mut self, c: card) -> Result<card, Error> {
        let mut previous = Default::default();
        previous = self.data().card_1;
        self.data().card_1 = c;
        Ok(previous)
    }

    ///return the ace of spades
    fn ace_of_spaces(&self) -> Result<card, Error> {
        return Ok(Card {
            s: suit.spades,
            v: value.ace,
        })
    }

    ///score card
    fn score_card(&self, c: card) -> Result<u32, Error> {
        let mut score = Default::default();
        if c.s == suit.hearts {
            if c.v == value.ace {
                score = 14;
            }
            if c.v == value.king {
                score = 13;
            }
            if c.v == value.queen {
                score = 12;
            }
            if c.v == value.jack {
                score = 11;
            }
        }
        // all others score 0
        Ok(score)
    }

    fn state(&self) -> State {
        self.data().state
    }
    fn pid(&self) -> i32 {
        self.data().pid
    }
    fn reaped(&self) -> u32 {
        self.data().reaped
    }
    fn card_1(&self) -> card {
        self.data().card_1
    }
    fn card_2(&self) -> card {
        self.data().card_2
    }
}

pub trait Internal {
    ///This mocks a pid state
    fn _get_pid_state(&self, pid: u64) -> Result<State, Error>;

    ///Overloaded function with different return value!
    fn _get_pid_state(&self) -> Result<u32, Error>;

}

impl<T: Storage<Data>> Internal for T {
    ///This mocks a pid state
    default fn _get_pid_state(&self, pid: u64) -> Result<State, Error> {
        let mut n: u64 = 8;
        let mut i: u16 = 1;
        while i < 10 {
            if (i % 3) == 0 {
                n *= pid / (i as u64);
            } else {
                n /= 3;
            }
            i += 1;
        }
        return Ok(state(n % (state.state_count as u64))?)
    }

    ///Overloaded function with different return value!
    default fn _get_pid_state(&self) -> Result<u32, Error> {
        return Ok(self.data().reaped)
    }

}
