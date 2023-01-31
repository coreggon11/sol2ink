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
    pub status: Status,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> CommentContract for T {
    fn status(&self) -> Status {
        self.data().status
    }

}

pub trait Internal {
    fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _emit_log(&self, sender: AccountId, message: String, priority: u8, status: Status);

}

impl<T: Storage<Data>> Internal for T {
    default fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        ids = 5;
        if ids < 4 {
            ids = 5;
        } else if ids == 0 {
            ids = 4;
        } else {
            ids = 0;
        }
        if to.is_contract()? {
            if ierc_1155_receiver(to)?
                .on_erc_1155_batch_received(operator, from, ids, amounts, data)?
                .is_err()
            {
                return Err(Error::Custom("Try failed"))
            }
        }
        Ok(())
    }

    default fn _emit_log(&self, _: AccountId, _: String, _: u8, _: Status) {}

}
