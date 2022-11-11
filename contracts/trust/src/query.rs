use cosmwasm_std::{to_binary, Order, QueryRequest, StdError, WasmQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};

use cw20_stake::msg::StakedValueResponse;

use crate::{
    response::*,
    state::{pending_reviews, CONFIG, TRUST_INFO},
};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse { config })
}

pub fn query_trust_info(deps: Deps, address: String) -> StdResult<TrustInfoResponse> {
    let address = deps.api.addr_validate(&address)?;
    let trust_info = TRUST_INFO.may_load(deps.storage, &address)?;

    Ok(TrustInfoResponse { trust_info })
}

pub fn query_stake_amount(deps: Deps, address: String) -> StdResult<StakeAmountResponse> {
    let address = deps.api.addr_validate(&address)?;

    let config = CONFIG.load(deps.storage)?;

    let query_msg = cw20_stake::msg::QueryMsg::StakedValue {
        address: address.to_string(),
    };
    let res: Result<StakedValueResponse, StdError> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: config.staking_contract.to_string(),
            msg: to_binary(&query_msg)?,
        }));

    let stake_amount = match res {
        Ok(stake_amount) => Some(stake_amount.value),
        Err(_) => None,
    };

    Ok(StakeAmountResponse { stake_amount })
}

pub fn query_accounts(deps: Deps) -> StdResult<AccountsResponse> {
    let accounts = TRUST_INFO
        .keys(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AccountsResponse { accounts })
}

pub fn query_pending_review(deps: Deps, peer: String) -> StdResult<PendingReviewResponse> {
    let api = deps.api;
    let peer = api.addr_validate(&peer)?;

    let pending_review = pending_reviews().may_load(deps.storage, peer)?;

    Ok(PendingReviewResponse { pending_review })
}

pub fn query_pending_reviews_by_reviewer(
    deps: Deps,
    reviewer: String,
) -> StdResult<PendingReviewsResponse> {
    let api = deps.api;
    let reviewer = api.addr_validate(&reviewer)?;

    let pending_reviews = pending_reviews()
        .idx
        .by_reviewer
        .prefix(reviewer)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(PendingReviewsResponse { pending_reviews })
}
