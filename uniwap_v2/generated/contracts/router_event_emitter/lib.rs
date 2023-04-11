#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod router_event_emitter {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(event)]
    pub struct Amounts {
        amounts: Vec<u128>,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct RouterEventEmitterContract {
        #[storage_field]
        data: impls::Data,
    }

    impl RouterEventEmitter for RouterEventEmitterContract {}
    impl generated::impls::router_event_emitter::Internal for RouterEventEmitterContract {

        fn _emit_amounts(&self, amounts: Vec<u128>) {
            self.env().emit_event(Amounts { amounts });
        }

    }

    impl RouterEventEmitterContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
        }

    }
}
