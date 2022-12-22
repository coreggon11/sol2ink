#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

///SPDX-License-Identifier: MIT
///OpenZeppelin Contracts (last updated v4.7.0) (token/ERC721/ERC721.sol)
/// @dev Implementation of https://eips.ethereum.org/EIPS/eip-721[ERC721] Non-Fungible Token Standard, including
/// the Metadata extension, but not including the Enumerable extension, which is available separately as
/// {ERC721Enumerable}.
#[openbrush::contract]
pub mod erc_721 {
    use erc_721::*;
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


    /// @dev Emitted when `tokenId` token is transferred from `from` to `to`.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        token_id: u128,
    }

    /// @dev Emitted when `owner` enables `approved` to manage the `tokenId` token.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        approved: AccountId,
        #[ink(topic)]
        token_id: u128,
    }

    /// @dev Emitted when `owner` enables or disables (`approved`) `operator` to manage all of its assets.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ERC721Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC721 for ERC721Contract {}

    impl erc_721::Internal for ERC721Contract {
        fn _emit_transfer(&self, from: AccountId, to: AccountId, token_id: u128) {
            self.env().emit_event(Transfer { from, to, token_id });
        }

        fn _emit_approval(&self, owner: AccountId, approved: AccountId, token_id: u128) {
            self.env().emit_event(Approval {
                owner,
                approved,
                token_id,
            });
        }

        fn _emit_approval_for_all(&self, owner: AccountId, operator: AccountId, approved: bool) {
            self.env().emit_event(ApprovalForAll {
                owner,
                operator,
                approved,
            });
        }

    }

    impl ERC721Contract {
        /// @dev Initializes the contract by setting a `name` and a `symbol` to the token collection.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data().name = name;
                instance.data().symbol = symbol;
            })
        }

    }
}
