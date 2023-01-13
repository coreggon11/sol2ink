// Generated with Sol2Ink v1.1.0
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
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC20 for T {}

pub trait Internal {
    fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128);

    fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}

}
