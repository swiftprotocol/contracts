use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex, UniqueIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PendingReview {
    pub peer: Addr,
    pub reviewer: Addr,
    pub commerce_contract: Addr,
    pub order_id: u64,
    pub expires_at: Timestamp,
}

pub struct PendingReviewIndexes<'a> {
    pub by_peer: UniqueIndex<'a, Addr, PendingReview>,
    pub by_reviewer: MultiIndex<'a, Addr, PendingReview, Addr>,
}

impl<'a> IndexList<PendingReview> for PendingReviewIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<PendingReview>> + '_> {
        let v: Vec<&dyn Index<PendingReview>> = vec![&self.by_peer, &self.by_reviewer];
        Box::new(v.into_iter())
    }
}

pub fn pending_reviews<'a>() -> IndexedMap<'a, Addr, PendingReview, PendingReviewIndexes<'a>> {
    let indexes = PendingReviewIndexes {
        by_reviewer: MultiIndex::new(
            |_, d: &PendingReview| d.peer.clone(),
            "pending_reviews",
            "pending_reviews__reviewer",
        ),
        by_peer: UniqueIndex::new(
            |d: &PendingReview| d.reviewer.clone(),
            "pending_reviews__peer",
        ),
    };

    IndexedMap::new("pending_reviews", indexes)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrustInfo {
    pub score: Decimal,
    pub data: TrustData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrustData {
    /// Amount of days the user has been staking k tokens
    /// The timer resets if any tokens are undelegated
    pub stake_days: u64,
    /// Amount of tokens currently staked
    pub stake_amount: Uint128,
    /// Amount of tokens staked, as queried in the previous run
    pub prev_stake_amount: Uint128,
    /// Rating score, as defined by thumbs-up/down reviews and dispute decisions
    /// Thumbs up +1, Thumbs down -1, Win dispute +0, Lose dispute -5
    pub rating: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrustScoreParams {
    pub base_score: u64,
    pub rating_multiplier: u64,
    pub stake_amount_denominator: u64,
    pub min_stake_days: u64,
    pub rating_floor_denominator: u64,
    pub denom_multiplier: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ReviewResult {
    ThumbsUp,
    ThumbsDown,
}

pub const TRUST_INFO: Map<&Addr, TrustInfo> = Map::new("trust_info");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Address of the DAO governing the contract
    pub admin: Addr,
    /// Address of the client used for updating staking info
    pub maintainer: Addr,
    /// CW20 staking contract
    pub staking_contract: Addr,
    /// Commerce contract CodeID
    pub commerce_code_id: u64,
    /// Interval between reviews in seconds
    pub review_interval: u64,
    /// Max amount of tokens taken into consideration for the staking calculation
    pub max_staked_tokens: Uint128,
    /// Max amount of days staked taken into consideration for the staking calc
    pub max_staked_days: u64,
    /// Maximum rating score (prevents inflated scores)
    pub max_rating: u64,
    /// How we calculate the trust score
    /// Should be adjusted based on token allocation/price/TVL
    pub trust_score_params: TrustScoreParams,
}

pub const CONFIG: Item<Config> = Item::new("config");
