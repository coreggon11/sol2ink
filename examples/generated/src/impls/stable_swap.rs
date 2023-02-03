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
    pub tokens: Vec<AccountId>,
    /// Normalize each token to 18 decimals
    /// Example - DAI (18 decimals), USDC (6 decimals), USDT (6 decimals)
    pub multipliers: Vec<u128>,
    pub balances: Vec<u128>,
    pub total_supply: u128,
    pub balance_of: Mapping<AccountId, u128>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> StableSwap for T {
    /// Newton's method
    /// Initial guess, y <= d
    /// Estimate value of 1 share
    /// How many tokens is one share worth?
    fn get_virtual_price(&self) -> Result<u128, Error> {
        let mut d: u128 = self._get_d(self._xp()?)?;
        let mut total_supply: u128 = self.data().total_supply;
        if self.data().total_supply > 0 {
            return Ok((d * 10.pow(DECIMALS)) / self.data().total_supply)
        }
        return Ok(0)
    }

    /// @notice Swap dx amount of token i for token j
    /// @param i Index of token in
    /// @param j Index of token out
    /// @param dx Token in amount
    /// @param minDy Minimum token out
    fn swap(&mut self, i: u128, j: u128, dx: u128, min_dy: u128) -> Result<u128, Error> {
        let mut dy = Default::default();
        if !(i != j) {
            return Err(Error::Custom(String::from("i = j")))
        };
        ierc_20(self.data().tokens[i])?.transfer_from(
            Self::env().caller(),
            Self::env().account_id(),
            dx,
        )?;
        let mut xp: Vec<u128> = self._xp()?;
        let mut x: u128 = xp[i] + dx * self.data().multipliers[i];
        let mut y_0: u128 = xp[j];
        let mut y_1: u128 = self._get_y(i, j, self.data().x, xp)?;
        dy = (y_0 - y_1 - 1) / self.data().multipliers[j];
        let mut fee: u128 = (dy * SWAP_FEE) / FEE_DENOMINATOR;
        dy -= fee;
        if !(dy >= min_dy) {
            return Err(Error::Custom(String::from("dy < min")))
        };
        self.data().balances[i] += dx;
        self.data().balances[j] -= dy;
        ierc_20(self.data().tokens[j])?.transfer(Self::env().caller(), dy)?;
        Ok(dy)
    }

    /// Calculate dy
    /// y0 must be >= y1, since x has increased
    /// -1 to round down
    /// Subtract fee from dy
    fn add_liquidity(&mut self, amounts: Vec<u128>, min_shares: u128) -> Result<u128, Error> {
        let mut shares = Default::default();
        let mut total_supply: u128 = self.data().total_supply;
        let mut old_xs: Vec<u128> = self._xp()?;
        if self.data().total_supply > 0 {
            d_0 = self._get_d(old_xs)?;
        };
        while i < N {
            let mut amount: u128 = amounts[i];
            if amount > 0 {
                ierc_20(self.data().tokens[i])?.transfer_from(
                    Self::env().caller(),
                    Self::env().account_id(),
                    amount,
                )?;
                new_xs[i] = old_xs[i] + amount * self.data().multipliers[i];
            } else {
                new_xs[i] = old_xs[i];
            }
            i += 1;
        }
        let mut d_1: u128 = self._get_d(new_xs)?;
        if !(d_1 > d_0) {
            return Err(Error::Custom(String::from("liquidity didn't increase")))
        };
        if self.data().total_supply > 0 {
            while i < N {
                let mut ideal_balance: u128 = (old_xs[i] * d_1) / d_0;
                let mut diff: u128 = math.abs(new_xs[i], ideal_balance)?;
                new_xs[i] -= (LIQUIDITY_FEE * diff) / FEE_DENOMINATOR;
                i += 1;
            }
            d_2 = self._get_d(new_xs)?;
        } else {
            d_2 = d_1;
        };
        while i < N {
            self.data().balances[i] += amounts[i];
            i += 1;
        }
        if self.data().total_supply > 0 {
            shares = ((d_2 - d_0) * self.data().total_supply) / d_0;
        } else {
            shares = d_2;
        }
        if !(shares >= min_shares) {
            return Err(Error::Custom(String::from("shares < min")))
        };
        self._mint(Self::env().caller(), shares)?;
        Ok(shares)
    }

    /// calculate current liquidity d0
    /// Transfer tokens in
    /// Calculate new liquidity d1
    /// Reccalcuate D accounting for fee on imbalance
    /// why old_xs[i] * d1 / d0? why not d1 / N?
    /// Update balances
    /// Shares to mint = (d2 - d0) / d0 * total supply
    /// d1 >= d2 >= d0
    fn remove_liquidity(
        &mut self,
        shares: u128,
        min_amounts_out: Vec<u128>,
    ) -> Result<Vec<u128>, Error> {
        let mut amounts_out = Default::default();
        let mut total_supply: u128 = self.data().total_supply;
        while i < N {
            let mut amount_out: u128 =
                (self.data().balances[i] * shares) / self.data().total_supply;
            if !(amount_out >= min_amounts_out[i]) {
                return Err(Error::Custom(String::from("out < min")))
            };
            self.data().balances[i] -= amount_out;
            amounts_out[i] = amount_out;
            ierc_20(self.data().tokens[i])?.transfer(Self::env().caller(), amount_out)?;
            i += 1;
        }
        self._burn(Self::env().caller(), shares)?;
        Ok(amounts_out)
    }

    /// Calculate d0 and d1
    /// Calculate reduction in y if D = d1
    /// d1 <= d0 so y must be <= xp[i]
    /// Calculate imbalance fee, update xp with fees
    /// d1 / d0 <= 1
    /// Recalculate y with xp including imbalance fees
    /// - 1 to round down
    fn calc_withdraw_one_token(&self, shares: u128, i: u128) -> Result<(u128, u128), Error> {
        let mut dy = Default::default();
        let mut fee = Default::default();
        return Ok(self._calc_withdraw_one_token(shares, i)?)
    }

    /// @notice Withdraw liquidity in token i
    /// @param shares Shares to burn
    /// @param i Token to withdraw
    /// @param minAmountOut Minimum amount of token i that must be withdrawn
    fn remove_liquidity_one_token(
        &mut self,
        shares: u128,
        i: u128,
        min_amount_out: u128,
    ) -> Result<u128, Error> {
        let mut amount_out = Default::default();
        (_, _) = self._calc_withdraw_one_token(shares, i)?;
        if !(amount_out >= min_amount_out) {
            return Err(Error::Custom(String::from("out < min")))
        };
        self.data().balances[i] -= amount_out;
        self._burn(Self::env().caller(), shares)?;
        ierc_20(self.data().tokens[i])?.transfer(Self::env().caller(), amount_out)?;
        Ok(amount_out)
    }

    fn tokens(&self) -> Vec<AccountId> {
        self.data().tokens
    }

    fn balances(&self) -> Vec<u128> {
        self.data().balances
    }

    fn total_supply(&self) -> u128 {
        self.data().total_supply
    }

    fn balance_of(&self) -> Mapping<AccountId, u128> {
        self.data().balance_of
    }

}

pub trait Internal {
    fn _mint(&mut self, to: AccountId, amount: u128) -> Result<(), Error>;

    fn _burn(&mut self, from: AccountId, amount: u128) -> Result<(), Error>;

    /// Return precision-adjusted balances, adjusted to 18 decimals
    fn _xp(&self) -> Result<Vec<u128>, Error>;

    /// @notice Calculate D, sum of balances in a perfectly balanced pool
    /// If balances of x_0, x_1, ... x_(n-1) then sum(x_i) = D
    /// @param xp Precision-adjusted balances
    /// @return D
    fn _get_d(&self, xp: Vec<u128>) -> Result<u128, Error>;

    ///Newton's method to compute D
    ///        -----------------------------
    ///        f(D) = ADn^n + D^(n + 1) / (n^n prod(x_i)) - An^n sum(x_i) - D
    ///        f'(D) = An^n + (n + 1) D^n / (n^n prod(x_i)) - 1
    ///
    ///                     (as + np)D_n
    ///        D_(n+1) = -----------------------
    ///                  (a - 1)D_n + (n + 1)p
    ///
    ///        a = An^n
    ///        s = sum(x_i)
    ///        p = (D_n)^(n + 1) / (n^n prod(x_i))
    /// An^n
    /// x_0 + x_1 + ... + x_(n-1)
    /// Newton's method
    /// Initial guess, d <= s
    /// p = D^(n + 1) / (n^n * x_0 * ... * x_(n-1))
    /// @notice Calculate the new balance of token j given the new balance of token i
    /// @param i Index of token in
    /// @param j Index of token out
    /// @param x New balance of token i
    /// @param xp Current precision-adjusted balances
    fn _get_y(&self, i: u128, j: u128, x: u128, xp: Vec<u128>) -> Result<u128, Error>;

    /// all others score 0
    ///Newton's method to compute y
    ///        -----------------------------
    ///        y = x_j
    ///
    ///        f(y) = y^2 + y(b - D) - c
    ///
    ///                    y_n^2 + c
    ///        y_(n+1) = --------------
    ///                   2y_n + b - D
    ///
    ///        where
    ///        s = sum(x_k), k != j
    ///        p = prod(x_k), k != j
    ///        b = s + D / (An^n)
    ///        c = D^(n + 1) / (n^n * p * An^n)
    /// Newton's method
    /// Initial guess, y <= d
    /// @notice Calculate the new balance of token i given precision-adjusted
    /// balances xp and liquidity d
    /// @dev Equation is calculate y is same as _getY
    /// @param i Index of token to calculate the new balance
    /// @param xp Precision-adjusted balances
    /// @param d Liquidity d
    /// @return New balance of token i
    fn _get_yd(&self, i: u128, xp: Vec<u128>, d: u128) -> Result<u128, Error>;

    /// @notice Calculate amount of token i to receive for shares
    /// @param shares Shares to burn
    /// @param i Index of token to withdraw
    /// @return dy Amount of token i to receive
    ///         fee Fee for withdraw. Fee already included in dy
    fn _calc_withdraw_one_token(&self, shares: u128, i: u128) -> Result<(u128, u128), Error>;

}

impl<T: Storage<Data>> Internal for T {
    default fn _mint(&mut self, to: AccountId, amount: u128) -> Result<(), Error> {
        let new_value = self.data().balance_of.get(&(to)).unwrap_or_default() + amount;
        self.data().balance_of.insert(&(to), &new_value);
        self.data().total_supply += amount;
        Ok(())
    }

    default fn _burn(&mut self, from: AccountId, amount: u128) -> Result<(), Error> {
        let new_value = self.data().balance_of.get(&(from)).unwrap_or_default() - amount;
        self.data().balance_of.insert(&(from), &new_value);
        self.data().total_supply -= amount;
        Ok(())
    }

    /// Return precision-adjusted balances, adjusted to 18 decimals
    default fn _xp(&self) -> Result<Vec<u128>, Error> {
        let mut xp = Default::default();
        while i < N {
            xp[i] = self.data().balances[i] * self.data().multipliers[i];
            i += 1;
        }
        Ok(xp)
    }

    /// @notice Calculate D, sum of balances in a perfectly balanced pool
    /// If balances of x_0, x_1, ... x_(n-1) then sum(x_i) = D
    /// @param xp Precision-adjusted balances
    /// @return D
    default fn _get_d(&self, xp: Vec<u128>) -> Result<u128, Error> {
        let mut a: u128 = A * N;
        while i < N {
            s += xp[i];
            i += 1;
        }
        let mut d: u128 = s;
        while i < 255 {
            let mut p: u128 = d;
            while j < N {
                p = (p * d) / (N * xp[j]);
                j += 1;
            }
            d_prev = d;
            d = ((a * s + N * p) * d) / ((a - 1) * d + (N + 1) * p);
            if math.abs(d, d_prev)? <= 1 {
                return Ok(d)
            }
            i += 1;
        }
        return Err(Error::Custom(String::from("_")))
    }

    ///Newton's method to compute D
    ///        -----------------------------
    ///        f(D) = ADn^n + D^(n + 1) / (n^n prod(x_i)) - An^n sum(x_i) - D
    ///        f'(D) = An^n + (n + 1) D^n / (n^n prod(x_i)) - 1
    ///
    ///                     (as + np)D_n
    ///        D_(n+1) = -----------------------
    ///                  (a - 1)D_n + (n + 1)p
    ///
    ///        a = An^n
    ///        s = sum(x_i)
    ///        p = (D_n)^(n + 1) / (n^n prod(x_i))
    /// An^n
    /// x_0 + x_1 + ... + x_(n-1)
    /// Newton's method
    /// Initial guess, d <= s
    /// p = D^(n + 1) / (n^n * x_0 * ... * x_(n-1))
    /// @notice Calculate the new balance of token j given the new balance of token i
    /// @param i Index of token in
    /// @param j Index of token out
    /// @param x New balance of token i
    /// @param xp Current precision-adjusted balances
    default fn _get_y(&self, i: u128, j: u128, x: u128, xp: Vec<u128>) -> Result<u128, Error> {
        let mut a: u128 = A * N;
        let mut d: u128 = self._get_d(xp)?;
        let mut c: u128 = d;
        while k < N {
            if k == i {
                x = self.data().x;
            } else if k == j {
                continue
            } else {
                x = xp[k];
            }
            s += x;
            c = (c * d) / (N * x);
            k += 1;
        }
        c = (c * d) / (N * a);
        let mut b: u128 = s + d / a;
        let mut y: u128 = d;
        while i < 255 {
            y_prev = y;
            y = (y * y + c) / (2 * y + b - d);
            if math.abs(y, y_prev)? <= 1 {
                return Ok(y)
            }
            i += 1;
        }
        return Err(Error::Custom(String::from("_")))
    }

    /// all others score 0
    ///Newton's method to compute y
    ///        -----------------------------
    ///        y = x_j
    ///
    ///        f(y) = y^2 + y(b - D) - c
    ///
    ///                    y_n^2 + c
    ///        y_(n+1) = --------------
    ///                   2y_n + b - D
    ///
    ///        where
    ///        s = sum(x_k), k != j
    ///        p = prod(x_k), k != j
    ///        b = s + D / (An^n)
    ///        c = D^(n + 1) / (n^n * p * An^n)
    /// Newton's method
    /// Initial guess, y <= d
    /// @notice Calculate the new balance of token i given precision-adjusted
    /// balances xp and liquidity d
    /// @dev Equation is calculate y is same as _getY
    /// @param i Index of token to calculate the new balance
    /// @param xp Precision-adjusted balances
    /// @param d Liquidity d
    /// @return New balance of token i
    default fn _get_yd(&self, i: u128, xp: Vec<u128>, d: u128) -> Result<u128, Error> {
        let mut a: u128 = A * N;
        let mut c: u128 = d;
        while k < N {
            if k != i {
                x = xp[k];
            } else {
                continue
            }
            s += x;
            c = (c * d) / (N * x);
            k += 1;
        }
        c = (c * d) / (N * a);
        let mut b: u128 = s + d / a;
        let mut y: u128 = d;
        while i < 255 {
            y_prev = y;
            y = (y * y + c) / (2 * y + b - d);
            if math.abs(y, y_prev)? <= 1 {
                return Ok(y)
            }
            i += 1;
        }
        return Err(Error::Custom(String::from("_")))
    }

    /// @notice Calculate amount of token i to receive for shares
    /// @param shares Shares to burn
    /// @param i Index of token to withdraw
    /// @return dy Amount of token i to receive
    ///         fee Fee for withdraw. Fee already included in dy
    default fn _calc_withdraw_one_token(
        &self,
        shares: u128,
        i: u128,
    ) -> Result<(u128, u128), Error> {
        let mut dy = Default::default();
        let mut fee = Default::default();
        let mut total_supply: u128 = self.data().total_supply;
        let mut xp: Vec<u128> = self._xp()?;
        let mut d_0: u128 = self._get_d(xp)?;
        let mut d_1: u128 = d_0 - (d_0 * shares) / self.data().total_supply;
        let mut y_0: u128 = self._get_yd(i, xp, d_1)?;
        let mut dy_0: u128 = (xp[i] - y_0) / self.data().multipliers[i];
        while j < N {
            if j == i {
                dx = (xp[j] * d_1) / d_0 - y_0;
            } else {
                dx = xp[j] - (xp[j] * d_1) / d_0;
            }
            xp[j] -= (LIQUIDITY_FEE * dx) / FEE_DENOMINATOR;
            j += 1;
        }
        let mut y_1: u128 = self._get_yd(i, xp, d_1)?;
        dy = (xp[i] - y_1 - 1) / self.data().multipliers[i];
        fee = dy_0 - dy;
        Ok((dy, fee))
    }

}
