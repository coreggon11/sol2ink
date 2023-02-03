// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use openbrush::{
    storage::Mapping,
    traits::AccountId,
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


#[derive(Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoleData {
    members: Mapping<AccountId, bool>,
    admin_role: [u8; 32],
}


#[openbrush::wrapper]
pub type AccessControlRef = dyn AccessControl;

#[openbrush::trait_definition]
pub trait AccessControl {
    /// @dev See {IERC165-supportsInterface}.
    #[ink(message)]
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error>;

    /// @dev Returns `true` if `account` has been granted `role`.
    #[ink(message)]
    fn has_role(&self, role: [u8; 32], account: AccountId) -> Result<bool, Error>;

    /// @dev Returns the admin role that controls `role`. See {grantRole} and
    /// {revokeRole}.
    ///
    /// To change a role's admin, use {_setRoleAdmin}.
    #[ink(message)]
    fn get_role_admin(&self, role: [u8; 32]) -> Result<[u8; 32], Error>;

    /// @dev Grants `role` to `account`.
    ///
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    ///
    /// May emit a {RoleGranted} event.
    #[ink(message)]
    fn grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Revokes `role` from `account`.
    ///
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    ///
    /// Requirements:
    ///
    /// - the caller must have ``role``'s admin role.
    ///
    /// May emit a {RoleRevoked} event.
    #[ink(message)]
    fn revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Revokes `role` from the calling account.
    ///
    /// Roles are often managed via {grantRole} and {revokeRole}: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    ///
    /// If the calling account had been revoked `role`, emits a {RoleRevoked}
    /// event.
    ///
    /// Requirements:
    ///
    /// - the caller must be `account`.
    ///
    /// May emit a {RoleRevoked} event.
    #[ink(message)]
    fn renounce_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

}
