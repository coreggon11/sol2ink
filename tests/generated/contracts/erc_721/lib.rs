#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC721/ERC721.sol)
/// @dev Implementation of https://eips.ethereum.org/EIPS/eip-721[ERC721] Non-Fungible Token Standard, including
/// the Metadata extension, but not including the Enumerable extension, which is available separately as
/// {ERC721Enumerable}.
#[openbrush::contract]
pub mod erc_721 {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ERC721Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC721 for ERC721Contract {}

    impl Context for ERC721Contract {}

    impl ERC165 for ERC721Contract {}

    impl IERC721 for ERC721Contract {}

    impl IERC721Metadata for ERC721Contract {}

    impl ERC721Contract {
        /// @dev Initializes the contract by setting a `name` and a `symbol` to the token collection.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            instance.data.name = name;
            instance.data.symbol = symbol;
            instance
        }

    }
}
