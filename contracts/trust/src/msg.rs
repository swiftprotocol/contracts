use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::{
    response::{
        AccountsResponse, ConfigResponse, PendingReviewResponse, PendingReviewsResponse,
        StakeAmountResponse, TrustInfoResponse,
    },
    state::{ReviewResult, TrustScoreParams},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub maintainer: String,
    pub staking_contract: String,
    pub commerce_code_id: u64,
    pub review_interval: u64,
    pub max_staked_tokens: Uint128,
    pub max_staked_days: u64,
    pub trust_score_params: TrustScoreParams,
    pub max_rating: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// --- DAO-RESTRICTED ---
    /// UpdateConfig makes changes to the contract's configuration
    UpdateConfig {
        /// Address of the DAO governing the contract
        admin: String,
        /// Address of the client used for updating staking info
        maintainer: String,
        /// CW20 staking contract
        staking_contract: String,
        /// Commerce contract CodeID
        commerce_code_id: u64,
        /// Interval between reviews in seconds
        review_interval: u64,
        /// Max amount of tokens taken into consideration for the staking calculation
        max_staked_tokens: Uint128,
        /// Max amount of days staked taken into consideration for the staking calc
        max_staked_days: u64,
        /// Maximum rating score (prevents inflated scores)
        max_rating: u64,
        /// How we calculate the trust score
        /// Should be adjusted based on token allocation/price/TVL
        trust_score_params: TrustScoreParams,
    },

    /// --- PRIVILEGED ---
    /// UpdateStakingInfo re-queries the staking information
    /// for a specific address and updates their trust score.
    UpdateStakingInfo { address: String },
    /// RegisterPendingReview adds a pending review for a user.
    /// Can only be called by a commerce contract (checked by codeID).
    RegisterPendingReview {
        peer: String,
        reviewer: String,
        order_id: u64,
    },

    /// --- USER-FACING ---
    /// Review allows a user to leave a thumbs-up/down review
    /// to another user if there is a pending review.
    Review {
        address: String,
        review: ReviewResult,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// --- ADMINISTRATION ---
    /// Query the contract's config.
    #[returns(ConfigResponse)]
    Config {},

    /// --- USER-FACING ---
    /// Get an address' trust info.
    #[returns(TrustInfoResponse)]
    TrustInfo { address: String },
    /// Get an address' staking info.
    #[returns(StakeAmountResponse)]
    StakeAmount { address: String },
    /// Get all trust accounts.
    #[returns(AccountsResponse)]
    Accounts {},
    /// Get pending review by peer
    #[returns(PendingReviewResponse)]
    PendingReview { peer: String },
    /// Get all pending reviews by reviewer
    #[returns(PendingReviewsResponse)]
    PendingReviewsByReviewer { reviewer: String },
}
