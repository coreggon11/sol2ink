// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::traits::{
    AccountId,
    AccountIdExt,
    ZERO_ADDRESS,
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

/// Process state
pub enum State {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    Zombie,
    StateCount,
}

/// no initializer means its 0.
/// cards
pub enum Suit {
    Club,
    Diamonds,
    Hearts,
    Spades,
}

pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Card {
    v: Value,
    s: Suit,
}


#[openbrush::wrapper]
pub type exampleRef = dyn example;

#[openbrush::trait_definition]
pub trait example {
    /// Set contract storage
    /// Reading but not writing contract storage means function
    /// can be declared view
    #[ink(message)]
    fn is_zombie_reaper(&self) -> Result<bool, Error>;

    ///must be pid 1 and not zombie ourselves
    /// Returning a constant does not access storage at all, so
    /// function can be declared pure
    #[ink(message)]
    fn systemd_pid(&self) -> Result<u32, Error>;

    /// Note that cast is required to change sign from
    /// int32 to uint32
    ///Convert celcius to fahrenheit
    #[ink(message)]
    fn celcius_2_fahrenheit(&self, celcius: i32) -> Result<i32, Error>;

    ///Convert fahrenheit to celcius
    #[ink(message)]
    fn fahrenheit_2_celcius(&self, fahrenheit: i32) -> Result<i32, Error>;

    ///is this number a power-of-two
    #[ink(message)]
    fn is_power_of_2(&self, n: u128) -> Result<bool, Error>;

    ///calculate the population count (number of set bits) using Brian Kerningham's way
    #[ink(message)]
    fn population_count(&self, n: u128) -> Result<u128, Error>;

    ///calculate the power of base to exp
    #[ink(message)]
    fn power(&self, base: u128, exp: u128) -> Result<u128, Error>;

    ///returns true if the address is 0
    #[ink(message)]
    fn is_address_zero(&self, a: AccountId) -> Result<bool, Error>;

    /// reverse the bytes in an array of 8 (endian swap)
    #[ink(message)]
    fn byte_8_reverse(&self, input: [u8; 8]) -> Result<[u8; 8], Error>;

    #[ink(message)]
    fn reap_processes(&mut self) -> Result<(), Error>;

    /// reap!
    #[ink(message)]
    fn run_queue(&self) -> Result<u16, Error>;

    /// This function does a lot of copying
    #[ink(message)]
    fn set_card_1(&mut self, c: Card) -> Result<Card, Error>;

    ///return the ace of spades
    #[ink(message)]
    fn ace_of_spaces(&self) -> Result<Card, Error>;

    ///score card
    #[ink(message)]
    fn score_card(&self, c: Card) -> Result<u32, Error>;

}
