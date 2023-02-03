// Generated with Sol2Ink v2.0.0-beta
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
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub fee_to: AccountId,
    pub fee_to_setter: AccountId,
    pub get_pair: Mapping<(AccountId, AccountId), AccountId>,
    pub all_pairs: Vec<AccountId>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> UniswapV2Factory for T {
    fn all_pairs_length(&self) -> Result<u128, Error> {
        return Ok(self.data().all_pairs.length)
    }

    fn create_pair(&mut self, token_a: AccountId, token_b: AccountId) -> Result<AccountId, Error> {
        let mut pair = Default::default();
        if !(token_a != token_b) {
            return Err(Error::Custom(String::from(
                "UniswapV2: IDENTICAL_ADDRESSES",
            )))
        };
        (token_0, token_1) = if token_a < token_b { (_, _) } else { (_, _) };
        if !(token_0 != ZERO_ADDRESS.into()) {
            return Err(Error::Custom(String::from("UniswapV2: ZERO_ADDRESS")))
        };
        if !(self
            .data()
            .get_pair
            .get(&(token_0, token_1))
            .unwrap_or_default()
            == ZERO_ADDRESS.into())
        {
            return Err(Error::Custom(String::from("UniswapV2: PAIR_EXISTS")))
        };
        let mut bytecode: Vec<u8> = type_of(uniswap_v_2_pair)?.creation_code;
        let mut salt: [u8; 32] = keccak_256(abi.encode_packed(token_0, token_1)?)?;
        __comment__!("Assembly block here. Parsing assembly is not implemented yet");
        i_uniswap_v_2_pair(pair)?.initialize(token_0, token_1)?;
        self.data().get_pair.insert(&(token_0, token_1), &pair);
        self.data().get_pair.insert(&(token_1, token_0), &pair);
        self.data().all_pairs.push(pair)?;
        self._emit_pair_created(token_0, token_1, pair, self.data().all_pairs.length);
        Ok(pair)
    }

    /// single check is sufficient
    /// populate mapping in the reverse direction
    fn set_fee_to(&mut self, fee_to: AccountId) -> Result<(), Error> {
        if !(Self::env().caller() == self.data().fee_to_setter) {
            return Err(Error::Custom(String::from("UniswapV2: FORBIDDEN")))
        };
        self.data().fee_to = fee_to;
        Ok(())
    }

    fn set_fee_to_setter(&mut self, fee_to_setter: AccountId) -> Result<(), Error> {
        if !(Self::env().caller() == self.data().fee_to_setter) {
            return Err(Error::Custom(String::from("UniswapV2: FORBIDDEN")))
        };
        self.data().fee_to_setter = fee_to_setter;
        Ok(())
    }

    fn fee_to(&self) -> AccountId {
        self.data().fee_to
    }

    fn fee_to_setter(&self) -> AccountId {
        self.data().fee_to_setter
    }

    fn get_pair(&self) -> Mapping<(AccountId, AccountId), AccountId> {
        self.data().get_pair
    }

    fn all_pairs(&self) -> Vec<AccountId> {
        self.data().all_pairs
    }

}

pub trait Internal {
    fn _emit_pair_created(
        &self,
        token_0: AccountId,
        token_1: AccountId,
        pair: AccountId,
        anonymous: u128,
    );

}

impl<T: Storage<Data>> Internal for T {
    default fn _emit_pair_created(&self, _: AccountId, _: AccountId, _: AccountId, _: u128) {}

}
