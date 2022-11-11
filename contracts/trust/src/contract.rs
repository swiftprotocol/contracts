#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw2::set_contract_version;

use crate::execute::*;
use crate::query::*;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:trust";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = info.sender;
    let staking_contract = deps.api.addr_validate(&msg.staking_contract)?;
    let maintainer = deps.api.addr_validate(&msg.maintainer)?;

    let config = Config {
        admin,
        maintainer,
        staking_contract,
        commerce_code_id: msg.commerce_code_id,
        review_interval: msg.review_interval,
        max_staked_tokens: msg.max_staked_tokens,
        max_staked_days: msg.max_staked_days,
        max_rating: msg.max_rating,
        trust_score_params: msg.trust_score_params,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_addr", env.contract.address.to_string())
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("admin", config.admin)
        .add_attribute("maintainer", config.maintainer)
        .add_attribute("staking_contract", config.staking_contract)
        .add_attribute("commerce_code_id", config.commerce_code_id.to_string())
        .add_attribute("review_interval", config.review_interval.to_string())
        .add_attribute("max_staked_tokens", config.max_staked_tokens.to_string())
        .add_attribute("max_staked_days", config.max_staked_days.to_string())
        .add_attribute("max_rating", config.max_rating.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateStakingInfo { address } => {
            execute_update_staking_info(deps, info, address)
        }
        ExecuteMsg::UpdateConfig {
            admin,
            maintainer,
            staking_contract,
            commerce_code_id,
            review_interval,
            max_staked_tokens,
            max_staked_days,
            max_rating,
            trust_score_params,
        } => execute_update_config(
            deps,
            info,
            admin,
            maintainer,
            staking_contract,
            commerce_code_id,
            review_interval,
            max_staked_tokens,
            max_staked_days,
            max_rating,
            trust_score_params,
        ),
        ExecuteMsg::RegisterPendingReview {
            peer,
            reviewer,
            order_id,
        } => execute_register_pending_review(deps, info, env, peer, reviewer, order_id),
        ExecuteMsg::Review { address, review } => execute_review(deps, info, env, address, review),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::StakeAmount { address } => to_binary(&query_stake_amount(deps, address)?),
        QueryMsg::TrustInfo { address } => to_binary(&query_trust_info(deps, address)?),
        QueryMsg::Accounts {} => to_binary(&query_accounts(deps)?),
        QueryMsg::PendingReview { peer } => to_binary(&query_pending_review(deps, peer)?),
        QueryMsg::PendingReviewsByReviewer { reviewer } => {
            to_binary(&query_pending_reviews_by_reviewer(deps, reviewer)?)
        }
    }
}
