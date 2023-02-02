#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod flipper {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::*;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        storage::Mapping,
        traits::{
            AccountId,
            AccountIdExt,
            Storage,
            String,
            ZERO_ADDRESS,
        },
    };
    use scale::{
        Decode,
        Encode,
    };


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct flipperContract {
        #[storage_field]
        data: impls::Data,
    }

    impl flipper for flipperContract {}

    impl flipper::Internal for flipperContract {}

    impl flipperContract {
        ///Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(initvalue: bool) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().value = initvalue;
            })
        }

    }
}
