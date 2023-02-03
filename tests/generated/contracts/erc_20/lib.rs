#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC20/ERC20.sol)
/// @dev Implementation of the {IERC20} interface.
///
/// This implementation is agnostic to the way tokens are created. This means
/// that a supply mechanism has to be added in a derived contract using {_mint}.
/// For a generic mechanism see {ERC20PresetMinterPauser}.
///
/// TIP: For a detailed writeup see our guide
/// https://forum.openzeppelin.com/t/how-to-implement-erc20-supply-mechanisms/226[How
/// to implement supply mechanisms].
///
/// The default value of {decimals} is 18. To change this, you should override
/// this function so it returns a different value.
///
/// We have followed general OpenZeppelin Contracts guidelines: functions revert
/// instead returning `false` on failure. This behavior is nonetheless
/// conventional and does not conflict with the expectations of ERC20
/// applications.
///
/// Additionally, an {Approval} event is emitted on calls to {transferFrom}.
/// This allows applications to reconstruct the allowance for all accounts just
/// by listening to said events. Other implementations of the EIP may not emit
/// these events, as it isn't required by the specification.
///
/// Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
/// functions have been added to mitigate the well-known issues around setting
/// allowances. See {IERC20-approve}.
#[openbrush::contract]
pub mod erc_20 {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC20Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC20 for ERC20Contract {}

    impl generated::impls::erc_20::Internal for ERC20Contract {}

    impl Context for ERC20Contract {}

    impl IERC20 for ERC20Contract {}

    impl IERC20Metadata for ERC20Contract {}

    impl ERC20Contract {
        /// @dev Sets the values for {name} and {symbol}.
        ///
        /// All two of these values are immutable: they can only be set once during
        /// construction.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.name = name;
                instance.data.symbol = symbol;
            })
        }

    }
}
