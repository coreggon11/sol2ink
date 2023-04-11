#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

#[openbrush::contract]
pub mod flipper {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct flipperContract {
        #[storage_field]
        data: impls::Data,
    }

    impl flipper for flipperContract {}

    impl flipperContract {
        ///Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(initvalue: bool) -> Self {
            let mut instance = Self::default();
            instance.data.value = initvalue;
            instance
        }

    }
}
