// Generated with Sol2Ink v1.1.0
// https://github.com/Supercolony-net/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use ink_prelude::string::String;
use openbrush::{
    modifier_definition,
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub roles: Mapping<[u8; 32], RoleData>,
    pub _reserved: Option<()>,
}

/// @dev Modifier that checks that an account has a specific role. Reverts
/// with a standardized message including the required role.
/// The format of the revert reason is given by the following regular expression:
/// /^AccessControl: account (0x[0-9a-f]{40}) is missing role (0x[0-9a-f]{64})$/
/// _Available since v4.1._
#[modifier_definition]
pub fn only_role<T, F, R>(instance: &mut T, body: F, role: [u8; 32]) -> Result<R, Error>
where
    T: AccessControl,
    F: FnOnce(&mut T) -> Result<R, Error>,
{
    self._check_role(role)?;
    body(instance);
}


impl<T: Storage<Data>> AccessControl for T {
    /// @dev See {IERC165-supportsInterface}.
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
        return Ok(interface_id == i_access_control.interface_id
            || super.supports_interface(interface_id)?)
    }

    /// @dev Returns `true` if `account` has been granted `role`.
    fn has_role(&self, role: [u8; 32], account: AccountId) -> Result<bool, Error> {
        return Ok(self
            .data()
            .roles
            .get(&role)
            .unwrap_or_default()
            .members
            .get(&account)
            .unwrap_or_default())
    }

    /// @dev Returns the admin role that controls `role`. See {grantRole} and
    /// {revokeRole}.
    /// To change a role's admin, use {_setRoleAdmin}.
    fn get_role_admin(&self, role: [u8; 32]) -> Result<[u8; 32], Error> {
        return Ok(self.data().roles.get(&role).unwrap_or_default().admin_role)
    }

    /// @dev Grants `role` to `account`.
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event.
    /// Requirements:
    /// - the caller must have ``role``'s admin role.
    /// May emit a {RoleGranted} event.
    # [modifiers (only_role (self . get_role_admin (role) ?) ?)]
    fn grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        self._grant_role(role, account)?;
        Ok(())
    }

    /// @dev Revokes `role` from `account`.
    /// If `account` had been granted `role`, emits a {RoleRevoked} event.
    /// Requirements:
    /// - the caller must have ``role``'s admin role.
    /// May emit a {RoleRevoked} event.
    # [modifiers (only_role (self . get_role_admin (role) ?) ?)]
    fn revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        self._revoke_role(role, account)?;
        Ok(())
    }

    /// @dev Revokes `role` from the calling account.
    /// Roles are often managed via {grantRole} and {revokeRole}: this function's
    /// purpose is to provide a mechanism for accounts to lose their privileges
    /// if they are compromised (such as when a trusted device is misplaced).
    /// If the calling account had been revoked `role`, emits a {RoleRevoked}
    /// event.
    /// Requirements:
    /// - the caller must be `account`.
    /// May emit a {RoleRevoked} event.
    fn renounce_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        if account != Self::env().caller() {
            return Err(Error::Custom(String::from(
                "AccessControl: can only renounce roles for self",
            )))
        }
        self._revoke_role(role, account)?;
        Ok(())
    }

}

pub trait Internal {
    /// @dev Revert with a standard message if `msg.sender` is missing `role`.
    /// Overriding this function changes the behavior of the {onlyRole} modifier.
    /// Format of the revert message is described in {_checkRole}.
    /// _Available since v4.6._
    fn _check_role(&self, role: [u8; 32]) -> Result<(), Error>;

    /// @dev Revert with a standard message if `account` is missing `role`.
    /// The format of the revert reason is given by the following regular expression:
    ///  /^AccessControl: account (0x[0-9a-f]{40}) is missing role (0x[0-9a-f]{64})$/
    fn _check_role(&self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Grants `role` to `account`.
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event. Note that unlike {grantRole}, this function doesn't perform any
    /// checks on the calling account.
    /// May emit a {RoleGranted} event.
    /// [WARNING]
    /// ====
    /// This function should only be called from the constructor when setting
    /// up the initial roles for the system.
    /// Using this function in any other way is effectively circumventing the admin
    /// system imposed by {AccessControl}.
    /// ====
    /// NOTE: This function is deprecated in favor of {_grantRole}.
    fn _setup_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Sets `adminRole` as ``role``'s admin role.
    /// Emits a {RoleAdminChanged} event.
    fn _set_role_admin(&mut self, role: [u8; 32], admin_role: [u8; 32]) -> Result<(), Error>;

    /// @dev Grants `role` to `account`.
    /// Internal function without access restriction.
    /// May emit a {RoleGranted} event.
    fn _grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    /// @dev Revokes `role` from `account`.
    /// Internal function without access restriction.
    /// May emit a {RoleRevoked} event.
    fn _revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error>;

    fn _emit_role_admin_changed(
        &self,
        role: [u8; 32],
        previous_admin_role: [u8; 32],
        new_admin_role: [u8; 32],
    );

    fn _emit_role_granted(&self, role: [u8; 32], account: AccountId, sender: AccountId);

    fn _emit_role_revoked(&self, role: [u8; 32], account: AccountId, sender: AccountId);

}

impl<T: Storage<Data>> Internal for T {
    /// @dev Revert with a standard message if `msg.sender` is missing `role`.
    /// Overriding this function changes the behavior of the {onlyRole} modifier.
    /// Format of the revert message is described in {_checkRole}.
    /// _Available since v4.6._
    default fn _check_role(&self, role: [u8; 32]) -> Result<(), Error> {
        self._check_role(role, msg.sender)?;
        Ok(())
    }

    /// @dev Revert with a standard message if `account` is missing `role`.
    /// The format of the revert reason is given by the following regular expression:
    ///  /^AccessControl: account (0x[0-9a-f]{40}) is missing role (0x[0-9a-f]{64})$/
    default fn _check_role(&self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        if !self.has_role(role, account)? {
            revert(
                (abi.encode_packed(
                    "AccessControl: account ",
                    strings.to_hex_string(account)?,
                    " is missing role ",
                    strings.to_hex_string((role as u128), 32)?,
                )? as String),
            )?;
        }
        Ok(())
    }

    /// @dev Grants `role` to `account`.
    /// If `account` had not been already granted `role`, emits a {RoleGranted}
    /// event. Note that unlike {grantRole}, this function doesn't perform any
    /// checks on the calling account.
    /// May emit a {RoleGranted} event.
    /// [WARNING]
    /// ====
    /// This function should only be called from the constructor when setting
    /// up the initial roles for the system.
    /// Using this function in any other way is effectively circumventing the admin
    /// system imposed by {AccessControl}.
    /// ====
    /// NOTE: This function is deprecated in favor of {_grantRole}.
    default fn _setup_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        self._grant_role(role, account)?;
        Ok(())
    }

    /// @dev Sets `adminRole` as ``role``'s admin role.
    /// Emits a {RoleAdminChanged} event.
    default fn _set_role_admin(
        &mut self,
        role: [u8; 32],
        admin_role: [u8; 32],
    ) -> Result<(), Error> {
        let mut previous_admin_role: [u8; 32] = self.get_role_admin(role)?;
        self.data().roles.get(&role).unwrap_or_default().admin_role = admin_role;
        self._emit_role_admin_changed(role, previous_admin_role, admin_role);
        Ok(())
    }

    /// @dev Grants `role` to `account`.
    /// Internal function without access restriction.
    /// May emit a {RoleGranted} event.
    default fn _grant_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        if !self.has_role(role, account)? {
            self.data()
                .roles
                .get(&role)
                .unwrap_or_default()
                .members
                .get(&account)
                .unwrap_or_default() = true;
            self._emit_role_granted(role, account, Self::env().caller());
        }
        Ok(())
    }

    /// @dev Revokes `role` from `account`.
    /// Internal function without access restriction.
    /// May emit a {RoleRevoked} event.
    default fn _revoke_role(&mut self, role: [u8; 32], account: AccountId) -> Result<(), Error> {
        if self.has_role(role, account)? {
            self.data()
                .roles
                .get(&role)
                .unwrap_or_default()
                .members
                .get(&account)
                .unwrap_or_default() = false;
            self._emit_role_revoked(role, account, Self::env().caller());
        }
        Ok(())
    }

    default fn _emit_role_admin_changed(&self, _: [u8; 32], _: [u8; 32], _: [u8; 32]) {}

    default fn _emit_role_granted(&self, _: [u8; 32], _: AccountId, _: AccountId) {}

    default fn _emit_role_revoked(&self, _: [u8; 32], _: AccountId, _: AccountId) {}

}
