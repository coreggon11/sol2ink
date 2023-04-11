// Generated with Sol2Ink v2.1.0
// https://github.com/Brushfam/sol2ink

pub use crate::{
    impls,
    traits::*,
};
pub use openbrush::traits::AccountId;
use openbrush::traits::Storage;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub router: IUniswapV2Router01,
    pub factory: AccountId,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ExampleSwapToPrice for T {
    /// swaps an amount of either token such that the trade is profit-maximizing, given an external true price
    /// true price is expressed in the ratio of token A to token B
    /// caller must approve this contract to spend whichever token is intended to be swapped
    fn swap_to_price(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        true_price_token_a: u128,
        true_price_token_b: u128,
        max_spend_token_a: u128,
        max_spend_token_b: u128,
        to: AccountId,
        deadline: u128,
    ) -> Result<(), Error> {
        if !(true_price_token_a != 0 && true_price_token_b != 0) {
            return Err(Error::Custom(String::from(
                "ExampleSwapToPrice: ZERO_PRICE",
            )))
        };
        if !(max_spend_token_a != 0 || max_spend_token_b != 0) {
            return Err(Error::Custom(String::from(
                "ExampleSwapToPrice: ZERO_SPEND",
            )))
        };
        (reserve_a, reserve_b) =
            uniswap_v_2_library.get_reserves(self.data().factory, token_a, token_b)?;
        (_, _) = uniswap_v_2_liquidity_math_library.compute_profit_maximizing_trade(
            true_price_token_a,
            true_price_token_b,
            reserve_a,
            reserve_b,
        )?;
        if !(amount_in > 0) {
            return Err(Error::Custom(String::from(
                "ExampleSwapToPrice: ZERO_AMOUNT_IN",
            )))
        };
        let mut max_spend: u128 = if a_to_b {
            max_spend_token_a
        } else {
            max_spend_token_b
        };
        if amount_in > max_spend {
            amount_in = max_spend;
        }
        let mut token_in: AccountId = if a_to_b { token_a } else { token_b };
        let mut token_out: AccountId = if a_to_b { token_b } else { token_a };
        transfer_helper.safe_transfer_from(
            token_in,
            Self::env().caller(),
            Self::env().account_id(),
            amount_in,
        )?;
        transfer_helper.safe_approve(token_in, AccountId::from(self.data().router), amount_in)?;
        let mut path: Vec<AccountId> = vec![AccountId::default(); 2];
        path[0] = token_in;
        path[1] = token_out;
        self.data()
            .router
            .swap_exact_tokens_for_tokens(amount_in, 0, path, to, deadline)?;
        Ok(())
    }

    fn router(&self) -> IUniswapV2Router01 {
        self.data().router
    }

    fn factory(&self) -> AccountId {
        self.data().factory
    }

}

pub trait Internal {}

impl<T: Storage<Data>> Internal for T {}
