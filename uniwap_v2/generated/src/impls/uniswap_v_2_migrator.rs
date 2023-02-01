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
    pub router: IUniswapV2Router01,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> UniswapV2Migrator for T {
    fn migrate(
        &mut self,
        token: AccountId,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        let mut exchange_v_1: IUniswapV1Exchange =
            i_uniswap_v_1_exchange(self.data().factory_v_1.get_exchange(token)?)?;
        let mut liquidity_v_1: u128 = exchange_v_1.balance_of(Self::env().caller())?;
        if !(exchange_v_1.transfer_from(
            Self::env().caller(),
            Self::env().account_id(),
            liquidity_v_1,
        )?) {
            return Err(Error::Custom(String::from("TRANSFER_FROM_FAILED")))
        };
        (amount_ethv_1, amount_token_v_1) =
            exchange_v_1.remove_liquidity(liquidity_v_1, 1, 1, <u128>::from(-1))?;
        transfer_helper.safe_approve(
            token,
            AccountId::from(self.data().router),
            amount_token_v_1,
        )?;
        (amount_token_v_2, amount_ethv_2, _) = self
            .data()
            .router
            .add_liquidity_eth(
                token,
                amount_token_v_1,
                amount_token_min,
                amount_eth_min,
                to,
                deadline,
            )
            .transferred_value(amount_ethv_1)?;
        if amount_token_v_1 > amount_token_v_2 {
            transfer_helper.safe_approve(token, AccountId::from(self.data().router), 0)?;
            transfer_helper.safe_transfer(
                token,
                Self::env().caller(),
                amount_token_v_1 - amount_token_v_2,
            )?;
        } else if amount_ethv_1 > amount_ethv_2 {
            transfer_helper
                .safe_transfer_eth(Self::env().caller(), amount_ethv_1 - amount_ethv_2)?;
        }
        Ok(())
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
