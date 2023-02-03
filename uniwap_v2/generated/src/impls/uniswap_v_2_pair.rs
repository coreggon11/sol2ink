// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
pub use ink_prelude::vec::*;
use openbrush::traits::Storage;
pub use openbrush::traits::{
    AccountId,
    AccountIdExt,
    ZERO_ADDRESS,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory: AccountId,
    pub token_0: AccountId,
    pub token_1: AccountId,
    pub reserve_0: u128,
    /// uses single storage slot, accessible via getReserves
    pub reserve_1: u128,
    /// uses single storage slot, accessible via getReserves
    pub block_timestamp_last: u32,
    /// uses single storage slot, accessible via getReserves
    pub price_0_cumulative_last: u128,
    pub price_1_cumulative_last: u128,
    pub k_last: u128,
    /// reserve0 * reserve1, as of immediately after the most recent liquidity event
    pub unlocked: u128,
    pub _reserved: Option<()>,
}

#[modifier_definition]
pub fn lock<T, F, R>(instance: &mut T, body: F) -> Result<R, Error>
where
    T: UniswapV2Pair,
    F: FnOnce(&mut T) -> Result<R, Error>,
{
    if !(instance.data().unlocked == 1) {
        return Err(Error::Custom(String::from("UniswapV2: LOCKED")))
    };
    instance.data().unlocked = 0;
    body(instance);
    instance.data().unlocked = 1;
}


impl<T: Storage<Data>> UniswapV2Pair for T {
    fn get_reserves(&self) -> Result<(u128, u128, u32), Error> {
        let mut reserve_0 = Default::default();
        let mut reserve_1 = Default::default();
        let mut block_timestamp_last = Default::default();
        reserve_0 = self.data().reserve_0;
        reserve_1 = self.data().reserve_1;
        block_timestamp_last = self.data().block_timestamp_last;
        Ok((reserve_0, reserve_1, block_timestamp_last))
    }

    /// called once by the factory at time of deployment
    fn initialize(&mut self, token_0: AccountId, token_1: AccountId) -> Result<(), Error> {
        if !(Self::env().caller() == self.data().factory) {
            return Err(Error::Custom(String::from("UniswapV2: FORBIDDEN")))
        };
        self.data().token_0 = token_0;
        self.data().token_1 = token_1;
        Ok(())
    }

    /// gas savings
    /// this low-level function should be called from a contract which performs important safety checks
    #[modifiers(lock())]
    fn mint(&mut self, to: AccountId) -> Result<u128, Error> {
        let mut liquidity = Default::default();
        (reserve_0, reserve_1, _) = self.get_reserves()?;
        let mut balance_0: u128 =
            ierc_20(self.data().token_0)?.balance_of(Self::env().account_id())?;
        let mut balance_1: u128 =
            ierc_20(self.data().token_1)?.balance_of(Self::env().account_id())?;
        let mut amount_0: u128 = balance_0.sub(reserve_0)?;
        let mut amount_1: u128 = balance_1.sub(reserve_1)?;
        let mut fee_on: bool = self._mint_fee(reserve_0, reserve_1)?;
        let mut total_supply: u128 = total_supply;
        if total_supply == 0 {
            liquidity = math.sqrt(amount_0.mul(amount_1)?)?.sub(MINIMUM_LIQUIDITY)?;
            mint(ZERO_ADDRESS.into(), MINIMUM_LIQUIDITY)?;
        } else {
            liquidity = math.min(
                amount_0.mul(total_supply)? / reserve_0,
                amount_1.mul(total_supply)? / reserve_1,
            )?;
        }
        if !(liquidity > 0) {
            return Err(Error::Custom(String::from(
                "UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED",
            )))
        };
        mint(to, liquidity)?;
        self._update(balance_0, balance_1, reserve_0, reserve_1)?;
        if fee_on {
            self.data().k_last = <u128>::from(self.data().reserve_0).mul(self.data().reserve_1)?;
        }
        self._emit_mint(Self::env().caller(), amount_0, amount_1);
        Ok(liquidity)
    }

    /// gas savings
    /// gas savings, must be defined here since totalSupply can update in _mintFee
    /// permanently lock the first MINIMUM_LIQUIDITY tokens
    /// reserve0 and reserve1 are up-to-date
    /// this low-level function should be called from a contract which performs important safety checks
    #[modifiers(lock())]
    fn burn(&mut self, to: AccountId) -> Result<(u128, u128), Error> {
        let mut amount_0 = Default::default();
        let mut amount_1 = Default::default();
        (reserve_0, reserve_1, _) = self.get_reserves()?;
        let mut token_0: AccountId = self.data().token_0;
        let mut token_1: AccountId = self.data().token_1;
        let mut balance_0: u128 = ierc_20(token_0)?.balance_of(Self::env().account_id())?;
        let mut balance_1: u128 = ierc_20(token_1)?.balance_of(Self::env().account_id())?;
        let mut liquidity: u128 = balance_of[Self::env().account_id()];
        let mut fee_on: bool = self._mint_fee(reserve_0, reserve_1)?;
        let mut total_supply: u128 = total_supply;
        amount_0 = liquidity.mul(balance_0)? / total_supply;
        amount_1 = liquidity.mul(balance_1)? / total_supply;
        if !(amount_0 > 0 && amount_1 > 0) {
            return Err(Error::Custom(String::from(
                "UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED",
            )))
        };
        burn(Self::env().account_id(), liquidity)?;
        self._safe_transfer(token_0, to, amount_0)?;
        self._safe_transfer(token_1, to, amount_1)?;
        balance_0 = ierc_20(token_0)?.balance_of(Self::env().account_id())?;
        balance_1 = ierc_20(token_1)?.balance_of(Self::env().account_id())?;
        self._update(balance_0, balance_1, reserve_0, reserve_1)?;
        if fee_on {
            self.data().k_last = <u128>::from(self.data().reserve_0).mul(self.data().reserve_1)?;
        }
        self._emit_burn(Self::env().caller(), amount_0, amount_1, to);
        Ok((amount_0, amount_1))
    }

    /// gas savings
    /// gas savings
    /// gas savings
    /// gas savings, must be defined here since totalSupply can update in _mintFee
    /// using balances ensures pro-rata distribution
    /// using balances ensures pro-rata distribution
    /// reserve0 and reserve1 are up-to-date
    /// this low-level function should be called from a contract which performs important safety checks
    #[modifiers(lock())]
    fn swap(
        &mut self,
        amount_0_out: u128,
        amount_1_out: u128,
        to: AccountId,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        if !(amount_0_out > 0 || amount_1_out > 0) {
            return Err(Error::Custom(String::from(
                "UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        (reserve_0, reserve_1, _) = self.get_reserves()?;
        if !(amount_0_out < reserve_0 && amount_1_out < reserve_1) {
            return Err(Error::Custom(String::from(
                "UniswapV2: INSUFFICIENT_LIQUIDITY",
            )))
        };
        let mut token_0: AccountId = self.data().token_0;
        let mut token_1: AccountId = self.data().token_1;
        if !(to != token_0 && to != token_1) {
            return Err(Error::Custom(String::from("UniswapV2: INVALID_TO")))
        };
        if amount_0_out > 0 {
            self._safe_transfer(token_0, to, amount_0_out)?;
        }
        if amount_1_out > 0 {
            self._safe_transfer(token_1, to, amount_1_out)?;
        }
        if data.length > 0 {
            i_uniswap_v_2_callee(to)?.uniswap_v_2_call(
                Self::env().caller(),
                amount_0_out,
                amount_1_out,
                data,
            )?;
        }
        balance_0 = ierc_20(token_0)?.balance_of(Self::env().account_id())?;
        balance_1 = ierc_20(token_1)?.balance_of(Self::env().account_id())?;
        let mut amount_0_in: u128 = if balance_0 > reserve_0 - amount_0_out {
            balance_0 - (reserve_0 - amount_0_out)
        } else {
            0
        };
        let mut amount_1_in: u128 = if balance_1 > reserve_1 - amount_1_out {
            balance_1 - (reserve_1 - amount_1_out)
        } else {
            0
        };
        if !(amount_0_in > 0 || amount_1_in > 0) {
            return Err(Error::Custom(String::from(
                "UniswapV2: INSUFFICIENT_INPUT_AMOUNT",
            )))
        };
        let mut balance_0_adjusted: u128 = balance_0.mul(1000)?.sub(amount_0_in.mul(3)?)?;
        let mut balance_1_adjusted: u128 = balance_1.mul(1000)?.sub(amount_1_in.mul(3)?)?;
        if !(balance_0_adjusted.mul(balance_1_adjusted)?
            >= <u128>::from(reserve_0).mul(reserve_1)?.mul(1000.pow(2))?)
        {
            return Err(Error::Custom(String::from("UniswapV2: K")))
        };
        self._update(balance_0, balance_1, reserve_0, reserve_1)?;
        self._emit_swap(
            Self::env().caller(),
            amount_0_in,
            amount_1_in,
            amount_0_out,
            amount_1_out,
            to,
        );
        Ok(())
    }

    /// gas savings
    /// scope for _token{0,1}, avoids stack too deep errors
    /// optimistically transfer tokens
    /// optimistically transfer tokens
    /// scope for reserve{0,1}Adjusted, avoids stack too deep errors
    /// force balances to match reserves
    #[modifiers(lock())]
    fn skim(&mut self, to: AccountId) -> Result<(), Error> {
        let mut token_0: AccountId = self.data().token_0;
        let mut token_1: AccountId = self.data().token_1;
        self._safe_transfer(
            token_0,
            to,
            ierc_20(token_0)?
                .balance_of(Self::env().account_id())?
                .sub(self.data().reserve_0)?,
        )?;
        self._safe_transfer(
            token_1,
            to,
            ierc_20(token_1)?
                .balance_of(Self::env().account_id())?
                .sub(self.data().reserve_1)?,
        )?;
        Ok(())
    }

    /// gas savings
    /// gas savings
    /// force reserves to match balances
    #[modifiers(lock())]
    fn sync(&mut self) -> Result<(), Error> {
        self._update(
            ierc_20(self.data().token_0)?.balance_of(Self::env().account_id())?,
            ierc_20(self.data().token_1)?.balance_of(Self::env().account_id())?,
            self.data().reserve_0,
            self.data().reserve_1,
        )?;
        Ok(())
    }

    fn factory(&self) -> AccountId {
        self.data().factory
    }

    fn token_0(&self) -> AccountId {
        self.data().token_0
    }

    fn token_1(&self) -> AccountId {
        self.data().token_1
    }

    fn price_0_cumulative_last(&self) -> u128 {
        self.data().price_0_cumulative_last
    }

    fn price_1_cumulative_last(&self) -> u128 {
        self.data().price_1_cumulative_last
    }

    fn k_last(&self) -> u128 {
        self.data().k_last
    }

}

pub trait Internal {
    fn _safe_transfer(&mut self, token: AccountId, to: AccountId, value: u128)
        -> Result<(), Error>;

    /// sufficient check
    /// update reserves and, on the first call per block, price accumulators
    fn _update(
        &mut self,
        balance_0: u128,
        balance_1: u128,
        reserve_0: u128,
        reserve_1: u128,
    ) -> Result<(), Error>;

    /// overflow is desired
    /// * never overflows, and + overflow is desired
    /// if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    fn _mint_fee(&mut self, reserve_0: u128, reserve_1: u128) -> Result<bool, Error>;

    fn _emit_mint(&self, sender: AccountId, amount_0: u128, amount_1: u128);

    fn _emit_burn(&self, sender: AccountId, amount_0: u128, amount_1: u128, to: AccountId);

    fn _emit_swap(
        &self,
        sender: AccountId,
        amount_0_in: u128,
        amount_1_in: u128,
        amount_0_out: u128,
        amount_1_out: u128,
        to: AccountId,
    );

    fn _emit_sync(&self, reserve_0: u128, reserve_1: u128);

}

impl<T: Storage<Data>> Internal for T {
    default fn _safe_transfer(
        &mut self,
        token: AccountId,
        to: AccountId,
        value: u128,
    ) -> Result<(), Error> {
        (success, data) = token.call(abi.encode_with_selector(SELECTOR, to, value)?)?;
        if !(success && (data.length == 0 || abi.decode(__comment__!(data, (bool)))?)) {
            return Err(Error::Custom(String::from("UniswapV2: TRANSFER_FAILED")))
        };
        Ok(())
    }

    /// sufficient check
    /// update reserves and, on the first call per block, price accumulators
    default fn _update(
        &mut self,
        balance_0: u128,
        balance_1: u128,
        reserve_0: u128,
        reserve_1: u128,
    ) -> Result<(), Error> {
        if !(balance_0 <= <u128>::from(-1) && balance_1 <= <u128>::from(-1)) {
            return Err(Error::Custom(String::from("UniswapV2: OVERFLOW")))
        };
        let mut block_timestamp: u32 = <u32>::from(block.timestamp % 2.pow(32));
        let mut time_elapsed: u32 = block_timestamp - self.data().block_timestamp_last;
        if time_elapsed > 0 && reserve_0 != 0 && reserve_1 != 0 {
            self.data().price_0_cumulative_last +=
                <u128>::from(uq_112_x_112.encode(reserve_1)?.uqdiv(reserve_0)?) * time_elapsed;
            self.data().price_1_cumulative_last +=
                <u128>::from(uq_112_x_112.encode(reserve_0)?.uqdiv(reserve_1)?) * time_elapsed;
        }
        self.data().reserve_0 = <u128>::from(balance_0);
        self.data().reserve_1 = <u128>::from(balance_1);
        self.data().block_timestamp_last = block_timestamp;
        self._emit_sync(self.data().reserve_0, self.data().reserve_1);
        Ok(())
    }

    /// overflow is desired
    /// * never overflows, and + overflow is desired
    /// if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    default fn _mint_fee(&mut self, reserve_0: u128, reserve_1: u128) -> Result<bool, Error> {
        let mut fee_on = Default::default();
        let mut fee_to: AccountId = i_uniswap_v_2_factory(self.data().factory)?.fee_to()?;
        fee_on = fee_to != ZERO_ADDRESS.into();
        let mut k_last: u128 = self.data().k_last;
        if fee_on {
            if k_last != 0 {
                let mut root_k: u128 = math.sqrt(<u128>::from(reserve_0).mul(reserve_1)?)?;
                let mut root_k_last: u128 = math.sqrt(k_last)?;
                if root_k > root_k_last {
                    let mut numerator: u128 = total_supply.mul(root_k.sub(root_k_last)?)?;
                    let mut denominator: u128 = root_k.mul(5)?.add(root_k_last)?;
                    let mut liquidity: u128 = numerator / denominator;
                    if liquidity > 0 {
                        mint(fee_to, liquidity)?;
                    }
                }
            }
        } else if k_last != 0 {
            self.data().k_last = 0;
        }
        Ok(fee_on)
    }

    default fn _emit_mint(&self, _: AccountId, _: u128, _: u128) {}

    default fn _emit_burn(&self, _: AccountId, _: u128, _: u128, _: AccountId) {}

    default fn _emit_swap(&self, _: AccountId, _: u128, _: u128, _: u128, _: u128, _: AccountId) {}

    default fn _emit_sync(&self, _: u128, _: u128) {}

}
