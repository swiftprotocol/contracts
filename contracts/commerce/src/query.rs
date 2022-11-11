#[cfg(not(feature = "library"))]
use cosmwasm_std::{Deps, StdResult};
use cosmwasm_std::{Env, Order, Uint128};

use cw1::CanExecuteResponse;
use cw20::{Balance, Cw20Coin};

use crate::response::*;
use crate::state::config::{CONFIG, MARKETING};
use crate::util::{can_execute, eval_cost};

use crate::state::admins::ADMIN_LIST;
use crate::state::listing::listings;
use crate::state::order::orders;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

pub fn query_marketing(deps: Deps) -> StdResult<MarketingResponse> {
    let marketing = MARKETING.load(deps.storage)?;
    Ok(MarketingResponse { marketing })
}

pub fn query_admin_list(deps: Deps) -> StdResult<AdminListResponse> {
    let cfg = ADMIN_LIST.load(deps.storage)?;
    Ok(AdminListResponse {
        admins: cfg.admins.into_iter().map(|a| a.into()).collect(),
    })
}

pub fn query_can_execute(deps: Deps, sender: String) -> StdResult<CanExecuteResponse> {
    Ok(CanExecuteResponse {
        can_execute: can_execute(deps, &sender)?,
    })
}

pub fn query_withdrawable_balance(deps: Deps, env: Env) -> StdResult<BalanceResponse> {
    let config = CONFIG.load(deps.storage)?;

    let orders = query_orders(deps)?;
    let mut total_cost: u128 = 0;
    for order in orders.orders {
        let cost = eval_cost(deps, order.items);
        if let Ok(cost) = cost {
            total_cost += cost.u128();
        }
    }

    let balances = deps.querier.query_all_balances(env.contract.address)?;
    let balance = Balance::from(balances);

    let held_balance: Uint128 = match balance {
        Balance::Cw20(token) => {
            if token.address == config.denom {
                token.amount
            } else {
                Uint128::zero()
            }
        }
        Balance::Native(_) => Uint128::zero(),
    };

    let withdrawable_balance = Cw20Coin {
        address: config.denom.to_string(),
        amount: held_balance - Uint128::from(total_cost),
    };

    Ok(BalanceResponse {
        balance: withdrawable_balance,
    })
}

pub fn query_locked_balance(deps: Deps) -> StdResult<BalanceResponse> {
    let config = CONFIG.load(deps.storage)?;

    let orders = query_orders(deps)?;
    let mut total_cost: u128 = 0;
    for order in orders.orders {
        let cost = eval_cost(deps, order.items);
        if let Ok(cost) = cost {
            total_cost += cost.u128();
        }
    }

    let locked_balance = Cw20Coin {
        address: config.denom.to_string(),
        amount: Uint128::from(total_cost),
    };

    Ok(BalanceResponse {
        balance: locked_balance,
    })
}

pub fn query_listings(deps: Deps) -> StdResult<ListingsResponse> {
    let listings = listings()
        .idx
        .id
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(ListingsResponse { listings })
}

pub fn query_listing(deps: Deps, id: u64) -> StdResult<ListingResponse> {
    let listing = listings().may_load(deps.storage, id)?;

    Ok(ListingResponse { listing })
}

pub fn query_orders(deps: Deps) -> StdResult<OrdersResponse> {
    let orders = orders()
        .idx
        .id
        .range(deps.storage, None, None, Order::Ascending)
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(OrdersResponse { orders })
}

pub fn query_order(deps: Deps, id: u64) -> StdResult<OrderResponse> {
    let order = orders().may_load(deps.storage, id)?;

    Ok(OrderResponse { order })
}
