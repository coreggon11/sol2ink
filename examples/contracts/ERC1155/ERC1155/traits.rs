// Generated with Sol2Ink v1.0.0
// https://github.com/Supercolony-net/sol2ink

use ink_prelude::{
    string::String,
    vec::Vec,
};
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        ZERO_ADDRESS,
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



#[openbrush::wrapper]
pub type ERC1155Ref = dyn ERC1155;

#[openbrush::trait_definition]
pub trait ERC1155 {
    /// @dev See {IERC165-supportsInterface}.
    #[ink(message)]
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error>;

    /// @dev See {IERC1155MetadataURI-uri}.
    /// This implementation returns the same URI for *all* token types. It relies
    /// on the token type ID substitution mechanism
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
    /// Clients calling this function must replace the `\{id\}` substring with the
    /// actual token type ID.
    #[ink(message)]
    fn uri(&self) -> Result<String, Error>;

    /// @dev See {IERC1155-balanceOf}.
    /// Requirements:
    /// - `account` cannot be the zero address.
    #[ink(message)]
    fn balance_of(&self, account: AccountId, id: u128) -> Result<u128, Error>;

    /// @dev See {IERC1155-balanceOfBatch}.
    /// Requirements:
    /// - `accounts` and `ids` must have the same length.
    #[ink(message)]
    fn balance_of_batch(
        &self,
        accounts: Vec<AccountId>,
        ids: Vec<u128>,
    ) -> Result<Vec<u128>, Error>;

    /// @dev See {IERC1155-setApprovalForAll}.
    #[ink(message)]
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error>;

    /// @dev See {IERC1155-isApprovedForAll}.
    #[ink(message)]
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> Result<bool, Error>;

    /// @dev See {IERC1155-safeTransferFrom}.
    #[ink(message)]
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev See {IERC1155-safeBatchTransferFrom}.
    #[ink(message)]
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    #[ink(message)]
    fn balances(&self) -> Mapping<(u128, AccountId), u128>;
    #[ink(message)]
    fn operator_approvals(&self) -> Mapping<(AccountId, AccountId), bool>;
    #[ink(message)]
    fn uri(&self) -> String;
}
