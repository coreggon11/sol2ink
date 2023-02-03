// Generated with Sol2Ink v2.0.0
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
    /// Token name
    pub name: String,
    /// Token symbol
    pub symbol: String,
    /// Mapping from token ID to owner address
    pub owners: Mapping<u128, AccountId>,
    /// Mapping owner address to token count
    pub balances: Mapping<AccountId, u128>,
    /// Mapping from token ID to approved address
    pub token_approvals: Mapping<u128, AccountId>,
    /// Mapping from owner to operator approvals
    pub operator_approvals: Mapping<(AccountId, AccountId), bool>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ERC721 for T {
    /// @dev See {IERC165-supportsInterface}.
    fn supports_interface(&self, interface_id: [u8; 4]) -> Result<bool, Error> {
        return Ok(interface_id == type_of(ierc_721)?.interface_id
            || interface_id == type_of(ierc_721_metadata)?.interface_id
            || super.supports_interface(interface_id)?)
    }

    /// @dev See {IERC721-balanceOf}.
    fn balance_of(&self, owner: AccountId) -> Result<u128, Error> {
        if !(self.data().owner != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC721: address zero is not a valid owner",
            )))
        };
        return Ok(self
            .data()
            .balances
            .get(&self.data().owner)
            .unwrap_or_default())
    }

    /// @dev See {IERC721-ownerOf}.
    fn owner_of(&self, token_id: u128) -> Result<AccountId, Error> {
        let mut owner: AccountId = self._owner_of(token_id)?;
        if !(self.data().owner != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from("ERC721: invalid token ID")))
        };
        return Ok(self.data().owner)
    }

    /// @dev See {IERC721Metadata-name}.
    fn name(&self) -> Result<String, Error> {
        return Ok(self.data().name)
    }

    /// @dev See {IERC721Metadata-symbol}.
    fn symbol(&self) -> Result<String, Error> {
        return Ok(self.data().symbol)
    }

    /// @dev See {IERC721Metadata-tokenURI}.
    fn token_uri(&self, token_id: u128) -> Result<String, Error> {
        self._require_minted(token_id)?;
        let mut base_uri: String = self._base_uri()?;
        return Ok(if Vec::<u8>::from(base_uri).length > 0 {
            <String>::from(abi.encode_packed(base_uri, token_id.to_string()?)?)
        } else {
            ""
        })
    }

    /// @dev See {IERC721-approve}.
    fn approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        if !(to != self.data().owner) {
            return Err(Error::Custom(String::from(
                "ERC721: approval to current owner",
            )))
        };
        if !(msg_sender()? == self.data().owner
            || self.is_approved_for_all(self.data().owner, msg_sender()?)?)
        {
            return Err(Error::Custom(String::from(
                "ERC721: approve caller is not token owner or approved for all",
            )))
        };
        self._approve(to, token_id)?;
        Ok(())
    }

    /// @dev See {IERC721-getApproved}.
    fn get_approved(&self, token_id: u128) -> Result<AccountId, Error> {
        self._require_minted(token_id)?;
        return Ok(self
            .data()
            .token_approvals
            .get(&token_id)
            .unwrap_or_default())
    }

    /// @dev See {IERC721-setApprovalForAll}.
    fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<(), Error> {
        self._set_approval_for_all(msg_sender()?, operator, approved)?;
        Ok(())
    }

    /// @dev See {IERC721-isApprovedForAll}.
    fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> Result<bool, Error> {
        return Ok(self
            .data()
            .operator_approvals
            .get(&(self.data().owner, operator))
            .unwrap_or_default())
    }

    /// @dev See {IERC721-transferFrom}.
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        if !(self._is_approved_or_owner(msg_sender()?, token_id)?) {
            return Err(Error::Custom(String::from(
                "ERC721: caller is not token owner or approved",
            )))
        };
        self._transfer(from, to, token_id)?;
        Ok(())
    }

    ///solhint-disable-next-line max-line-length
    /// @dev See {IERC721-safeTransferFrom}.
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        self.safe_transfer_from(from, to, token_id, "")?;
        Ok(())
    }

    /// @dev See {IERC721-safeTransferFrom}.
    fn safe_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(self._is_approved_or_owner(msg_sender()?, token_id)?) {
            return Err(Error::Custom(String::from(
                "ERC721: caller is not token owner or approved",
            )))
        };
        self._safe_transfer(from, to, token_id, data)?;
        Ok(())
    }

}

pub trait Internal {
    /// @dev Base URI for computing {tokenURI}. If set, the resulting URI for each
    /// token will be the concatenation of the `baseURI` and the `tokenId`. Empty
    /// by default, can be overridden in child contracts.
    fn _base_uri(&self) -> Result<String, Error>;

    /// @dev Safely transfers `tokenId` token from `from` to `to`, checking first that contract recipients
    /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
    ///
    /// `data` is additional data, it has no specified format and it is sent in call to `to`.
    ///
    /// This internal function is equivalent to {safeTransferFrom}, and can be used to e.g.
    /// implement alternative mechanisms to perform token transfer, such as signature-based.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must exist and be owned by `from`.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    fn _safe_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error>;

    /// @dev Returns the owner of the `tokenId`. Does NOT revert if token doesn't exist
    fn _owner_of(&self, token_id: u128) -> Result<AccountId, Error>;

    /// @dev Returns whether `tokenId` exists.
    ///
    /// Tokens can be managed by their owner or approved accounts via {approve} or {setApprovalForAll}.
    ///
    /// Tokens start existing when they are minted (`_mint`),
    /// and stop existing when they are burned (`_burn`).
    fn _exists(&self, token_id: u128) -> Result<bool, Error>;

    /// @dev Returns whether `spender` is allowed to manage `tokenId`.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must exist.
    fn _is_approved_or_owner(&self, spender: AccountId, token_id: u128) -> Result<bool, Error>;

    /// @dev Safely mints `tokenId` and transfers it to `to`.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must not exist.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Same as {xref-ERC721-_safeMint-address-uint256-}[`_safeMint`], with an additional `data` parameter which is
    /// forwarded in {IERC721Receiver-onERC721Received} to contract recipients.
    fn _safe_mint(&mut self, to: AccountId, token_id: u128, data: Vec<u8>) -> Result<(), Error>;

    /// @dev Mints `tokenId` and transfers it to `to`.
    ///
    /// WARNING: Usage of this method is discouraged, use {_safeMint} whenever possible
    ///
    /// Requirements:
    ///
    /// - `tokenId` must not exist.
    /// - `to` cannot be the zero address.
    ///
    /// Emits a {Transfer} event.
    fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// Check that tokenId was not minted by `_beforeTokenTransfer` hook
    /// Will not overflow unless all 2**256 token ids are minted to the same owner.
    /// Given that tokens are minted one by one, it is impossible in practice that
    /// this ever happens. Might change if we allow batch minting.
    /// The ERC fails to describe this case.
    /// @dev Destroys `tokenId`.
    /// The approval is cleared when the token is burned.
    /// This is an internal function that does not check if the sender is authorized to operate on the token.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must exist.
    ///
    /// Emits a {Transfer} event.
    fn _burn(&mut self, token_id: u128) -> Result<(), Error>;

    /// Update ownership in case tokenId was transferred by `_beforeTokenTransfer` hook
    /// Clear approvals
    /// Cannot overflow, as that would require more tokens to be burned/transferred
    /// out than the owner initially received through minting and transferring in.
    /// @dev Transfers `tokenId` from `from` to `to`.
    ///  As opposed to {transferFrom}, this imposes no restrictions on msg.sender.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must be owned by `from`.
    ///
    /// Emits a {Transfer} event.
    fn _transfer(&mut self, from: AccountId, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// Check that tokenId was not transferred by `_beforeTokenTransfer` hook
    /// Clear approvals from the previous owner
    /// `_balances[from]` cannot overflow for the same reason as described in `_burn`:
    /// `from`'s balance is the number of token held, which is at least one before the current
    /// transfer.
    /// `_balances[to]` could overflow in the conditions described in `_mint`. That would require
    /// all 2**256 token ids to be minted, which in practice is impossible.
    /// @dev Approve `to` to operate on `tokenId`
    ///
    /// Emits an {Approval} event.
    fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error>;

    /// @dev Approve `operator` to operate on all of `owner` tokens
    ///
    /// Emits an {ApprovalForAll} event.
    fn _set_approval_for_all(
        &mut self,
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    ) -> Result<(), Error>;

    /// @dev Reverts if the `tokenId` has not been minted yet.
    fn _require_minted(&self, token_id: u128) -> Result<(), Error>;

    /// @dev Internal function to invoke {IERC721Receiver-onERC721Received} on a target address.
    /// The call is not executed if the target address is not a contract.
    ///
    /// @param from address representing the previous owner of the given token ID
    /// @param to target address that will receive the tokens
    /// @param tokenId uint256 ID of the token to be transferred
    /// @param data bytes optional data to send along with the call
    /// @return bool whether the call correctly returned the expected magic value
    fn _check_on_erc_721_received(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<bool, Error>;

    ///@solidity memory-safe-assembly
    /// @dev Hook that is called before any token transfer. This includes minting and burning. If {ERC721Consecutive} is
    /// used, the hook may be called as part of a consecutive (batch) mint, as indicated by `batchSize` greater than 1.
    ///
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both non-zero, ``from``'s tokens will be transferred to `to`.
    /// - When `from` is zero, the tokens will be minted for `to`.
    /// - When `to` is zero, ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    /// - `batchSize` is non-zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    ///firstTokenId
    fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        _: u128,
        batch_size: u128,
    ) -> Result<(), Error>;

    /// @dev Hook that is called after any token transfer. This includes minting and burning. If {ERC721Consecutive} is
    /// used, the hook may be called as part of a consecutive (batch) mint, as indicated by `batchSize` greater than 1.
    ///
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both non-zero, ``from``'s tokens were transferred to `to`.
    /// - When `from` is zero, the tokens were minted for `to`.
    /// - When `to` is zero, ``from``'s tokens were burned.
    /// - `from` and `to` are never both zero.
    /// - `batchSize` is non-zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        first_token_id: u128,
        batch_size: u128,
    ) -> Result<(), Error>;

}

impl<T: Storage<Data>> Internal for T {
    /// @dev Base URI for computing {tokenURI}. If set, the resulting URI for each
    /// token will be the concatenation of the `baseURI` and the `tokenId`. Empty
    /// by default, can be overridden in child contracts.
    default fn _base_uri(&self) -> Result<String, Error> {
        return Ok("")
    }

    /// @dev Safely transfers `tokenId` token from `from` to `to`, checking first that contract recipients
    /// are aware of the ERC721 protocol to prevent tokens from being forever locked.
    ///
    /// `data` is additional data, it has no specified format and it is sent in call to `to`.
    ///
    /// This internal function is equivalent to {safeTransferFrom}, and can be used to e.g.
    /// implement alternative mechanisms to perform token transfer, such as signature-based.
    ///
    /// Requirements:
    ///
    /// - `from` cannot be the zero address.
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must exist and be owned by `from`.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    default fn _safe_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self._transfer(from, to, token_id)?;
        if !(self._check_on_erc_721_received(from, to, token_id, data)?) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to non ERC721Receiver implementer",
            )))
        };
        Ok(())
    }

    /// @dev Returns the owner of the `tokenId`. Does NOT revert if token doesn't exist
    default fn _owner_of(&self, token_id: u128) -> Result<AccountId, Error> {
        return Ok(self.data().owners.get(&token_id).unwrap_or_default())
    }

    /// @dev Returns whether `tokenId` exists.
    ///
    /// Tokens can be managed by their owner or approved accounts via {approve} or {setApprovalForAll}.
    ///
    /// Tokens start existing when they are minted (`_mint`),
    /// and stop existing when they are burned (`_burn`).
    default fn _exists(&self, token_id: u128) -> Result<bool, Error> {
        return Ok(self._owner_of(token_id)? != ZERO_ADDRESS.into())
    }

    /// @dev Returns whether `spender` is allowed to manage `tokenId`.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must exist.
    default fn _is_approved_or_owner(
        &self,
        spender: AccountId,
        token_id: u128,
    ) -> Result<bool, Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        return Ok((spender == self.data().owner
            || self.is_approved_for_all(self.data().owner, spender)?
            || self.get_approved(token_id)? == spender))
    }

    /// @dev Safely mints `tokenId` and transfers it to `to`.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must not exist.
    /// - If `to` refers to a smart contract, it must implement {IERC721Receiver-onERC721Received}, which is called upon a safe transfer.
    ///
    /// Emits a {Transfer} event.
    default fn _safe_mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        self._safe_mint(to, token_id, "")?;
        Ok(())
    }

    /// @dev Same as {xref-ERC721-_safeMint-address-uint256-}[`_safeMint`], with an additional `data` parameter which is
    /// forwarded in {IERC721Receiver-onERC721Received} to contract recipients.
    default fn _safe_mint(
        &mut self,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self._mint(to, token_id)?;
        if !(self._check_on_erc_721_received(ZERO_ADDRESS.into(), to, token_id, data)?) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to non ERC721Receiver implementer",
            )))
        };
        Ok(())
    }

    /// @dev Mints `tokenId` and transfers it to `to`.
    ///
    /// WARNING: Usage of this method is discouraged, use {_safeMint} whenever possible
    ///
    /// Requirements:
    ///
    /// - `tokenId` must not exist.
    /// - `to` cannot be the zero address.
    ///
    /// Emits a {Transfer} event.
    default fn _mint(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC721: mint to the zero address",
            )))
        };
        if !(!self._exists(token_id)?) {
            return Err(Error::Custom(String::from("ERC721: token already minted")))
        };
        self._before_token_transfer(ZERO_ADDRESS.into(), to, token_id, 1)?;
        if !(!self._exists(token_id)?) {
            return Err(Error::Custom(String::from("ERC721: token already minted")))
        };
        let new_value = self.data().balances.get(&(to)).unwrap_or_default() + 1;
        self.data().balances.insert(&(to), &new_value);
        self.data().owners.insert(&(token_id), &to);
        self._emit_transfer(ZERO_ADDRESS.into(), to, token_id);
        self._after_token_transfer(ZERO_ADDRESS.into(), to, token_id, 1)?;
        Ok(())
    }

    /// Check that tokenId was not minted by `_beforeTokenTransfer` hook
    /// Will not overflow unless all 2**256 token ids are minted to the same owner.
    /// Given that tokens are minted one by one, it is impossible in practice that
    /// this ever happens. Might change if we allow batch minting.
    /// The ERC fails to describe this case.
    /// @dev Destroys `tokenId`.
    /// The approval is cleared when the token is burned.
    /// This is an internal function that does not check if the sender is authorized to operate on the token.
    ///
    /// Requirements:
    ///
    /// - `tokenId` must exist.
    ///
    /// Emits a {Transfer} event.
    default fn _burn(&mut self, token_id: u128) -> Result<(), Error> {
        let mut owner: AccountId = erc_721.owner_of(token_id)?;
        self._before_token_transfer(self.data().owner, ZERO_ADDRESS.into(), token_id, 1)?;
        self.data().owner = erc_721.owner_of(token_id)?;
        self.data().token_approvals.remove(&(token_id));
        let new_value = self
            .data()
            .balances
            .get(&(self.data().owner))
            .unwrap_or_default()
            - 1;
        self.data()
            .balances
            .insert(&(self.data().owner), &new_value);
        self.data().owners.remove(&(token_id));
        self._emit_transfer(self.data().owner, ZERO_ADDRESS.into(), token_id);
        self._after_token_transfer(self.data().owner, ZERO_ADDRESS.into(), token_id, 1)?;
        Ok(())
    }

    /// Update ownership in case tokenId was transferred by `_beforeTokenTransfer` hook
    /// Clear approvals
    /// Cannot overflow, as that would require more tokens to be burned/transferred
    /// out than the owner initially received through minting and transferring in.
    /// @dev Transfers `tokenId` from `from` to `to`.
    ///  As opposed to {transferFrom}, this imposes no restrictions on msg.sender.
    ///
    /// Requirements:
    ///
    /// - `to` cannot be the zero address.
    /// - `tokenId` token must be owned by `from`.
    ///
    /// Emits a {Transfer} event.
    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
    ) -> Result<(), Error> {
        if !(erc_721.owner_of(token_id)? == from) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer from incorrect owner",
            )))
        };
        if !(to != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer to the zero address",
            )))
        };
        self._before_token_transfer(from, to, token_id, 1)?;
        if !(erc_721.owner_of(token_id)? == from) {
            return Err(Error::Custom(String::from(
                "ERC721: transfer from incorrect owner",
            )))
        };
        self.data().token_approvals.remove(&(token_id));
        let new_value = self.data().balances.get(&(from)).unwrap_or_default() - 1;
        self.data().balances.insert(&(from), &new_value);
        let new_value = self.data().balances.get(&(to)).unwrap_or_default() + 1;
        self.data().balances.insert(&(to), &new_value);
        self.data().owners.insert(&(token_id), &to);
        self._emit_transfer(from, to, token_id);
        self._after_token_transfer(from, to, token_id, 1)?;
        Ok(())
    }

    /// Check that tokenId was not transferred by `_beforeTokenTransfer` hook
    /// Clear approvals from the previous owner
    /// `_balances[from]` cannot overflow for the same reason as described in `_burn`:
    /// `from`'s balance is the number of token held, which is at least one before the current
    /// transfer.
    /// `_balances[to]` could overflow in the conditions described in `_mint`. That would require
    /// all 2**256 token ids to be minted, which in practice is impossible.
    /// @dev Approve `to` to operate on `tokenId`
    ///
    /// Emits an {Approval} event.
    default fn _approve(&mut self, to: AccountId, token_id: u128) -> Result<(), Error> {
        self.data().token_approvals.insert(&(token_id), &to);
        self._emit_approval(erc_721.owner_of(token_id)?, to, token_id);
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
        if !(self.data().owner != operator) {
            return Err(Error::Custom(String::from("ERC721: approve to caller")))
        };
        self.data()
            .operator_approvals
            .insert(&(self.data().owner, operator), &approved);
        self._emit_approval_for_all(self.data().owner, operator, approved);
        Ok(())
    }

    /// @dev Reverts if the `tokenId` has not been minted yet.
    default fn _require_minted(&self, token_id: u128) -> Result<(), Error> {
        if !(self._exists(token_id)?) {
            return Err(Error::Custom(String::from("ERC721: invalid token ID")))
        };
        Ok(())
    }

    /// @dev Internal function to invoke {IERC721Receiver-onERC721Received} on a target address.
    /// The call is not executed if the target address is not a contract.
    ///
    /// @param from address representing the previous owner of the given token ID
    /// @param to target address that will receive the tokens
    /// @param tokenId uint256 ID of the token to be transferred
    /// @param data bytes optional data to send along with the call
    /// @return bool whether the call correctly returned the expected magic value
    default fn _check_on_erc_721_received(
        &mut self,
        from: AccountId,
        to: AccountId,
        token_id: u128,
        data: Vec<u8>,
    ) -> Result<bool, Error> {
        if to.is_contract()? {
            if ierc_721_receiver(to)?
                .on_erc_721_received(msg_sender()?, from, token_id, data)?
                .is_err()
            {
                return Err(Error::Custom("Try failed"))
            }
        } else {
            return Ok(true)
        }
    }

    ///@solidity memory-safe-assembly
    /// @dev Hook that is called before any token transfer. This includes minting and burning. If {ERC721Consecutive} is
    /// used, the hook may be called as part of a consecutive (batch) mint, as indicated by `batchSize` greater than 1.
    ///
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both non-zero, ``from``'s tokens will be transferred to `to`.
    /// - When `from` is zero, the tokens will be minted for `to`.
    /// - When `to` is zero, ``from``'s tokens will be burned.
    /// - `from` and `to` are never both zero.
    /// - `batchSize` is non-zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    ///firstTokenId
    default fn _before_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        _: u128,
        batch_size: u128,
    ) -> Result<(), Error> {
        if batch_size > 1 {
            if from != ZERO_ADDRESS.into() {
                let new_value = self.data().balances.get(&(from)).unwrap_or_default() - batch_size;
                self.data().balances.insert(&(from), &new_value);
            }
            if to != ZERO_ADDRESS.into() {
                let new_value = self.data().balances.get(&(to)).unwrap_or_default() + batch_size;
                self.data().balances.insert(&(to), &new_value);
            }
        }
        Ok(())
    }

    /// @dev Hook that is called after any token transfer. This includes minting and burning. If {ERC721Consecutive} is
    /// used, the hook may be called as part of a consecutive (batch) mint, as indicated by `batchSize` greater than 1.
    ///
    /// Calling conditions:
    ///
    /// - When `from` and `to` are both non-zero, ``from``'s tokens were transferred to `to`.
    /// - When `from` is zero, the tokens were minted for `to`.
    /// - When `to` is zero, ``from``'s tokens were burned.
    /// - `from` and `to` are never both zero.
    /// - `batchSize` is non-zero.
    ///
    /// To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
    default fn _after_token_transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        first_token_id: u128,
        batch_size: u128,
    ) -> Result<(), Error> {
        Ok(())
    }

}
