#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.7.0) (token/ERC1155/ERC1155.sol)
/// @dev Implementation of the basic standard multi-token.
/// See https://eips.ethereum.org/EIPS/eip-1155
/// Originally based on code by Enjin: https://github.com/enjin/erc-1155
/// _Available since v3.1._
#[openbrush::contract]
pub mod erc_1155 {
    use erc_1155::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::Vec;
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


    /// @dev Emitted when `value` tokens of token type `id` are transferred from `from` to `to` by `operator`.
    #[ink(event)]
    pub struct TransferSingle {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        id: u128,
        value: u128,
    }

    /// @dev Equivalent to multiple {TransferSingle} events, where `operator`, `from` and `to` are the same for all
    /// transfers.
    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        ids: Vec<u128>,
        values: Vec<u128>,
    }

    /// @dev Emitted when `account` grants or revokes permission to `operator` to transfer their tokens, according to
    /// `approved`.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    /// @dev Emitted when the URI for token type `id` changes to `value`, if it is a non-programmatic URI.
    /// If an {URI} event was emitted for `id`, the standard
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata-extensions[guarantees] that `value` will equal the value
    /// returned by {IERC1155MetadataURI-uri}.
    #[ink(event)]
    pub struct URI {
        value: String,
        #[ink(topic)]
        id: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC1155Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC1155 for ERC1155Contract {}

    impl erc_1155::Internal for ERC1155Contract {
        fn _emit_transfer_single(
            &self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            id: u128,
            value: u128,
        ) {
            self.env().emit_event(TransferSingle {
                operator,
                from,
                to,
                id,
                value,
            });
        }

        fn _emit_transfer_batch(
            &self,
            operator: AccountId,
            from: AccountId,
            to: AccountId,
            ids: Vec<u128>,
            values: Vec<u128>,
        ) {
            self.env().emit_event(TransferBatch {
                operator,
                from,
                to,
                ids,
                values,
            });
        }

        fn _emit_approval_for_all(&self, account: AccountId, operator: AccountId, approved: bool) {
            self.env().emit_event(ApprovalForAll {
                account,
                operator,
                approved,
            });
        }

        fn _emit_uri(&self, value: String, id: u128) {
            self.env().emit_event(URI { value, id });
        }

    }

    impl ERC1155Contract {
        /// @dev See {_setURI}.
        #[ink(constructor)]
        pub fn new(uri: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_uri(uri)?;
            })
        }

    }
}
