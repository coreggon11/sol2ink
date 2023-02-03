#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

/// sliding window oracle that uses observations collected over a window to provide moving price averages in the past
/// `windowSize` with a precision of `windowSize / granularity`
/// note this is a singleton oracle and only needs to be deployed once per desired parameters, which
/// differs from the simple oracle which must be deployed once per pair.
#[openbrush::contract]
pub mod example_sliding_window_oracle {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ExampleSlidingWindowOracleContract {
        #[storage_field]
        data: impls::Data,
    }

    impl ExampleSlidingWindowOracle for ExampleSlidingWindowOracleContract {}

    impl generated::impls::example_sliding_window_oracle::Internal
        for ExampleSlidingWindowOracleContract
    {
    }

    impl ExampleSlidingWindowOracleContract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, window_size: u128, granularity: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                if !(granularity > 1) {
                    return Err(Error::Custom(String::from(
                        "SlidingWindowOracle: GRANULARITY",
                    )))
                };
                if !((instance.data.period_size = window_size / granularity) * granularity
                    == window_size)
                {
                    return Err(Error::Custom(String::from(
                        "SlidingWindowOracle: WINDOW_NOT_EVENLY_DIVISIBLE",
                    )))
                };
                instance.data.factory = factory;
                instance.data.window_size = window_size;
                instance.data.granularity = granularity;
            })
        }

    }
}
