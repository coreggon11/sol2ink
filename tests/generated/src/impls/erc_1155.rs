// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use crate::{
    impls,
    traits::*,
};
pub use ink::prelude::vec::*;
use openbrush::traits::Storage;
pub use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// Mapping from token ID to account balances
    pub balances: Mapping<(u128, AccountId), u128>,
    /// Mapping from account to operator approvals
    pub operator_approvals: Mapping<(AccountId, AccountId), bool>,
    /// Used as the URI for all token types by relying on ID substitution, e.g. https://token-cdn-domain/{id}.json
    pub uri: String,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC1155 for T {
    /// @dev See {IERC165-supportsInterface}.
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
        return Ok(interface_id == type_of(ierc_1155)?.interface_id
            || interface_id == type_of(ierc_1155_metadata_uri)?.interface_id
            || super.supports_interface(interface_id)?)
    }

    /// @dev See {IERC1155MetadataURI-uri}.
    ///
    /// This implementation returns the same URI for *all* token types. It relies
    /// on the token type ID substitution mechanism
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
    ///
    /// Clients calling this function must replace the `\{id\}` substring with the
    /// actual token type ID.
    fn uri(&self, _: u128) -> Result<String, Error> {
        return Ok(self.data().uri)
    }

    /// @dev See {IERC1155-balanceOf}.
    ///
    /// Requirements:
    ///
    /// - `account` cannot be the zero address.
    fn balance_of(&self, account: AccountId, id: u128) -> Result<u128, Error> {
        if !(account != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: address zero is not a valid owner",
            )))
        };
        return Ok(self.data().balances.get(&(id, account)).unwrap_or_default())
    }

    /// @dev See {IERC1155-balanceOfBatch}.
    ///
    /// Requirements:
    ///
    /// - `accounts` and `ids` must have the same length.
    fn balance_of_batch(
        &self,
        accounts: Vec<AccountId>,
        ids: Vec<u128>,
    ) -> Result<Vec<u128>, Error> {
        if !(accounts.length == ids.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: accounts and ids length mismatch",
            )))
        };
        let mut batch_balances: Vec<u128> = vec![u128::default(); accounts.length];
        let mut i: u128 = 0;
        while i < accounts.length {
            batch_balances[i] = self.balance_of(accounts[i], ids[i])?;
            i += 1;
        }
        return Ok(batch_balances)
    }

    /// @dev See {IERC1155-setApprovalForAll}.
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error> {
        self._set_approval_for_all(msg_sender()?, operator, approved)?;
        Ok(())
    }

    /// @dev See {IERC1155-isApprovedForAll}.
    fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> Result<bool, Error> {
        return Ok(self
            .data()
            .operator_approvals
            .get(&(account, operator))
            .unwrap_or_default())
    }

    /// @dev See {IERC1155-safeTransferFrom}.
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(from == msg_sender()? || self.is_approved_for_all(from, msg_sender()?)?) {
            return Err(Error::Custom(String::from(
                "ERC1155: caller is not token owner or approved",
            )))
        };
        self._safe_transfer_from(from, to, id, amount, data)?;
        Ok(())
    }

    /// @dev See {IERC1155-safeBatchTransferFrom}.
    fn safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(from == msg_sender()? || self.is_approved_for_all(from, msg_sender()?)?) {
            return Err(Error::Custom(String::from(
                "ERC1155: caller is not token owner or approved",
            )))
        };
        self._safe_batch_transfer_from(from, to, ids, amounts, data)?;
        Ok(())
    }

}

pub trait Internal {
    /// @dev Transfers `amount` tokens of token type `id` from `from` to `to`.
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - `from` must have a balance of tokens of type `id` of at least `amount`.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    fn _safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_safeTransferFrom}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    fn _safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev Sets a new URI for all token types, by relying on the token type ID
    /// substitution mechanism
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
    ///
    /// By this mechanism, any occurrence of the `\{id\}` substring in either the
    /// URI or any of the amounts in the JSON file at said URI will be replaced by
    /// clients with the token type ID.
    ///
    /// For example, the `https://token-cdn-domain/\{id\}.json` URI would be
    /// interpreted by clients as
    /// `https://token-cdn-domain/000000000000000000000000000000000000000000000000000000000004cce0.json`
    /// for token type ID 0x4cce0.
    ///
    /// See {uri}.
    ///
    /// Because these URIs cannot be meaningfully represented by the {URI} event,
    /// this function emits no events.
    fn _set_uri(&mut self, newuri: String) -> Result<(), Error>;

    /// @dev Creates `amount` tokens of token type `id`, and assigns them to `to`.
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    fn _mint(&mut self, to: AccountId, id: u128, amount: u128, data: Vec<u8>) -> Result<(), Error>;

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_mint}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - `ids` and `amounts` must have the same length.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    fn _mint_batch(
        &mut self,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev Destroys `amount` tokens of token type `id` from `from`
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `from` must have at least `amount` tokens of token type `id`.
    fn _burn(&mut self, from: AccountId, id: u128, amount: u128) -> Result<(), Error>;

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_burn}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - `ids` and `amounts` must have the same length.
    fn _burn_batch(
        &mut self,
        from: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
    ) -> Result<(), Error>;

    /// @dev Approve `operator` to operate on all of `owner` tokens
    ///
    /// Emits an {ApprovalForAll} event.
    fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error>;

    /// @dev Hook that is called before any token transfer. This includes minting
    /// and burning, as well as batched variants.
    ///
    /// The same hook is called on both single and batched variants. For single
    /// transfers, the length of the `ids` and `amounts` arrays will be 1.
    ///
    /// Calling conditions (for each `id` and `amount` pair):
    ///
    /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// of token type `id` will be  transferred to `to`.
    /// - When `from` is zero, `amount` tokens of token type `id` will be minted
    /// for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
    /// will be burned.
    /// - `from` and `to` are never both zero.
    /// - `ids` and `amounts` have the same, non-zero length.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _before_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev Hook that is called after any token transfer. This includes minting
    /// and burning, as well as batched variants.
    ///
    /// The same hook is called on both single and batched variants. For single
    /// transfers, the length of the `id` and `amount` arrays will be 1.
    ///
    /// Calling conditions (for each `id` and `amount` pair):
    ///
    /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// of token type `id` will be  transferred to `to`.
    /// - When `from` is zero, `amount` tokens of token type `id` will be minted
    /// for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
    /// will be burned.
    /// - `from` and `to` are never both zero.
    /// - `ids` and `amounts` have the same, non-zero length.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _after_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    fn _as_singleton_array(&self, element: u128) -> Result<Vec<u128>, Error>;

}

impl<T: Storage<Data>> Internal for T {
    /// @dev Transfers `amount` tokens of token type `id` from `from` to `to`.
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - `from` must have a balance of tokens of type `id` of at least `amount`.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    default fn _safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: transfer to the zero address",
            )))
        };
        let mut operator: AccountId = msg_sender()?;
        let mut ids: Vec<u128> = self._as_singleton_array(id)?;
        let mut amounts: Vec<u128> = self._as_singleton_array(amount)?;
        self._before_token_transfer(operator, from, to, ids, amounts, data)?;
        let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
        if !(from_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC1155: insufficient balance for transfer",
            )))
        };
        self.data()
            .balances
            .insert(&(id, from), &(from_balance - amount));
        let new_value = self.data().balances.get(&(id, to)).unwrap_or_default() + amount;
        self.data().balances.insert(&(id, to), &new_value);
        self._emit_transfer_single(operator, from, to, id, amount);
        self._after_token_transfer(operator, from, to, ids, amounts, data)?;
        self._do_safe_transfer_acceptance_check(operator, from, to, id, amount, data)?;
        Ok(())
    }

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_safeTransferFrom}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    default fn _safe_batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(ids.length == amounts.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: ids and amounts length mismatch",
            )))
        };
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: transfer to the zero address",
            )))
        };
        let mut operator: AccountId = msg_sender()?;
        self._before_token_transfer(operator, from, to, ids, amounts, data)?;
        let mut i: u128 = 0;
        while i < ids.length {
            let mut id: u128 = ids[i];
            let mut amount: u128 = amounts[i];
            let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
            if !(from_balance >= amount) {
                return Err(Error::Custom(String::from(
                    "ERC1155: insufficient balance for transfer",
                )))
            };
            self.data()
                .balances
                .insert(&(id, from), &(from_balance - amount));
            let new_value = self.data().balances.get(&(id, to)).unwrap_or_default() + amount;
            self.data().balances.insert(&(id, to), &new_value);
            i += 1;
        }
        self._emit_transfer_batch(operator, from, to, ids, amounts);
        self._after_token_transfer(operator, from, to, ids, amounts, data)?;
        self._do_safe_batch_transfer_acceptance_check(operator, from, to, ids, amounts, data)?;
        Ok(())
    }

    /// @dev Sets a new URI for all token types, by relying on the token type ID
    /// substitution mechanism
    /// https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
    ///
    /// By this mechanism, any occurrence of the `\{id\}` substring in either the
    /// URI or any of the amounts in the JSON file at said URI will be replaced by
    /// clients with the token type ID.
    ///
    /// For example, the `https://token-cdn-domain/\{id\}.json` URI would be
    /// interpreted by clients as
    /// `https://token-cdn-domain/000000000000000000000000000000000000000000000000000000000004cce0.json`
    /// for token type ID 0x4cce0.
    ///
    /// See {uri}.
    ///
    /// Because these URIs cannot be meaningfully represented by the {URI} event,
    /// this function emits no events.
    default fn _set_uri(&mut self, newuri: String) -> Result<(), Error> {
        self.data().uri = newuri;
        Ok(())
    }

    /// @dev Creates `amount` tokens of token type `id`, and assigns them to `to`.
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
    /// acceptance magic value.
    default fn _mint(
        &mut self,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: mint to the zero address",
            )))
        };
        let mut operator: AccountId = msg_sender()?;
        let mut ids: Vec<u128> = self._as_singleton_array(id)?;
        let mut amounts: Vec<u128> = self._as_singleton_array(amount)?;
        self._before_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        let new_value = self.data().balances.get(&(id, to)).unwrap_or_default() + amount;
        self.data().balances.insert(&(id, to), &new_value);
        self._emit_transfer_single(operator, ZERO_ADDRESS.into(), to, id, amount);
        self._after_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        self._do_safe_transfer_acceptance_check(
            operator,
            ZERO_ADDRESS.into(),
            to,
            id,
            amount,
            data,
        )?;
        Ok(())
    }

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_mint}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - `ids` and `amounts` must have the same length.
    /// - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
    /// acceptance magic value.
    default fn _mint_batch(
        &mut self,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: mint to the zero address",
            )))
        };
        if !(ids.length == amounts.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: ids and amounts length mismatch",
            )))
        };
        let mut operator: AccountId = msg_sender()?;
        self._before_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        let mut i: u128 = 0;
        while i < ids.length {
            let new_value =
                self.data().balances.get(&(ids[i], to)).unwrap_or_default() + amounts[i];
            self.data().balances.insert(&(ids[i], to), &new_value);
            i += 1;
        }
        self._emit_transfer_batch(operator, ZERO_ADDRESS.into(), to, ids, amounts);
        self._after_token_transfer(operator, ZERO_ADDRESS.into(), to, ids, amounts, data)?;
        self._do_safe_batch_transfer_acceptance_check(
            operator,
            ZERO_ADDRESS.into(),
            to,
            ids,
            amounts,
            data,
        )?;
        Ok(())
    }

    /// @dev Destroys `amount` tokens of token type `id` from `from`
    ///
    /// Emits a {TransferSingle} event.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `from` must have at least `amount` tokens of token type `id`.
    default fn _burn(&mut self, from: AccountId, id: u128, amount: u128) -> Result<(), Error> {
        if !(from != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: burn from the zero address",
            )))
        };
        let mut operator: AccountId = msg_sender()?;
        let mut ids: Vec<u128> = self._as_singleton_array(id)?;
        let mut amounts: Vec<u128> = self._as_singleton_array(amount)?;
        self._before_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
        if !(from_balance >= amount) {
            return Err(Error::Custom(String::from(
                "ERC1155: burn amount exceeds balance",
            )))
        };
        self.data()
            .balances
            .insert(&(id, from), &(from_balance - amount));
        self._emit_transfer_single(operator, from, ZERO_ADDRESS.into(), id, amount);
        self._after_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        Ok(())
    }

    /// @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_burn}.
    ///
    /// Emits a {TransferBatch} event.
    ///
    /// Requirements:
    ///
    /// - `ids` and `amounts` must have the same length.
    default fn _burn_batch(
        &mut self,
        from: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
    ) -> Result<(), Error> {
        if !(from != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC1155: burn from the zero address",
            )))
        };
        if !(ids.length == amounts.length) {
            return Err(Error::Custom(String::from(
                "ERC1155: ids and amounts length mismatch",
            )))
        };
        let mut operator: AccountId = msg_sender()?;
        self._before_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        let mut i: u128 = 0;
        while i < ids.length {
            let mut id: u128 = ids[i];
            let mut amount: u128 = amounts[i];
            let mut from_balance: u128 = self.data().balances.get(&(id, from)).unwrap_or_default();
            if !(from_balance >= amount) {
                return Err(Error::Custom(String::from(
                    "ERC1155: burn amount exceeds balance",
                )))
            };
            self.data()
                .balances
                .insert(&(id, from), &(from_balance - amount));
            i += 1;
        }
        self._emit_transfer_batch(operator, from, ZERO_ADDRESS.into(), ids, amounts);
        self._after_token_transfer(operator, from, ZERO_ADDRESS.into(), ids, amounts, "")?;
        Ok(())
    }

    /// @dev Approve `operator` to operate on all of `owner` tokens
    ///
    /// Emits an {ApprovalForAll} event.
    default fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error> {
        if !(owner != operator) {
            return Err(Error::Custom(String::from(
                "ERC1155: setting approval status for self",
            )))
        };
        self.data()
            .operator_approvals
            .insert(&(owner, operator), &(approved));
        self._emit_approval_for_all(owner, operator, approved);
        Ok(())
    }

    /// @dev Hook that is called before any token transfer. This includes minting
    /// and burning, as well as batched variants.
    ///
    /// The same hook is called on both single and batched variants. For single
    /// transfers, the length of the `ids` and `amounts` arrays will be 1.
    ///
    /// Calling conditions (for each `id` and `amount` pair):
    ///
    /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// of token type `id` will be  transferred to `to`.
    /// - When `from` is zero, `amount` tokens of token type `id` will be minted
    /// for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
    /// will be burned.
    /// - `from` and `to` are never both zero.
    /// - `ids` and `amounts` have the same, non-zero length.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _before_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// @dev Hook that is called after any token transfer. This includes minting
    /// and burning, as well as batched variants.
    ///
    /// The same hook is called on both single and batched variants. For single
    /// transfers, the length of the `id` and `amount` arrays will be 1.
    ///
    /// Calling conditions (for each `id` and `amount` pair):
    ///
    /// - When `from` and `to` are both non-zero, `amount` of ``from``'s tokens
    /// of token type `id` will be  transferred to `to`.
    /// - When `from` is zero, `amount` tokens of token type `id` will be minted
    /// for `to`.
    /// - when `to` is zero, `amount` of ``from``'s tokens of token type `id`
    /// will be burned.
    /// - `from` and `to` are never both zero.
    /// - `ids` and `amounts` have the same, non-zero length.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _after_token_transfer(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        Ok(())
    }

    default fn _do_safe_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        id: u128,
        amount: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if to.is_contract()? {
            if ierc_1155_receiver(to)?
                .on_erc_1155_received(operator, from, id, amount, data)?
                .is_err()
            {
                return Err(Error::Custom("Try failed"))
            }
        }
        Ok(())
    }

    default fn _do_safe_batch_transfer_acceptance_check(
        &mut self,
        operator: AccountId,
        from: AccountId,
        to: AccountId,
        ids: Vec<u128>,
        amounts: Vec<u128>,
        data: Vec<u8>,
    ) -> Result<(), Error> {
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

    default fn _as_singleton_array(&self, element: u128) -> Result<Vec<u128>, Error> {
        let mut array: Vec<u128> = vec![u128::default(); 1];
        array[0] = element;
        return Ok(array)
    }

}
