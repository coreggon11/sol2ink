#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

/// example.sol
#[openbrush::contract]
pub mod example {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;

    /// Constants
    pub const BAD_STATE: State = state.zombie;
    pub const FIRST_PID: i32 = 1;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct exampleContract {
        #[storage_field]
        data: impls::Data,
    }

    impl example for exampleContract {}

    impl exampleContract {
        /// Our constructors
        #[ink(constructor)]
        pub fn new(pid: i32) -> Self {
            let mut instance = Self::default();
            instance.data.pid = pid;
            self.reaped = 3;
            self.card_1 = card(value.two, suit.club)?;
            self.card_2 = card {
                s: suit.club,
                v: value.two,
            };
            instance
        }

    }
}
