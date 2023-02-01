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
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub balance_of: Mapping<AccountId, u128>,
    pub allowance: Mapping<(AccountId, AccountId), u128>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> WETH9 for T {
    /// function() public payable {
    ///     deposit();
    /// }
    fn deposit(&mut self) -> Result<(), Error> {
        let new_value = self
            .data()
            .balance_of
            .get(&(Self::env().caller()))
            .unwrap_or_default()
            + Self::env().transferred_value();
        self.data()
            .balance_of
            .insert(&(Self::env().caller()), &new_value);
        self._emit_deposit(Self::env().caller(), Self::env().transferred_value());
        Ok(())
    }

    fn withdraw(&mut self, wad: u128) -> Result<(), Error> {
        if !(self
            .data()
            .balance_of
            .get(&Self::env().caller())
            .unwrap_or_default()
            >= wad)
        {
            return Err(Error::Custom(String::from("")))
        };
        let new_value = self
            .data()
            .balance_of
            .get(&(Self::env().caller()))
            .unwrap_or_default()
            - wad;
        self.data()
            .balance_of
            .insert(&(Self::env().caller()), &new_value);
        Self::env().caller().transfer(wad)?;
        self._emit_withdrawal(Self::env().caller(), wad);
        Ok(())
    }

    fn total_supply(&self) -> Result<u128, Error> {
        return Ok(Self::env().account_id().balance)
    }

    fn approve(&mut self, guy: AccountId, wad: u128) -> Result<bool, Error> {
        self.data()
            .allowance
            .insert(&(Self::env().caller(), guy), &wad);
        self._emit_approval(Self::env().caller(), guy, wad);
        return Ok(true)
    }

    /// be a good blockchain citizen, reset allowance to 0
    fn transfer(&mut self, dst: AccountId, wad: u128) -> Result<bool, Error> {
        return Ok(self.transfer_from(Self::env().caller(), dst, wad)?)
    }

    /// addLiquidityETH guarantees that all of amountETHV1 or amountTokenV1 will be used, hence this else is safe
    fn transfer_from(&mut self, src: AccountId, dst: AccountId, wad: u128) -> Result<bool, Error> {
        if !(self.data().balance_of.get(&src).unwrap_or_default() >= wad) {
            return Err(Error::Custom(String::from("")))
        };
        if src != Self::env().caller()
            && self
                .data()
                .allowance
                .get(&(src, Self::env().caller()))
                .unwrap_or_default()
                != <u128>::from(-1)
        {
            if !(self
                .data()
                .allowance
                .get(&(src, Self::env().caller()))
                .unwrap_or_default()
                >= wad)
            {
                return Err(Error::Custom(String::from("")))
            };
            let new_value = self
                .data()
                .allowance
                .get(&(src, Self::env().caller()))
                .unwrap_or_default()
                - wad;
            self.data()
                .allowance
                .insert(&(src, Self::env().caller()), &new_value);
        }
        let new_value = self.data().balance_of.get(&(src)).unwrap_or_default() - wad;
        self.data().balance_of.insert(&(src), &new_value);
        let new_value = self.data().balance_of.get(&(dst)).unwrap_or_default() + wad;
        self.data().balance_of.insert(&(dst), &new_value);
        self._emit_transfer(src, dst, wad);
        return Ok(true)
    }

    fn name(&self) -> String {
        self.data().name
    }

    fn symbol(&self) -> String {
        self.data().symbol
    }

    fn decimals(&self) -> u8 {
        self.data().decimals
    }

    fn balance_of(&self) -> Mapping<AccountId, u128> {
        self.data().balance_of
    }

    fn allowance(&self) -> Mapping<(AccountId, AccountId), u128> {
        self.data().allowance
    }

}

pub trait Internal {
    fn _emit_approval(&self, src: AccountId, guy: AccountId, wad: u128);

    fn _emit_transfer(&self, src: AccountId, dst: AccountId, wad: u128);

    fn _emit_deposit(&self, dst: AccountId, wad: u128);

    fn _emit_withdrawal(&self, src: AccountId, wad: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_deposit(&self, _: AccountId, _: u128) {}

    default fn _emit_withdrawal(&self, _: AccountId, _: u128) {}

}
