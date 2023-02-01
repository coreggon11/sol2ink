// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::vec::*;
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

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory_v_1: IUniswapV1Factory,
    pub factory: AccountId,
    pub weth: Iweth,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ExampleFlashSwap for T {
    /// gets tokens/WETH via a V2 flash swap, swaps for the ETH/tokens on V1, repays V2, and keeps the rest!
    fn uniswap_v_2_call(
        &mut self,
        sender: AccountId,
        amount_0: u128,
        amount_1: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        let mut path: Vec<AccountId> = vec![AccountId::default(); 2];
        let mut token_0: AccountId = i_uniswap_v_2_pair(Self::env().caller())?.token_0()?;
        let mut token_1: AccountId = i_uniswap_v_2_pair(Self::env().caller())?.token_1()?;
        assert(
            Self::env().caller()
                == uniswap_v_2_library.pair_for(
                    self.data().factory,
                    self.data().token_0,
                    self.data().token_1,
                )?,
        )?;
        assert(amount_0 == 0 || amount_1 == 0)?;
        path[0] = if amount_0 == 0 {
            self.data().token_0
        } else {
            self.data().token_1
        };
        path[1] = if amount_0 == 0 {
            self.data().token_1
        } else {
            self.data().token_0
        };
        amount_token = if self.data().token_0 == AccountId::from(self.data().weth) {
            amount_1
        } else {
            amount_0
        };
        amount_eth = if self.data().token_0 == AccountId::from(self.data().weth) {
            amount_0
        } else {
            amount_1
        };
        assert(
            path[0] == AccountId::from(self.data().weth)
                || path[1] == AccountId::from(self.data().weth),
        )?;
        let mut token: Ierc20 = ierc_20(
            if path[0] == AccountId::from(self.data().weth) {
                path[1]
            } else {
                path[0]
            },
        )?;
        let mut exchange_v_1: IUniswapV1Exchange = i_uniswap_v_1_exchange(
            self.data()
                .factory_v_1
                .get_exchange(AccountId::from(token))?,
        )?;
        if amount_token > 0 {
            (min_eth) = abi.decode(__comment__!(data, (u128)))?;
            token.approve(AccountId::from(exchange_v_1), amount_token)?;
            let mut amount_received: u128 =
                exchange_v_1.token_to_eth_swap_input(amount_token, min_eth, <u128>::from(-1))?;
            let mut amount_required: u128 =
                uniswap_v_2_library.get_amounts_in(self.data().factory, amount_token, path)?[0];
            assert(amount_received > amount_required)?;
            self.data()
                .weth
                .deposit()
                .transferred_value(amount_required)?;
            assert(
                self.data()
                    .weth
                    .transfer(Self::env().caller(), amount_required)?,
            )?;
            (success, _) = sender
                .call(Vec::with_capacity(0))
                .transferred_value(amount_received - amount_required)?;
            assert(success)?;
        } else {
            (min_tokens) = abi.decode(__comment__!(data, (u128)))?;
            self.data().weth.withdraw(amount_eth)?;
            let mut amount_received: u128 = exchange_v_1
                .eth_to_token_swap_input(min_tokens, <u128>::from(-1))
                .transferred_value(amount_eth)?;
            let mut amount_required: u128 =
                uniswap_v_2_library.get_amounts_in(self.data().factory, amount_eth, path)?[0];
            assert(amount_received > amount_required)?;
            assert(token.transfer(Self::env().caller(), amount_required)?)?;
            assert(token.transfer(sender, amount_received - amount_required)?)?;
        }
        Ok(())
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
