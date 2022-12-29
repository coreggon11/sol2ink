// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

use ink_prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        String,
    },
};
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}

pub enum Status {
    /// pending comment1
    /// pending comment2
    Pending,
    /// shipped comment1
    /// shipped comment2
    Shipped,
    Accepted,
    /// rejected comment
    Rejected,
    /// canceled comment1
    /// canceled comment2
    Canceled,
}


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData1 {
    /// MULTILINE_COMMENT::members
    /// COMMENT::members
    members: Mapping<AccountId, bool>,
    /// COMMENT::adminRole
    /// MULTILINE_COMMENT::adminRole
    admin_role: [u8; 32],
}

#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData2 {
    /// MULTILINE_COMMENT::members
    /// COMMENT::members
    members: Mapping<AccountId, bool>,
    admin_role: [u8; 32],
}

#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData3 {
    /// COMMENT::members
    members: Mapping<AccountId, bool>,
    /// COMMENT::adminRole
    admin_role: [u8; 32],
}

#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData4 {
    /// MULTILINE_COMMENT::members
    members: Mapping<AccountId, bool>,
    /// MULTILINE_COMMENT::adminRole
    admin_role: [u8; 32],
}


#[openbrush::wrapper]
pub type CommentContractRef = dyn CommentContract;

#[openbrush::trait_definition]
pub trait CommentContract {
    #[ink(message)]
    fn status(&self) -> Status;

}
