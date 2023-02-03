// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
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
    pub weth: AccountId,
    pub _reserved: Option<()>,
}

#[modifier_definition]
pub fn ensure<T, F, R>(instance: &mut T, body: F, deadline: u128) -> Result<R, Error>
where
    T: UniswapV2Router02,
    F: FnOnce(&mut T) -> Result<R, Error>,
{
    if !(deadline >= block.timestamp) {
        return Err(Error::Custom(String::from("UniswapV2Router: EXPIRED")))
    };
    body(instance);
}


impl<T: Storage<Data>> UniswapV2Router02 for T {
    /// create the pair if it doesn't exist yet
    #[modifiers(ensure(deadline))]
    fn add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: u128,
        amount_b_desired: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128, u128), Error> {
        let mut amount_a = Default::default();
        let mut amount_b = Default::default();
        let mut liquidity = Default::default();
        (_, _) = self._add_liquidity(
            token_a,
            token_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
        )?;
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token_a, token_b)?;
        transfer_helper.safe_transfer_from(token_a, Self::env().caller(), pair, amount_a)?;
        transfer_helper.safe_transfer_from(token_b, Self::env().caller(), pair, amount_b)?;
        liquidity = i_uniswap_v_2_pair(pair)?.mint(to)?;
        Ok((amount_a, amount_b, liquidity))
    }

    #[modifiers(ensure(deadline))]
    fn add_liquidity_eth(
        &mut self,
        token: AccountId,
        amount_token_desired: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128, u128), Error> {
        let mut amount_token = Default::default();
        let mut amount_eth = Default::default();
        let mut liquidity = Default::default();
        (_, _) = self._add_liquidity(
            token,
            self.data().weth,
            amount_token_desired,
            Self::env().transferred_value(),
            amount_token_min,
            amount_eth_min,
        )?;
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token, self.data().weth)?;
        transfer_helper.safe_transfer_from(token, Self::env().caller(), pair, amount_token)?;
        iweth(self.data().weth)?
            .deposit()
            .transferred_value(amount_eth)?;
        assert(iweth(self.data().weth)?.transfer(pair, amount_eth)?)?;
        liquidity = i_uniswap_v_2_pair(pair)?.mint(to)?;
        if Self::env().transferred_value() > amount_eth {
            transfer_helper.safe_transfer_eth(
                Self::env().caller(),
                Self::env().transferred_value() - amount_eth,
            )?;
        }
        Ok((amount_token, amount_eth, liquidity))
    }

    /// refund dust eth, if any
    /// **** REMOVE LIQUIDITY ****
    #[modifiers(ensure(deadline))]
    fn remove_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128), Error> {
        let mut amount_a = Default::default();
        let mut amount_b = Default::default();
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token_a, token_b)?;
        i_uniswap_v_2_pair(pair)?.transfer_from(Self::env().caller(), pair, liquidity)?;
        (amount_0, amount_1) = i_uniswap_v_2_pair(pair)?.burn(to)?;
        (token_0, _) = uniswap_v_2_library.sort_tokens(token_a, token_b)?;
        (_, _) = if token_a == token_0 { (_, _) } else { (_, _) };
        if !(amount_a >= amount_a_min) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_A_AMOUNT",
            )))
        };
        if !(amount_b >= amount_b_min) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_B_AMOUNT",
            )))
        };
        Ok((amount_a, amount_b))
    }

    /// send liquidity to pair
    #[modifiers(ensure(deadline))]
    fn remove_liquidity_eth(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(u128, u128), Error> {
        let mut amount_token = Default::default();
        let mut amount_eth = Default::default();
        (_, _) = self.remove_liquidity(
            token,
            self.data().weth,
            liquidity,
            amount_token_min,
            amount_eth_min,
            Self::env().account_id(),
            deadline,
        )?;
        transfer_helper.safe_transfer(token, to, amount_token)?;
        iweth(self.data().weth)?.withdraw(amount_eth)?;
        transfer_helper.safe_transfer_eth(to, amount_eth)?;
        Ok((amount_token, amount_eth))
    }

    fn remove_liquidity_with_permit(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        liquidity: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        to: AccountId,
        deadline: u128,
        approve_max: bool,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(u128, u128), Error> {
        let mut amount_a = Default::default();
        let mut amount_b = Default::default();
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token_a, token_b)?;
        let mut value: u128 = if approve_max {
            <u128>::from(-1)
        } else {
            liquidity
        };
        i_uniswap_v_2_pair(pair)?.permit(
            Self::env().caller(),
            Self::env().account_id(),
            value,
            deadline,
            v,
            r,
            s,
        )?;
        (_, _) = self.remove_liquidity(
            token_a,
            token_b,
            liquidity,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        )?;
        Ok((amount_a, amount_b))
    }

    fn remove_liquidity_eth_with_permit(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
        approve_max: bool,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<(u128, u128), Error> {
        let mut amount_token = Default::default();
        let mut amount_eth = Default::default();
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token, self.data().weth)?;
        let mut value: u128 = if approve_max {
            <u128>::from(-1)
        } else {
            liquidity
        };
        i_uniswap_v_2_pair(pair)?.permit(
            Self::env().caller(),
            Self::env().account_id(),
            value,
            deadline,
            v,
            r,
            s,
        )?;
        (_, _) = self.remove_liquidity_eth(
            token,
            liquidity,
            amount_token_min,
            amount_eth_min,
            to,
            deadline,
        )?;
        Ok((amount_token, amount_eth))
    }

    /// **** REMOVE LIQUIDITY (supporting fee-on-transfer tokens) ****
    #[modifiers(ensure(deadline))]
    fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<u128, Error> {
        let mut amount_eth = Default::default();
        (_, _) = self.remove_liquidity(
            token,
            self.data().weth,
            liquidity,
            amount_token_min,
            amount_eth_min,
            Self::env().account_id(),
            deadline,
        )?;
        transfer_helper.safe_transfer(
            token,
            to,
            ierc_20(token)?.balance_of(Self::env().account_id())?,
        )?;
        iweth(self.data().weth)?.withdraw(amount_eth)?;
        transfer_helper.safe_transfer_eth(to, amount_eth)?;
        Ok(amount_eth)
    }

    fn remove_liquidity_eth_with_permit_supporting_fee_on_transfer_tokens(
        &mut self,
        token: AccountId,
        liquidity: u128,
        amount_token_min: u128,
        amount_eth_min: u128,
        to: AccountId,
        deadline: u128,
        approve_max: bool,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    ) -> Result<u128, Error> {
        let mut amount_eth = Default::default();
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token, self.data().weth)?;
        let mut value: u128 = if approve_max {
            <u128>::from(-1)
        } else {
            liquidity
        };
        i_uniswap_v_2_pair(pair)?.permit(
            Self::env().caller(),
            Self::env().account_id(),
            value,
            deadline,
            v,
            r,
            s,
        )?;
        amount_eth = self.remove_liquidity_eth_supporting_fee_on_transfer_tokens(
            token,
            liquidity,
            amount_token_min,
            amount_eth_min,
            to,
            deadline,
        )?;
        Ok(amount_eth)
    }

    #[modifiers(ensure(deadline))]
    fn swap_exact_tokens_for_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        amounts = uniswap_v_2_library.get_amounts_out(self.data().factory, amount_in, path)?;
        if !(amounts[amounts.length - 1] >= amount_out_min) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        transfer_helper.safe_transfer_from(
            path[0],
            Self::env().caller(),
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amounts[0],
        )?;
        self._swap(amounts, path, to)?;
        Ok(amounts)
    }

    #[modifiers(ensure(deadline))]
    fn swap_tokens_for_exact_tokens(
        &mut self,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        amounts = uniswap_v_2_library.get_amounts_in(self.data().factory, amount_out, path)?;
        if !(amounts[0] <= amount_in_max) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: EXCESSIVE_INPUT_AMOUNT",
            )))
        };
        transfer_helper.safe_transfer_from(
            path[0],
            Self::env().caller(),
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amounts[0],
        )?;
        self._swap(amounts, path, to)?;
        Ok(amounts)
    }

    #[modifiers(ensure(deadline))]
    fn swap_exact_eth_for_tokens(
        &mut self,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        if !(path[0] == self.data().weth) {
            return Err(Error::Custom(String::from("UniswapV2Router: INVALID_PATH")))
        };
        amounts = uniswap_v_2_library.get_amounts_out(
            self.data().factory,
            Self::env().transferred_value(),
            path,
        )?;
        if !(amounts[amounts.length - 1] >= amount_out_min) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        iweth(self.data().weth)?
            .deposit()
            .transferred_value(amounts[0])?;
        assert(iweth(self.data().weth)?.transfer(
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amounts[0],
        )?)?;
        self._swap(amounts, path, to)?;
        Ok(amounts)
    }

    #[modifiers(ensure(deadline))]
    fn swap_tokens_for_exact_eth(
        &mut self,
        amount_out: u128,
        amount_in_max: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        if !(path[path.length - 1] == self.data().weth) {
            return Err(Error::Custom(String::from("UniswapV2Router: INVALID_PATH")))
        };
        amounts = uniswap_v_2_library.get_amounts_in(self.data().factory, amount_out, path)?;
        if !(amounts[0] <= amount_in_max) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: EXCESSIVE_INPUT_AMOUNT",
            )))
        };
        transfer_helper.safe_transfer_from(
            path[0],
            Self::env().caller(),
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amounts[0],
        )?;
        self._swap(amounts, path, Self::env().account_id())?;
        iweth(self.data().weth)?.withdraw(amounts[amounts.length - 1])?;
        transfer_helper.safe_transfer_eth(to, amounts[amounts.length - 1])?;
        Ok(amounts)
    }

    #[modifiers(ensure(deadline))]
    fn swap_exact_tokens_for_eth(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        if !(path[path.length - 1] == self.data().weth) {
            return Err(Error::Custom(String::from("UniswapV2Router: INVALID_PATH")))
        };
        amounts = uniswap_v_2_library.get_amounts_out(self.data().factory, amount_in, path)?;
        if !(amounts[amounts.length - 1] >= amount_out_min) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        transfer_helper.safe_transfer_from(
            path[0],
            Self::env().caller(),
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amounts[0],
        )?;
        self._swap(amounts, path, Self::env().account_id())?;
        iweth(self.data().weth)?.withdraw(amounts[amounts.length - 1])?;
        transfer_helper.safe_transfer_eth(to, amounts[amounts.length - 1])?;
        Ok(amounts)
    }

    #[modifiers(ensure(deadline))]
    fn swap_eth_for_exact_tokens(
        &mut self,
        amount_out: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        if !(path[0] == self.data().weth) {
            return Err(Error::Custom(String::from("UniswapV2Router: INVALID_PATH")))
        };
        amounts = uniswap_v_2_library.get_amounts_in(self.data().factory, amount_out, path)?;
        if !(amounts[0] <= Self::env().transferred_value()) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: EXCESSIVE_INPUT_AMOUNT",
            )))
        };
        iweth(self.data().weth)?
            .deposit()
            .transferred_value(amounts[0])?;
        assert(iweth(self.data().weth)?.transfer(
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amounts[0],
        )?)?;
        self._swap(amounts, path, to)?;
        if Self::env().transferred_value() > amounts[0] {
            transfer_helper.safe_transfer_eth(
                Self::env().caller(),
                Self::env().transferred_value() - amounts[0],
            )?;
        }
        Ok(amounts)
    }

    /// scope to avoid stack too deep errors
    #[modifiers(ensure(deadline))]
    fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        transfer_helper.safe_transfer_from(
            path[0],
            Self::env().caller(),
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amount_in,
        )?;
        let mut balance_before: u128 = ierc_20(path[path.length - 1])?.balance_of(to)?;
        self._swap_supporting_fee_on_transfer_tokens(path, to)?;
        if !(ierc_20(path[path.length - 1])?
            .balance_of(to)?
            .sub(balance_before)?
            >= amount_out_min)
        {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        Ok(())
    }

    #[modifiers(ensure(deadline))]
    fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
        &mut self,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        if !(path[0] == self.data().weth) {
            return Err(Error::Custom(String::from("UniswapV2Router: INVALID_PATH")))
        };
        let mut amount_in: u128 = Self::env().transferred_value();
        iweth(self.data().weth)?
            .deposit()
            .transferred_value(amount_in)?;
        assert(iweth(self.data().weth)?.transfer(
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amount_in,
        )?)?;
        let mut balance_before: u128 = ierc_20(path[path.length - 1])?.balance_of(to)?;
        self._swap_supporting_fee_on_transfer_tokens(path, to)?;
        if !(ierc_20(path[path.length - 1])?
            .balance_of(to)?
            .sub(balance_before)?
            >= amount_out_min)
        {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        Ok(())
    }

    #[modifiers(ensure(deadline))]
    fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<AccountId>,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        if !(path[path.length - 1] == self.data().weth) {
            return Err(Error::Custom(String::from("UniswapV2Router: INVALID_PATH")))
        };
        transfer_helper.safe_transfer_from(
            path[0],
            Self::env().caller(),
            uniswap_v_2_library.pair_for(self.data().factory, path[0], path[1])?,
            amount_in,
        )?;
        self._swap_supporting_fee_on_transfer_tokens(path, Self::env().account_id())?;
        let mut amount_out: u128 =
            ierc_20(self.data().weth)?.balance_of(Self::env().account_id())?;
        if !(amount_out >= amount_out_min) {
            return Err(Error::Custom(String::from(
                "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT",
            )))
        };
        iweth(self.data().weth)?.withdraw(amount_out)?;
        transfer_helper.safe_transfer_eth(to, amount_out)?;
        Ok(())
    }

    /// **** LIBRARY FUNCTIONS ****
    fn quote(&self, amount_a: u128, reserve_a: u128, reserve_b: u128) -> Result<u128, Error> {
        let mut amount_b = Default::default();
        return Ok(uniswap_v_2_library.quote(amount_a, reserve_a, reserve_b)?)
    }

    fn get_amount_out(
        &self,
        amount_in: u128,
        reserve_in: u128,
        reserve_out: u128,
    ) -> Result<u128, Error> {
        let mut amount_out = Default::default();
        return Ok(uniswap_v_2_library.get_amount_out(amount_in, reserve_in, reserve_out)?)
    }

    fn get_amount_in(
        &self,
        amount_out: u128,
        reserve_in: u128,
        reserve_out: u128,
    ) -> Result<u128, Error> {
        let mut amount_in = Default::default();
        return Ok(uniswap_v_2_library.get_amount_in(amount_out, reserve_in, reserve_out)?)
    }

    fn get_amounts_out(&self, amount_in: u128, path: Vec<AccountId>) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        return Ok(uniswap_v_2_library.get_amounts_out(self.data().factory, amount_in, path)?)
    }

    fn get_amounts_in(&self, amount_out: u128, path: Vec<AccountId>) -> Result<Vec<u128>, Error> {
        let mut amounts = Default::default();
        return Ok(uniswap_v_2_library.get_amounts_in(self.data().factory, amount_out, path)?)
    }

    fn factory(&self) -> AccountId {
        self.data().factory
    }

    fn weth(&self) -> AccountId {
        self.data().weth
    }

}

pub trait Internal {
    /// only accept ETH via fallback from the WETH contract
    /// **** ADD LIQUIDITY ****
    fn _add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: u128,
        amount_b_desired: u128,
        amount_a_min: u128,
        amount_b_min: u128,
    ) -> Result<(u128, u128), Error>;

    /// **** SWAP ****
    /// requires the initial amount to have already been sent to the first pair
    fn _swap(
        &mut self,
        amounts: Vec<u128>,
        path: Vec<AccountId>,
        to: AccountId,
    ) -> Result<(), Error>;

    /// refund dust eth, if any
    /// **** SWAP (supporting fee-on-transfer tokens) ****
    /// requires the initial amount to have already been sent to the first pair
    fn _swap_supporting_fee_on_transfer_tokens(
        &mut self,
        path: Vec<AccountId>,
        to: AccountId,
    ) -> Result<(), Error>;

}

impl<T: Storage<Data>> Internal for T {
    /// only accept ETH via fallback from the WETH contract
    /// **** ADD LIQUIDITY ****
    default fn _add_liquidity(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        amount_a_desired: u128,
        amount_b_desired: u128,
        amount_a_min: u128,
        amount_b_min: u128,
    ) -> Result<(u128, u128), Error> {
        let mut amount_a = Default::default();
        let mut amount_b = Default::default();
        if i_uniswap_v_2_factory(self.data().factory)?.get_pair(token_a, token_b)?
            == ZERO_ADDRESS.into()
        {
            i_uniswap_v_2_factory(self.data().factory)?.create_pair(token_a, token_b)?;
        }
        (reserve_a, reserve_b) =
            uniswap_v_2_library.get_reserves(self.data().factory, token_a, token_b)?;
        if reserve_a == 0 && reserve_b == 0 {
            (_, _) = (_, _);
        } else {
            let mut amount_b_optimal: u128 =
                uniswap_v_2_library.quote(amount_a_desired, reserve_a, reserve_b)?;
            if amount_b_optimal <= amount_b_desired {
                if !(amount_b_optimal >= amount_b_min) {
                    return Err(Error::Custom(String::from(
                        "UniswapV2Router: INSUFFICIENT_B_AMOUNT",
                    )))
                };
                (_, _) = (_, _);
            } else {
                let mut amount_a_optimal: u128 =
                    uniswap_v_2_library.quote(amount_b_desired, reserve_b, reserve_a)?;
                assert(amount_a_optimal <= amount_a_desired)?;
                if !(amount_a_optimal >= amount_a_min) {
                    return Err(Error::Custom(String::from(
                        "UniswapV2Router: INSUFFICIENT_A_AMOUNT",
                    )))
                };
                (_, _) = (_, _);
            }
        }
        Ok((amount_a, amount_b))
    }

    /// **** SWAP ****
    /// requires the initial amount to have already been sent to the first pair
    default fn _swap(
        &mut self,
        amounts: Vec<u128>,
        path: Vec<AccountId>,
        to: AccountId,
    ) -> Result<(), Error> {
        while i < path.length - 1 {
            (input, output) = (_, _);
            (token_0, _) = uniswap_v_2_library.sort_tokens(input, output)?;
            let mut amount_out: u128 = amounts[i + 1];
            (amount_0_out, amount_1_out) = if input == token_0 { (_, _) } else { (_, _) };
            let mut to: AccountId = if i < path.length - 2 {
                uniswap_v_2_library.pair_for(self.data().factory, output, path[i + 2])?
            } else {
                to
            };
            i_uniswap_v_2_pair(uniswap_v_2_library.pair_for(
                self.data().factory,
                input,
                output,
            )?)?
            .swap(amount_0_out, amount_1_out, to, Vec::with_capacity(0))?;
            i += 1;
        }
        Ok(())
    }

    /// refund dust eth, if any
    /// **** SWAP (supporting fee-on-transfer tokens) ****
    /// requires the initial amount to have already been sent to the first pair
    default fn _swap_supporting_fee_on_transfer_tokens(
        &mut self,
        path: Vec<AccountId>,
        to: AccountId,
    ) -> Result<(), Error> {
        while i < path.length - 1 {
            (input, output) = (_, _);
            (token_0, _) = uniswap_v_2_library.sort_tokens(input, output)?;
            let mut pair: IUniswapV2Pair = i_uniswap_v_2_pair(uniswap_v_2_library.pair_for(
                self.data().factory,
                input,
                output,
            )?)?;
            (reserve_0, reserve_1, _) = pair.get_reserves()?;
            (reserve_input, reserve_output) = if input == token_0 { (_, _) } else { (_, _) };
            amount_input = ierc_20(input)?
                .balance_of(AccountId::from(pair))?
                .sub(reserve_input)?;
            amount_output =
                uniswap_v_2_library.get_amount_out(amount_input, reserve_input, reserve_output)?;
            (amount_0_out, amount_1_out) = if input == token_0 { (_, _) } else { (_, _) };
            let mut to: AccountId = if i < path.length - 2 {
                uniswap_v_2_library.pair_for(self.data().factory, output, path[i + 2])?
            } else {
                to
            };
            pair.swap(amount_0_out, amount_1_out, to, Vec::with_capacity(0))?;
            i += 1;
        }
        Ok(())
    }

}
