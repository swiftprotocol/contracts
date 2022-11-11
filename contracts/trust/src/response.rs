use crate::state::{Config, PendingReview, TrustInfo};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct TrustInfoResponse {
    pub trust_info: Option<TrustInfo>,
}

#[cw_serde]
pub struct StakeAmountResponse {
    pub stake_amount: Option<Uint128>,
}

#[cw_serde]
pub struct AccountsResponse {
    pub accounts: Vec<Addr>,
}

#[cw_serde]
pub struct PendingReviewResponse {
    pub pending_review: Option<PendingReview>,
}

#[cw_serde]
pub struct PendingReviewsResponse {
    pub pending_reviews: Vec<PendingReview>,
}
