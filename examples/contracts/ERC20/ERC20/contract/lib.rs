#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.6.0) (token/ERC20/ERC20.sol)
/// @dev Implementation of the {IERC20} interface.
/// This implementation is agnostic to the way tokens are created. This means
/// that a supply mechanism has to be added in a derived contract using {_mint}.
/// For a generic mechanism see {ERC20PresetMinterPauser}.
/// TIP: For a detailed writeup see our guide
/// https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How
/// to implement supply mechanisms].
/// We have followed general OpenZeppelin Contracts guidelines: functions revert
/// instead returning `false` on failure. This behavior is nonetheless
/// conventional and does not conflict with the expectations of ERC20
/// applications.
/// Additionally, an {Approval} event is emitted on calls to {transferFrom}.
/// This allows applications to reconstruct the allowance for all accounts just
/// by listening to said events. Other implementations of the EIP may not emit
/// these events, as it isn't required by the specification.
/// Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
/// functions have been added to mitigate the well-known issues around setting
/// allowances. See {IERC20-approve}.
#[openbrush::contract]
pub mod erc_20 {
    use erc_20::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
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


    /// @dev Emitted when `value` tokens are moved from one account (`from`) to
    /// another (`to`).
    /// Note that `value` may be zero.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: u128,
    }

    /// @dev Emitted when the allowance of a `spender` for an `owner` is set by
    /// a call to {approve}. `value` is the new allowance.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC20Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC20 for ERC20Contract {}

    impl erc_20::Internal for ERC20Contract {
        fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128) {
            self.env().emit_event(Transfer { from, to, value });
        }

        fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
        }

    }

    impl ERC20Contract {
        /// @dev Sets the values for {name} and {symbol}.
        /// The default value of {decimals} is 18. To select a different value for
        /// {decimals} you should overload it.
        /// All two of these values are immutable: they can only be set once during
        /// construction.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().name = name;
                instance.data().symbol = symbol;
            })
        }

    }
}
