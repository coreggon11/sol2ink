#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

/// Copyright (C) 2015, 2016, 2017 Dapphub
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <http://www.gnu.org/licenses/>.
#[openbrush::contract]
pub mod weth_9 {
    use generated::*;
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;


    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        src: AccountId,
        #[ink(topic)]
        guy: AccountId,
        wad: u128,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        src: AccountId,
        #[ink(topic)]
        dst: AccountId,
        wad: u128,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        dst: AccountId,
        wad: u128,
    }

    #[ink(event)]
    pub struct Withdrawal {
        #[ink(topic)]
        src: AccountId,
        wad: u128,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct WETH9Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl WETH9 for WETH9Contract {}
    impl generated::impls::weth_9::Internal for WETH9Contract {

        fn _emit_approval(&self, src: AccountId, guy: AccountId, wad: u128) {
            self.env().emit_event(Approval { src, guy, wad });
        }

        fn _emit_transfer(&self, src: AccountId, dst: AccountId, wad: u128) {
            self.env().emit_event(Transfer { src, dst, wad });
        }

        fn _emit_deposit(&self, dst: AccountId, wad: u128) {
            self.env().emit_event(Deposit { dst, wad });
        }

        fn _emit_withdrawal(&self, src: AccountId, wad: u128) {
            self.env().emit_event(Withdrawal { src, wad });
        }

    }

    impl WETH9Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            self.name = "Wrapped Ether";
            self.symbol = "WETH";
            self.decimals = 18;
            instance
        }

    }
}
