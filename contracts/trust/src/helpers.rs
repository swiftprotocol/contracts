use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

use crate::state::{Config, TrustData};

/// TrustContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct TrustContract(pub Addr);

impl TrustContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }
}

pub fn calculate_trust_score(trust_data: TrustData, config: Config) -> Decimal {
    // t = 500 + 25R + ( T / 250 ) * D - 14 * ( R / 10 )
    // https://medium.com/@swiftprotocol/the-case-for-decentralized-trust-in-modern-blockchain-systems-9ec69a184d1c
    // See thumbs-up/thumbs-down approach section of this article

    // We multiply everything by 1_000_000_000_000 to allow us to do decimal-point math.

    let params = config.trust_score_params;

    let base_score = params.base_score * 1_000_000_000_000;

    // Rating
    let rating = trust_data.rating * 1_000_000_000_000;

    // Stake info
    let stake_amount = (match trust_data.stake_amount > config.max_staked_tokens {
        true => config.max_staked_tokens,
        false => trust_data.stake_amount,
    }
    .u128()
        / params.denom_multiplier)
        * 1_000_000_000_000;

    let stake_days = match trust_data.stake_days > config.max_staked_days {
        true => config.max_staked_days,
        false => trust_data.stake_days,
    };

    let rating_comp = params.rating_multiplier as i64 * rating;
    let staked_comp = stake_amount / params.stake_amount_denominator as u128 * stake_days as u128;
    let rating_floor_comp =
        params.min_stake_days as i64 * (rating / params.rating_floor_denominator as i64);

    let score =
        base_score as i128 + rating_comp as i128 + staked_comp as i128 - rating_floor_comp as i128;

    let trust_score = match score < 0 {
        true => 0,
        false => match score > 1500 * 1_000_000_000_000 {
            true => 1500 * 1_000_000_000_000,
            false => score,
        },
    };

    Decimal::from_atomics(Uint128::from(trust_score as u128), 12).unwrap()
}
