#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

///example.sol
#[openbrush::contract]
pub mod example {
    use example::impls::example::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        AccountId,
        AccountIdExt,
        Storage,
        ZERO_ADDRESS,
    };
    use scale::{
        Decode,
        Encode,
    };

    ///Constants
    pub const BAD_STATE: State = state.zombie;
    pub const FIRST_PID: i32 = 1;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct exampleContract {
        #[storage_field]
        data: example::Data,
    }

    impl example for exampleContract {}

    impl example::Internal for exampleContract {}

    impl exampleContract {
        ///Our constructors
        #[ink(constructor)]
        pub fn new(pid: i32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                // Set contract storage
                instance.data().pid = pid;
                self.reaped = 3;
                self.card_1 = card(value.two, suit.club)?;
                self.card_2 = Card {
                    s: suit.club,
                    v: value.two,
                };
            })
        }

    }
}
