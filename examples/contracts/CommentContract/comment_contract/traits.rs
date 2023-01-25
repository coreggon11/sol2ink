// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

use ink_prelude::vec::*;
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

/// pending comment1
/// pending comment2
///shipped comment1
/// shipped comment2
///rejected comment
///canceled comment1
/// canceled comment2
pub enum Status {
    Pending,
    Shipped,
    Accepted,
    Rejected,
    Canceled,
}


/// MULTILINE_COMMENT::members
/// COMMENT::members
///COMMENT::adminRole
/// MULTILINE_COMMENT::adminRole
#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData1 {
    members: Mapping<AccountId, bool>,
    admin_role: [u8; 32],
}

///
///ULTILINE_COMMENT::members
/// COMMENT::members
/// COMMENT::adminRole
#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData2 {
    members: Mapping<AccountId, bool>,
    admin_role: [u8; 32],
}

/// COMMENT::members
///                 COMMENT::adminRole
#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData3 {
    members: Mapping<AccountId, bool>,
    admin_role: [u8; 32],
}

/// MULTILINE_COMMENT::members
/// MULTILINE_COMMENT::adminRole
#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData4 {
    members: Mapping<AccountId, bool>,
    admin_role: [u8; 32],
}


#[openbrush::wrapper]
pub type CommentContractRef = dyn CommentContract;

#[openbrush::trait_definition]
pub trait CommentContract {
    #[ink(message)]
    fn status(&self) -> Status;

}
