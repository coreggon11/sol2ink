// Generated with Sol2Ink v2.0.0-beta
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub balances: Mapping<AccountId, u128>,
    pub allowances: Mapping<(AccountId, AccountId), u128>,
    pub total_supply: u128,
    pub name: String,
    pub symbol: String,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC20 for T {
    fn name(&self) -> Result<String, Error> {}

    fn symbol(&self) -> Result<String, Error> {}

    fn decimals(&self) -> Result<u8, Error> {}

    fn total_supply(&self) -> Result<u128, Error> {}

    fn balance_of(&self, account: AccountId) -> Result<u128, Error> {}

    fn transfer(&mut self, to: AccountId, amount: u128) -> Result<bool, Error> {}

    fn allowance(&self, owner: AccountId, spender: AccountId) -> Result<u128, Error> {}

    fn approve(&mut self, spender: AccountId, amount: u128) -> Result<bool, Error> {}

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<bool, Error> {
    }

    fn increase_allowance(&mut self, spender: AccountId, added_value: u128) -> Result<bool, Error> {
    }

    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        subtracted_value: u128,
    ) -> Result<bool, Error> {
    }

}

pub trait Internal {
    fn _transfer(&mut self, from: AccountId, to: AccountId, amount: u128) -> Result<(), Error>;

    fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error>;

    fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error>;

    fn _approve(&mut self, owner: AccountId, spender: AccountId, amount: u128)
        -> Result<(), Error>;

    fn _spend_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error>;

    fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128);

    fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _mint(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
        Ok(())
    }

    default fn _burn(&mut self, account: AccountId, amount: u128) -> Result<(), Error> {
        Ok(())
    }

    default fn _approve(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _spend_allowance(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        if current_allowance != u128::max {}
        Ok(())
    }

    default fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}

}
