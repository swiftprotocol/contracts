use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

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

    let params = config.trust_score_params;

    let stake_amount = match trust_data.stake_amount > config.max_staked_tokens {
        true => config.max_staked_tokens,
        false => trust_data.stake_amount,
    };

    let stake_days = match trust_data.stake_days > config.max_staked_days {
        true => config.max_staked_days,
        false => trust_data.stake_days,
    };

    let rating_comp = params.rating_multiplier as i64 * trust_data.rating;
    let staked_comp = (stake_amount.u128() / params.denom_multiplier) as f64
        / params.stake_amount_denominator as f64;
    let rating_denom_comp = trust_data.rating as f64 / params.rating_floor_denominator as f64;

    let score =
        params.base_score as f64 + rating_comp as f64 + staked_comp as f64 * stake_days as f64
            - params.min_stake_days as f64 * rating_denom_comp;

    let trust_score = match score < 0.0 {
        true => 0.0,
        false => match score > 1500.0 {
            true => 1500.0,
            false => score,
        },
    };

    return Decimal::from_str(&trust_score.to_string()).unwrap();
}
