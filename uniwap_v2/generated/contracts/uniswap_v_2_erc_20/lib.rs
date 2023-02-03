#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_erc_20 {
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

    pub const NAME: String = "Uniswap V2";
    pub const SYMBOL: String = "UNI-V2";
    pub const DECIMALS: u8 = 18;
    /// keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    pub const PERMIT_TYPEHASH: [u8; 32] =
        &hex::decode("0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9");

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: u128,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct UniswapV2ERC20Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2ERC20 for UniswapV2ERC20Contract {}

    impl uniswap_v_2_erc_20::Internal for UniswapV2ERC20Contract {
        fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
        }

        fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128) {
            self.env().emit_event(Transfer { from, to, value });
        }

    }

    impl IUniswapV2ERC20 for UniswapV2ERC20Contract {}

    impl UniswapV2ERC20Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                __comment__!("Assembly block here. Parsing assembly is not implemented yet");
                instance . data () . domain_separator = keccak_256 (abi . encode (keccak_256 ("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)") ? , keccak_256 (Vec :: < u8 > :: from (NAME)) ? , keccak_256 (Vec :: < u8 > :: from ("1")) ? , chain_id , instance . env () . account_id ()) ?) ? ;
            })
        }

    }
}
