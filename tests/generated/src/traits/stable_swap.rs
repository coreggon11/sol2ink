// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use ink_prelude::vec::*;
pub use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        AccountIdExt,
        String,
        ZERO_ADDRESS,
    },
};
use scale::{
    Decode,
    Encode,
};

#[derive(Debug, Encode, Decode, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
}



#[openbrush::wrapper]
pub type StableSwapRef = dyn StableSwap;

#[openbrush::trait_definition]
pub trait StableSwap {
    /// Newton's method
    /// Initial guess, y <= d
    /// Estimate value of 1 share
    /// How many tokens is one share worth?
    #[ink(message)]
    fn get_virtual_price(&self) -> Result<u128, Error>;

    /// @notice Swap dx amount of token i for token j
    /// @param i Index of token in
    /// @param j Index of token out
    /// @param dx Token in amount
    /// @param minDy Minimum token out
    #[ink(message)]
    fn swap(&mut self, i: u128, j: u128, dx: u128, min_dy: u128) -> Result<u128, Error>;

    /// Calculate dy
    /// y0 must be >= y1, since x has increased
    /// -1 to round down
    /// Subtract fee from dy
    #[ink(message)]
    fn add_liquidity(&mut self, amounts: Vec<u128>, min_shares: u128) -> Result<u128, Error>;

    /// calculate current liquidity d0
    /// Transfer tokens in
    /// Calculate new liquidity d1
    /// Reccalcuate D accounting for fee on imbalance
    /// why old_xs[i] * d1 / d0? why not d1 / N?
    /// Update balances
    /// Shares to mint = (d2 - d0) / d0 * total supply
    /// d1 >= d2 >= d0
    #[ink(message)]
    fn remove_liquidity(
        &mut self,
        shares: u128,
        min_amounts_out: Vec<u128>,
    ) -> Result<Vec<u128>, Error>;

    /// Calculate d0 and d1
    /// Calculate reduction in y if D = d1
    /// d1 <= d0 so y must be <= xp[i]
    /// Calculate imbalance fee, update xp with fees
    /// d1 / d0 <= 1
    /// Recalculate y with xp including imbalance fees
    /// - 1 to round down
    #[ink(message)]
    fn calc_withdraw_one_token(&self, shares: u128, i: u128) -> Result<(u128, u128), Error>;

    /// @notice Withdraw liquidity in token i
    /// @param shares Shares to burn
    /// @param i Token to withdraw
    /// @param minAmountOut Minimum amount of token i that must be withdrawn
    #[ink(message)]
    fn remove_liquidity_one_token(
        &mut self,
        shares: u128,
        i: u128,
        min_amount_out: u128,
    ) -> Result<u128, Error>;

}
