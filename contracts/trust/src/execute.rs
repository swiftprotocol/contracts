use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, Uint128};

use crate::{
    helpers::calculate_trust_score,
    query::query_stake_amount,
    state::{
        pending_reviews, Config, PendingReview, ReviewResult, TrustData, TrustInfo,
        TrustScoreParams, CONFIG, TRUST_INFO,
    },
    ContractError,
};

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: String,
    maintainer: String,
    staking_contract: String,
    commerce_code_id: u64,
    review_interval: u64,
    max_staked_tokens: Uint128,
    max_staked_days: u64,
    max_rating: u64,
    trust_score_params: TrustScoreParams,
) -> Result<Response, ContractError> {
    let api = deps.api;
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let admin = api.addr_validate(&admin)?;
    let maintainer = api.addr_validate(&maintainer)?;
    let staking_contract = api.addr_validate(&staking_contract)?;

    let config = Config {
        admin,
        maintainer,
        staking_contract,
        commerce_code_id,
        review_interval,
        max_staked_tokens,
        max_staked_days,
        max_rating,
        trust_score_params,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("sender", info.sender)
        .add_attribute("admin", config.admin)
        .add_attribute("maintainer", config.maintainer)
        .add_attribute("staking_contract", config.staking_contract)
        .add_attribute("commerce_code_id", config.commerce_code_id.to_string())
        .add_attribute("review_interval", config.review_interval.to_string())
        .add_attribute("max_staked_tokens", config.max_staked_tokens.to_string())
        .add_attribute("max_staked_days", config.max_staked_days.to_string())
        .add_attribute("max_rating", config.max_rating.to_string()))
}

pub fn execute_update_staking_info(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let api = deps.api;
    let staker = api.addr_validate(&address)?;

    let config = CONFIG.load(deps.storage)?;

    // Only the maintainer can execute this message
    if info.sender != config.maintainer {
        return Err(ContractError::Unauthorized {});
    }

    let res = query_stake_amount(deps.as_ref(), staker.to_string())?;

    match res.stake_amount {
        Some(stake_amount) => {
            if stake_amount.u128() == 0 {
                return Err(ContractError::Std(StdError::NotFound {
                    kind: String::from("staking_account"),
                }));
            };

            TRUST_INFO.update(deps.storage, &staker, |info| match info {
                Some(info) => {
                    let stake_days;

                    if stake_amount < info.data.prev_stake_amount {
                        stake_days = 0;
                    } else if stake_amount >= info.data.prev_stake_amount {
                        stake_days = info.data.stake_days + 1
                    } else {
                        stake_days = 0;
                    }

                    let data = TrustData {
                        stake_days,
                        stake_amount,
                        prev_stake_amount: info.data.stake_amount,
                        rating: info.data.rating,
                    };

                    let score = calculate_trust_score(data.clone(), config.clone());

                    Ok(TrustInfo { score, data })
                }
                None => Err(StdError::NotFound {
                    kind: String::from("trust_account"),
                }),
            })?;

            let trust_info = TRUST_INFO.load(deps.storage, &info.sender)?;

            Ok(Response::new()
                .add_attribute("action", "update_staking_info")
                .add_attribute("delegator", info.sender.to_string())
                .add_attribute("stake_days", trust_info.data.stake_days.to_string())
                .add_attribute("stake_amount", trust_info.data.stake_amount.to_string())
                .add_attribute(
                    "prev_stake_amount",
                    trust_info.data.prev_stake_amount.to_string(),
                ))
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("staking_account"),
        })),
    }
}

pub fn execute_register_pending_review(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    peer: String,
    reviewer: String,
    order_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let api = deps.api;
    let peer = api.addr_validate(&peer)?;
    let reviewer = api.addr_validate(&reviewer)?;

    // Verify that the sender is a commerce contract
    let contract_info = deps.querier.query_wasm_contract_info(&info.sender)?;
    if contract_info.code_id != config.commerce_code_id {
        return Err(ContractError::InvalidContract {
            expected_code_id: config.commerce_code_id.to_string(),
        });
    }

    let pending_review = pending_reviews().may_load(deps.storage, peer.clone())?;

    // if there is a review that is expired, remove it
    match pending_review {
        Some(pending_review) => {
            if env.block.time < pending_review.expires_at {
                return Err(ContractError::AwaitingReview {
                    reviewer: reviewer.to_string(),
                });
            }

            pending_reviews().remove(deps.storage, peer.clone())?;
        }
        None => {}
    }

    let expires_at = env.block.time.plus_seconds(config.review_interval);

    let pending_review = PendingReview {
        peer: peer.clone(),
        reviewer: reviewer.clone(),
        commerce_contract: info.sender.clone(),
        order_id,
        expires_at: expires_at.clone(),
    };

    // Save a new pending review
    pending_reviews().save(deps.storage, peer.clone(), &pending_review)?;

    Ok(Response::new()
        .add_attribute("action", "register_pending_review")
        .add_attribute("peer", peer.to_string())
        .add_attribute("reviewer", reviewer.to_string())
        .add_attribute("commerce_contract", info.sender.to_string())
        .add_attribute("order_id", order_id.to_string())
        .add_attribute("expires_at", expires_at.to_string()))
}

pub fn execute_review(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    address: String,
    review: ReviewResult,
) -> Result<Response, ContractError> {
    let api = deps.api;
    let peer = api.addr_validate(&address)?;

    let config = CONFIG.load(deps.storage)?;

    let pending_review = pending_reviews().load(deps.storage, peer.clone())?;

    // If there is an expired review, remove it
    if env.block.time >= pending_review.expires_at {
        pending_reviews().remove(deps.storage, peer.clone())?;
    }

    // Now that we've possibly removed an expired review, let's revalidate!
    pending_reviews().load(deps.storage, peer.clone())?;

    // If there isn't a trust account for the peer, create one
    // If there is one, update the data
    let trust_info = TRUST_INFO.load(deps.storage, &peer);
    match trust_info {
        Ok(trust_info) => {
            let trust_data = TrustData {
                stake_days: trust_info.data.stake_days,
                stake_amount: trust_info.data.stake_amount,
                prev_stake_amount: trust_info.data.prev_stake_amount,
                rating: trust_info.data.rating
                    + match review {
                        ReviewResult::ThumbsUp => 1,
                        ReviewResult::ThumbsDown => -1,
                    },
            };

            TRUST_INFO.update(deps.storage, &peer, |info| match info {
                Some(_) => {
                    // Update trust score
                    let score = calculate_trust_score(trust_data.clone(), config);

                    Ok(TrustInfo {
                        score,
                        data: trust_data,
                    })
                }
                None => Err(StdError::NotFound {
                    kind: String::from("trust_account"),
                }),
            })?;
        }
        Err(err) => {
            if err
                == (StdError::NotFound {
                    kind: String::from("trust::state::TrustInfo"),
                })
            {
                let trust_data = TrustData {
                    stake_days: 0,
                    stake_amount: Uint128::from(0u128),
                    prev_stake_amount: Uint128::from(0u128),
                    rating: 0 + match review {
                        ReviewResult::ThumbsUp => 1,
                        ReviewResult::ThumbsDown => -1,
                    },
                };

                let score = calculate_trust_score(trust_data.clone(), config);

                TRUST_INFO.save(
                    deps.storage,
                    &peer,
                    &TrustInfo {
                        score,
                        data: trust_data,
                    },
                )?;
            }
        }
    }

    // Let's remove that pending review now that a review has been submitted
    pending_reviews().remove(deps.storage, peer.clone())?;

    // Query trust score for response
    let trust_info = TRUST_INFO.load(deps.storage, &peer)?;

    Ok(Response::new()
        .add_attribute("action", "review")
        .add_attribute("peer", peer.to_string())
        .add_attribute("reviewer", info.sender.to_string())
        .add_attribute(
            "review",
            (match review {
                ReviewResult::ThumbsUp => 1,
                ReviewResult::ThumbsDown => -1,
            })
            .to_string(),
        )
        .add_attribute("new_score", trust_info.score.to_string()))
}
