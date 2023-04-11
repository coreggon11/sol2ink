// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use crate::{
    impls,
    traits::*,
};
pub use openbrush::traits::AccountId;
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> RouterEventEmitter for T {
    fn swap_exact_tokens_for_tokens(
        &mut self,
        router: AccountId,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        (success, return_data) = router.delegatecall(
            abi.encode_with_selector(
                i_uniswap_v_2_router_01(router)?
                    .swap_exact_tokens_for_tokens
                    .selector,
                amount_in,
                amount_out_min,
                path,
                to,
                deadline,
            )?,
        )?;
        assert(success)?;
        self._emit_amounts(abi.decode(__comment__ ! (return_data , (u128 [])))?);
        Ok(())
    }

    fn swap_tokens_for_exact_tokens(
        &mut self,
        router: AccountId,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        (success, return_data) = router.delegatecall(
            abi.encode_with_selector(
                i_uniswap_v_2_router_01(router)?
                    .swap_tokens_for_exact_tokens
                    .selector,
                amount_out,
                amount_in_max,
                path,
                to,
                deadline,
            )?,
        )?;
        assert(success)?;
        self._emit_amounts(abi.decode(__comment__ ! (return_data , (u128 [])))?);
        Ok(())
    }

    fn swap_exact_eth_for_tokens(
        &mut self,
        router: AccountId,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        (success, return_data) = router.delegatecall(
            abi.encode_with_selector(
                i_uniswap_v_2_router_01(router)?
                    .swap_exact_eth_for_tokens
                    .selector,
                amount_out_min,
                path,
                to,
                deadline,
            )?,
        )?;
        assert(success)?;
        self._emit_amounts(abi.decode(__comment__ ! (return_data , (u128 [])))?);
        Ok(())
    }

    fn swap_tokens_for_exact_eth(
        &mut self,
        router: AccountId,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        (success, return_data) = router.delegatecall(
            abi.encode_with_selector(
                i_uniswap_v_2_router_01(router)?
                    .swap_tokens_for_exact_eth
                    .selector,
                amount_out,
                amount_in_max,
                path,
                to,
                deadline,
            )?,
        )?;
        assert(success)?;
        self._emit_amounts(abi.decode(__comment__ ! (return_data , (u128 [])))?);
        Ok(())
    }

    fn swap_exact_tokens_for_eth(
        &mut self,
        router: AccountId,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        (success, return_data) = router.delegatecall(
            abi.encode_with_selector(
                i_uniswap_v_2_router_01(router)?
                    .swap_exact_tokens_for_eth
                    .selector,
                amount_in,
                amount_out_min,
                path,
                to,
                deadline,
            )?,
        )?;
        assert(success)?;
        self._emit_amounts(abi.decode(__comment__ ! (return_data , (u128 [])))?);
        Ok(())
    }

    fn swap_eth_for_exact_tokens(
        &mut self,
        router: AccountId,
        amount_out: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        (success, return_data) = router.delegatecall(
            abi.encode_with_selector(
                i_uniswap_v_2_router_01(router)?
                    .swap_eth_for_exact_tokens
                    .selector,
                amount_out,
                path,
                to,
                deadline,
            )?,
        )?;
        assert(success)?;
        self._emit_amounts(abi.decode(__comment__ ! (return_data , (u128 [])))?);
        Ok(())
    }

}

pub trait Internal {
    fn _emit_amounts(&self, amounts: Vec<u128>);

}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_amounts(&self, _: Vec<u128>) {}

}
