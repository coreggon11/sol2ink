#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.7.0) (access/AccessControl.sol)
/// 3 ways to initialize a struct
/// - calling it like a function
/// key value mapping
/// initialize an empty struct and then update it
/// completed initialized to false
/// @dev Contract module that allows children to implement role-based access
/// control mechanisms. This is a lightweight version that doesn't allow enumerating role
/// members except through off-chain means by accessing the contract event logs. Some
/// applications may benefit from on-chain enumerability, for those cases see
/// {AccessControlEnumerable}.
///
/// Roles are referred to by their `bytes32` identifier. These should be exposed
/// in the external API and be unique. The best way to achieve this is by
/// using `public constant` hash digests:
///
/// ```
/// bytes32 public constant MY_ROLE = keccak256("MY_ROLE");
/// ```
///
/// Roles can be used to represent a set of permissions. To restrict access to a
/// function call, use {hasRole}:
///
/// ```
/// function foo() public {
///     require(hasRole(MY_ROLE, msg.sender));
///     ...
/// }
/// ```
///
/// Roles can be granted and revoked dynamically via the {grantRole} and
/// {revokeRole} functions. Each role has an associated admin role, and only
/// accounts that have a role's admin role can call {grantRole} and {revokeRole}.
///
/// By default, the admin role for all roles is `DEFAULT_ADMIN_ROLE`, which means
/// that only accounts with this role will be able to grant or revoke other
/// roles. More complex role relationships can be created by using
/// {_setRoleAdmin}.
///
/// WARNING: The `DEFAULT_ADMIN_ROLE` is also its own admin: it has permission to
/// grant and revoke this role. Extra precautions should be taken to secure
/// accounts that have been granted it.
#[openbrush::contract]
pub mod access_control {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_prelude::vec::*;
    use ink_storage::traits::SpreadAllocate;
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
    use scale::{
        Decode,
        Encode,
    };

    pub const DEFAULT_ADMIN_ROLE: [u8; 32] = &hex::decode("0x00");

    /// @dev Emitted when `newAdminRole` is set as ``role``'s admin role, replacing `previousAdminRole`
    ///
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// {RoleAdminChanged} not being emitted signaling this.
    ///
    /// _Available since v3.1._
    #[ink(event)]
    pub struct RoleAdminChanged {
        #[ink(topic)]
        role: [u8; 32],
        #[ink(topic)]
        previous_admin_role: [u8; 32],
        #[ink(topic)]
        new_admin_role: [u8; 32],
    }

    /// @dev Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call, an admin role
    /// bearer except when using {AccessControl-_setupRole}.
    #[ink(event)]
    pub struct RoleGranted {
        #[ink(topic)]
        role: [u8; 32],
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        sender: AccountId,
    }

    /// @dev Emitted when `account` is revoked `role`.
    ///
    /// `sender` is the account that originated the contract call:
    ///   - if using `revokeRole`, it is the admin role bearer
    ///   - if using `renounceRole`, it is the role bearer (i.e. `account`)
    #[ink(event)]
    pub struct RoleRevoked {
        #[ink(topic)]
        role: [u8; 32],
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        sender: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct AccessControlContract {
        #[storage_field]
        data: impls::Data,
    }

    impl AccessControl for AccessControlContract {}

    impl access_control::Internal for AccessControlContract {
        fn _emit_role_admin_changed(
            &self,
            role: [u8; 32],
            previous_admin_role: [u8; 32],
            new_admin_role: [u8; 32],
        ) {
            self.env().emit_event(RoleAdminChanged {
                role,
                previous_admin_role,
                new_admin_role,
            });
        }

        fn _emit_role_granted(&self, role: [u8; 32], account: AccountId, sender: AccountId) {
            self.env().emit_event(RoleGranted {
                role,
                account,
                sender,
            });
        }

        fn _emit_role_revoked(&self, role: [u8; 32], account: AccountId, sender: AccountId) {
            self.env().emit_event(RoleRevoked {
                role,
                account,
                sender,
            });
        }

    }

    impl AccessControlContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

    }
}
