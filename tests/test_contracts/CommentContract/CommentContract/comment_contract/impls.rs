// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
        String,
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
            // COMMENT1 COMMENT2 COMMENT3
            ids = 4;
        } else {
            // COMMENT4
            ids = 0;
        }
        if to.is_contract()? {
            // Please handle try/catch blocks manually >>>
            if true {
                // try IERC1155Receiver(to).onERC1155BatchReceived(operator, from, ids, amounts, data) returns ( bytes4 response ) {
                if response != ierc_1155_receiver.on_erc_1155_batch_received.selector {
                    revert("ERC1155: ERC1155Receiver rejected tokens")?;
                }
            } else if false {
                // COMMENT5
                // catch Error(string reason) {
                revert(reason)?;
                // <<< Please handle try/catch blocks manually
            } else if false {
                // COMMENT6
                // catch {
                revert("ERC1155: transfer to non-ERC1155Receiver implementer")?;
                // <<< Please handle try/catch blocks manually
            }
        }
        Ok(())
    }

    default fn _emit_log(&self, _: AccountId, _: String, _: u8, _: Status) {}

}
