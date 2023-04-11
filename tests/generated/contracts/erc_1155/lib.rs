#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/727-Ventures/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC1155/ERC1155.sol)
/// @dev Implementation of the basic standard multi-token.
/// See https://eips.ethereum.org/EIPS/eip-1155
/// Originally based on code by Enjin: https://github.com/enjin/erc-1155
///
/// _Available since v3.1._
#[openbrush::contract]
pub mod erc_1155 {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ERC1155Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl ERC1155 for ERC1155Contract {}

    impl Context for ERC1155Contract {}

    impl ERC165 for ERC1155Contract {}

    impl IERC1155 for ERC1155Contract {}

    impl IERC1155MetadataURI for ERC1155Contract {}

    impl ERC1155Contract {
        /// @dev See {_setURI}.
        #[ink(constructor)]
        pub fn new(uri: String) -> Self {
            let mut instance = Self::default();
            instance._set_uri(uri)?;
            instance
        }

    }
}
