#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod router_event_emitter {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(event)]
    pub struct Amounts {
        amounts: Vec<u128>,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
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
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

    }
}
