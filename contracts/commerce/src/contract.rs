#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use cw2::set_contract_version;
use cw20::Balance;
use cw20::Cw20CoinVerified;
use cw20::Cw20ReceiveMsg;

use crate::execute::*;
use crate::query::*;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};
use crate::state::config::Config;
use crate::state::config::CONFIG;
use crate::util::map_validate;

use crate::state::admins::{AdminList, ADMIN_LIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:commerce";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin_list = AdminList {
        admins: map_validate(deps.api, &msg.admins)?,
    };

    let denom = deps.api.addr_validate(&msg.denom)?;
    let withdrawal_address = deps.api.addr_validate(&msg.withdrawal_address)?;
    let trust_contract = deps.api.addr_validate(&msg.trust_contract)?;

    let config = Config {
        denom,
        withdrawal_address,
        trust_contract,
    };

    ADMIN_LIST.save(deps.storage, &admin_list)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmins { admins } => execute_update_admins(deps, info, admins),
        ExecuteMsg::UpdateConfig {
            denom,
            withdrawal_address,
            trust_contract,
        } => execute_update_config(deps, info, denom, withdrawal_address, trust_contract),
        ExecuteMsg::UpdateMarketing { marketing } => {
            execute_update_marketing(deps, info, marketing)
        }

        ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, amount),

        ExecuteMsg::CreateListing {
            active,
            price,
            attributes,
            options,
        } => execute_create_listing(deps, info, active, price, attributes, options),
        ExecuteMsg::UpdateListing {
            id,
            active,
            price,
            attributes,
            options,
        } => execute_update_listing(deps, info, id, active, price, attributes, options),
        ExecuteMsg::DeleteListing { id } => execute_delete_listing(deps, info, id),

        ExecuteMsg::UpdateOrder {
            id,
            status,
            tracking,
        } => execute_update_order(deps, info, id, status, tracking),
        ExecuteMsg::CompleteOrder { id } => execute_complete_order(deps, info, id),
        ExecuteMsg::RefundOrder { id } => execute_refund_order(deps, info, id),

        ExecuteMsg::CreateOrder { items } => execute_create_order(deps, info, items, None, None),
        ExecuteMsg::CancelOrder { id } => execute_cancel_order(deps, info, id),

        ExecuteMsg::Receive(msg) => execute_receive(deps, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Marketing {} => to_binary(&query_marketing(deps)?),

        QueryMsg::WithdrawableBalance {} => to_binary(&query_withdrawable_balance(deps, env)?),
        QueryMsg::LockedBalance {} => to_binary(&query_locked_balance(deps)?),

        QueryMsg::AdminList {} => to_binary(&query_admin_list(deps)?),
        QueryMsg::CanExecute { sender } => to_binary(&query_can_execute(deps, sender)?),

        QueryMsg::Listings {} => to_binary(&query_listings(deps)?),
        QueryMsg::Listing { id } => to_binary(&query_listing(deps, id)?),

        QueryMsg::Orders {} => to_binary(&query_orders(deps)?),
        QueryMsg::Order { id } => to_binary(&query_order(deps, id)?),
        QueryMsg::OrderCost { id } => to_binary(&query_order_cost(deps, id)?),
    }
}

pub fn execute_receive(
    deps: DepsMut,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: ReceiveMsg = from_binary(&wrapper.msg)?;

    let balance = Balance::Cw20(Cw20CoinVerified {
        address: info.clone().sender,
        amount: wrapper.amount,
    });

    let api = deps.api;

    match msg {
        ReceiveMsg::CreateOrder { items } => execute_create_order(
            deps,
            info,
            items,
            Some(balance),
            Some(api.addr_validate(&wrapper.sender)?),
        ),
    }
}
