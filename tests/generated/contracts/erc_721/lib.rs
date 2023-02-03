#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC721/ERC721.sol)
/// @dev Implementation of https://eips.ethereum.org/EIPS/eip-721[ERC721] Non-Fungible Token Standard, including
/// the Metadata extension, but not including the Enumerable extension, which is available separately as
/// {ERC721Enumerable}.
#[openbrush::contract]
pub mod erc_721 {
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
    pub struct ERC721Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC721 for ERC721Contract {}

    impl erc_721::Internal for ERC721Contract {}

    impl Context for ERC721Contract {}

    impl ERC165 for ERC721Contract {}

    impl IERC721 for ERC721Contract {}

    impl IERC721Metadata for ERC721Contract {}

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
