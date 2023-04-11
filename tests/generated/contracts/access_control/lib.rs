#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

/// SPDX-License-Identifier: MIT
/// OpenZeppelin Contracts (last updated v4.8.0) (access/AccessControl.sol)
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
/// ```solidity
/// bytes32 public constant MY_ROLE = keccak256("MY_ROLE");
/// ```
///
/// Roles can be used to represent a set of permissions. To restrict access to a
/// function call, use {hasRole}:
///
/// ```solidity
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
    use ink::lang::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::traits::Storage;

    pub const DEFAULT_ADMIN_ROLE: [u8; 32] = &hex::decode("0x00");

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AccessControlContract {
        #[storage_field]
        data: impls::Data,
    }

    impl AccessControl for AccessControlContract {}

    impl Context for AccessControlContract {}

    impl IAccessControl for AccessControlContract {}

    impl ERC165 for AccessControlContract {}

    impl AccessControlContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance
        }

    }
}
