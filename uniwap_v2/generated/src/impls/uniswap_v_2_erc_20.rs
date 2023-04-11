// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use crate::{
    impls,
    traits::*,
};
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
    pub total_supply: u128,
    pub balance_of: Mapping<AccountId, u128>,
    pub allowance: Mapping<(AccountId, AccountId), u128>,
    pub domain_separator: [u8; 32],
    pub nonces: Mapping<AccountId, u128>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> UniswapV2ERC20 for T {
    fn approve(&mut self, spender: AccountId, value: u128) -> Result<bool, Error> {
        self._approve(Self::env().caller(), spender, value)?;
        return Ok(true)
    }

    fn transfer(&mut self, to: AccountId, value: u128) -> Result<bool, Error> {
        self._transfer(Self::env().caller(), to, value)?;
        return Ok(true)
    }

    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: u128,
    ) -> Result<bool, Error> {
        if self
            .data()
            .allowance
            .get(&(from, Self::env().caller()))
            .unwrap_or_default()
            != <u128>::from(-1)
        {
            self.data().allowance.insert(
                &(from, Self::env().caller()),
                &(self
                    .data()
                    .allowance
                    .get(&(from, Self::env().caller()))
                    .unwrap_or_default()
                    .sub(value)?),
            );
        }
        self._transfer(from, to, value)?;
        return Ok(true)
    }

    fn permit(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: u128,
        deadline: u128,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(), Error> {
        if !(deadline >= Self::env().block_timestamp()) {
            return Err(Error::Custom(String::from("UniswapV2: EXPIRED")))
        };
        let mut digest: [u8; 32] = keccak_256(abi.encode_packed(
            "\\x19\\x01",
            self.data().domain_separator,
            keccak_256(abi.encode(
                PERMIT_TYPEHASH,
                owner,
                spender,
                value,
                self.data().nonces.get(&owner).unwrap_or_default() += 1,
                deadline,
            )?)?,
        )?)?;
        let mut recovered_address: AccountId = ecrecover(digest, v, r, s)?;
        if !(recovered_address != ZERO_ADDRESS.into() && recovered_address == owner) {
            return Err(Error::Custom(String::from("UniswapV2: INVALID_SIGNATURE")))
        };
        self._approve(owner, spender, value)?;
        Ok(())
    }

    fn total_supply(&self) -> u128 {
        self.data().total_supply
    }

    fn balance_of(&self) -> Mapping<AccountId, u128> {
        self.data().balance_of
    }

    fn allowance(&self) -> Mapping<(AccountId, AccountId), u128> {
        self.data().allowance
    }

    fn domain_separator(&self) -> [u8; 32] {
        self.data().domain_separator
    }

    fn nonces(&self) -> Mapping<AccountId, u128> {
        self.data().nonces
    }

}

pub trait Internal {
    fn _mint(&mut self, to: AccountId, value: u128) -> Result<(), Error>;

    fn _burn(&mut self, from: AccountId, value: u128) -> Result<(), Error>;

    fn _approve(&mut self, owner: AccountId, spender: AccountId, value: u128) -> Result<(), Error>;

    fn _transfer(&mut self, from: AccountId, to: AccountId, value: u128) -> Result<(), Error>;

    fn _emit_approval(&self, owner: AccountId, spender: AccountId, value: u128);

    fn _emit_transfer(&self, from: AccountId, to: AccountId, value: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _mint(&mut self, to: AccountId, value: u128) -> Result<(), Error> {
        self.data().total_supply = self.data().total_supply.add(value)?;
        self.data().balance_of.insert(
            &(to),
            &(self
                .data()
                .balance_of
                .get(&to)
                .unwrap_or_default()
                .add(value)?),
        );
        self._emit_transfer(ZERO_ADDRESS.into(), to, value);
        Ok(())
    }

    default fn _burn(&mut self, from: AccountId, value: u128) -> Result<(), Error> {
        self.data().balance_of.insert(
            &(from),
            &(self
                .data()
                .balance_of
                .get(&from)
                .unwrap_or_default()
                .sub(value)?),
        );
        self.data().total_supply = self.data().total_supply.sub(value)?;
        self._emit_transfer(from, ZERO_ADDRESS.into(), value);
        Ok(())
    }

    default fn _approve(
        &mut self,
        owner: AccountId,
        spender: AccountId,
        value: u128,
    ) -> Result<(), Error> {
        self.data().allowance.insert(&(owner, spender), &(value));
        self._emit_approval(owner, spender, value);
        Ok(())
    }

    default fn _transfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: u128,
    ) -> Result<(), Error> {
        self.data().balance_of.insert(
            &(from),
            &(self
                .data()
                .balance_of
                .get(&from)
                .unwrap_or_default()
                .sub(value)?),
        );
        self.data().balance_of.insert(
            &(to),
            &(self
                .data()
                .balance_of
                .get(&to)
                .unwrap_or_default()
                .add(value)?),
        );
        self._emit_transfer(from, to, value);
        Ok(())
    }

    default fn _emit_approval(&self, _: AccountId, _: AccountId, _: u128) {}

    default fn _emit_transfer(&self, _: AccountId, _: AccountId, _: u128) {}

}
