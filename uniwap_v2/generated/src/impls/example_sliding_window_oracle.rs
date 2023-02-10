// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

pub use crate::{
    impls,
    traits::*,
};
use openbrush::traits::Storage;
pub use openbrush::{
    storage::Mapping,
    traits::AccountId,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory: AccountId,
    /// the desired amount of time over which the moving average should be computed, e.g. 24 hours
    pub window_size: u128,
    /// the number of observations stored for each pair, i.e. how many price observations are stored for the window.
    /// as granularity increases from 1, more frequent updates are needed, but moving averages become more precise.
    /// averages are computed over intervals with sizes in the range:
    ///   [windowSize - (windowSize / granularity) * 2, windowSize]
    /// e.g. if the window size is 24 hours, and the granularity is 24, the oracle will return the average price for
    ///   the period:
    ///   [now - [22 hours, 24 hours], now]
    pub granularity: u8,
    /// this is redundant with granularity and windowSize, but stored for gas savings & informational purposes.
    pub period_size: u128,
    /// mapping from pair address to a list of price observations of that pair
    pub pair_observations: Mapping<AccountId, Vec<Observation>>,
    pub _reserved: Option<()>,
}


impl<T: Storage<Data>> ExampleSlidingWindowOracle for T {
    /// returns the index of the observation corresponding to the given timestamp
    fn observation_index_of(&self, timestamp: u128) -> Result<u8, Error> {
        let mut index = Default::default();
        let mut epoch_period: u128 = timestamp / self.data().period_size;
        return Ok(<u8>::from(epoch_period % self.data().granularity))
    }

    /// no overflow issue. if observationIndex + 1 overflows, result is still zero.
    /// update the cumulative price for the observation at the current timestamp. each observation is updated at most
    /// once per epoch period.
    fn update(&mut self, token_a: AccountId, token_b: AccountId) -> Result<(), Error> {
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token_a, token_b)?;
        let mut i: u128 = self
            .data()
            .pair_observations
            .get(&pair)
            .unwrap_or_default()
            .length;
        while i < self.data().granularity {
            self.data()
                .pair_observations
                .get(&pair)
                .unwrap_or_default()
                .push()?;
            i += 1;
        }
        let mut observation_index: u8 = self.observation_index_of(Self::env().block_timestamp())?;
        let mut observation: Observation = self
            .data()
            .pair_observations
            .get(&(pair, observation_index))
            .unwrap_or_default();
        let mut time_elapsed: u128 = Self::env().block_timestamp() - observation.timestamp;
        if time_elapsed > self.data().period_size {
            (price_0_cumulative, price_1_cumulative, _) =
                uniswap_v_2_oracle_library.current_cumulative_prices(pair)?;
            observation.timestamp = Self::env().block_timestamp();
            observation.price_0_cumulative = price_0_cumulative;
            observation.price_1_cumulative = price_1_cumulative;
        }
        Ok(())
    }

    /// overflow is desired.
    /// returns the amount out corresponding to the amount in for a given token using the moving average over the time
    /// range [now - [windowSize, windowSize - periodSize * 2], now]
    /// update must have been called for the bucket corresponding to timestamp `now - windowSize`
    fn consult(
        &self,
        token_in: AccountId,
        amount_in: u128,
        token_out: AccountId,
    ) -> Result<u128, Error> {
        let mut amount_out = Default::default();
        let mut pair: AccountId =
            uniswap_v_2_library.pair_for(self.data().factory, token_in, token_out)?;
        let mut first_observation: Observation = self._get_first_observation_in_window(pair)?;
        let mut time_elapsed: u128 = Self::env().block_timestamp() - first_observation.timestamp;
        if !(time_elapsed <= self.data().window_size) {
            return Err(Error::Custom(String::from(
                "SlidingWindowOracle: MISSING_HISTORICAL_OBSERVATION",
            )))
        };
        if !(time_elapsed >= self.data().window_size - self.data().period_size * 2) {
            return Err(Error::Custom(String::from(
                "SlidingWindowOracle: UNEXPECTED_TIME_ELAPSED",
            )))
        };
        (price_0_cumulative, price_1_cumulative, _) =
            uniswap_v_2_oracle_library.current_cumulative_prices(pair)?;
        (token_0, _) = uniswap_v_2_library.sort_tokens(token_in, token_out)?;
        if token_0 == token_in {
            return Ok(self._compute_amount_out(
                first_observation.price_0_cumulative,
                price_0_cumulative,
                time_elapsed,
                amount_in,
            )?)
        } else {
            return Ok(self._compute_amount_out(
                first_observation.price_1_cumulative,
                price_1_cumulative,
                time_elapsed,
                amount_in,
            )?)
        }
        Ok(amount_out)
    }

    fn factory(&self) -> AccountId {
        self.data().factory
    }

    fn window_size(&self) -> u128 {
        self.data().window_size
    }

    fn granularity(&self) -> u8 {
        self.data().granularity
    }

    fn period_size(&self) -> u128 {
        self.data().period_size
    }

    fn pair_observations(&self) -> Mapping<AccountId, Vec<Observation>> {
        self.data().pair_observations
    }

}

pub trait Internal {
    /// returns the observation from the oldest epoch (at the beginning of the window) relative to the current time
    fn _get_first_observation_in_window(&self, pair: AccountId) -> Result<Observation, Error>;

    /// populate the array with empty observations (first call only)
    /// get the observation for the current period
    /// we only want to commit updates once per period (i.e. windowSize / granularity)
    /// given the cumulative prices of the start and end of a period, and the length of the period, compute the average
    /// price in terms of how much amount out is received for the amount in
    fn _compute_amount_out(
        &self,
        price_cumulative_start: u128,
        price_cumulative_end: u128,
        time_elapsed: u128,
        amount_in: u128,
    ) -> Result<u128, Error>;

}

impl<T: Storage<Data>> Internal for T {
    /// returns the observation from the oldest epoch (at the beginning of the window) relative to the current time
    default fn _get_first_observation_in_window(
        &self,
        pair: AccountId,
    ) -> Result<Observation, Error> {
        let mut first_observation = Default::default();
        let mut observation_index: u8 = self.observation_index_of(Self::env().block_timestamp())?;
        let mut first_observation_index: u8 = (observation_index + 1) % self.data().granularity;
        first_observation = self
            .data()
            .pair_observations
            .get(&(pair, first_observation_index))
            .unwrap_or_default();
        Ok(first_observation)
    }

    /// populate the array with empty observations (first call only)
    /// get the observation for the current period
    /// we only want to commit updates once per period (i.e. windowSize / granularity)
    /// given the cumulative prices of the start and end of a period, and the length of the period, compute the average
    /// price in terms of how much amount out is received for the amount in
    default fn _compute_amount_out(
        &self,
        price_cumulative_start: u128,
        price_cumulative_end: u128,
        time_elapsed: u128,
        amount_in: u128,
    ) -> Result<u128, Error> {
        let mut amount_out = Default::default();
        let mut price_average: fixed_point::uq_112_x_112 = fixed_point.uq_112_x_112(
            <u128>::from((price_cumulative_end - price_cumulative_start) / time_elapsed),
        )?;
        amount_out = price_average.mul(amount_in)?.decode_144()?;
        Ok(amount_out)
    }

}
