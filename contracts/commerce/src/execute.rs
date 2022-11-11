use cosmwasm_std::{
    to_binary, Addr, DepsMut, Env, MessageInfo, Response, StdError, Uint128, WasmMsg,
};
use cw20::{Balance, Cw20Coin};
use trust::msg::ExecuteMsg as TrustExecuteMsg;

use crate::query::query_withdrawable_balance;
use crate::state::admins::ADMIN_LIST;
use crate::state::config::{Config, Marketing, CONFIG, MARKETING};
use crate::state::listing::{listings, next_listing_id, Attributes, Listing, ListingOption};
use crate::state::order::{next_order_id, orders, Order, OrderItem, OrderStatus, TrackingInfo};
use crate::util::{
    can_execute, eval_cost, map_validate, must_pay, send_cw20_tokens, validate_empty_orders,
    validate_listing,
};
use crate::ContractError;

pub fn execute_update_admins(
    deps: DepsMut,
    info: MessageInfo,
    admins: Vec<String>,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    }

    let mut cfg = ADMIN_LIST.load(deps.storage)?;
    cfg.admins = map_validate(deps.api, &admins)?;
    ADMIN_LIST.save(deps.storage, &cfg)?;

    Ok(Response::new().add_attribute("action", "update_admins"))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    withdrawal_address: String,
    trust_contract: String,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    }

    let denom = deps.api.addr_validate(&denom)?;
    let withdrawal_address = deps.api.addr_validate(&withdrawal_address)?;
    let trust_contract = deps.api.addr_validate(&trust_contract)?;

    let config = Config {
        denom,
        withdrawal_address,
        trust_contract,
    };

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn execute_update_marketing(
    deps: DepsMut,
    info: MessageInfo,
    marketing: Marketing,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    }

    MARKETING.save(deps.storage, &marketing)?;
    Ok(Response::new().add_attribute("action", "update_marketing"))
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let withdrawable_balance = query_withdrawable_balance(deps.as_ref(), env)?;
    let withdrawal_address = deps.api.addr_validate(config.withdrawal_address.as_str())?;

    let balance = match amount {
        Some(amount) => {
            if amount > withdrawable_balance.balance.amount {
                Err(ContractError::PaymentError(
                    cw_utils::PaymentError::NoFunds {},
                ))
            } else {
                Ok(Cw20Coin {
                    address: config.denom.to_string(),
                    amount,
                })
            }
        }
        None => Ok(withdrawable_balance.balance),
    }?;

    let msg = send_cw20_tokens(&withdrawal_address, &balance)?;

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("amount", balance.amount)
        .add_attribute("to", withdrawal_address)
        .add_submessage(msg))
}

pub fn execute_create_listing(
    deps: DepsMut,
    info: MessageInfo,
    active: bool,
    price: Uint128,
    attributes: Attributes,
    options: Vec<ListingOption>,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    }

    let config = CONFIG.load(deps.storage)?;

    // let contract_info = deps
    //     .querier
    //     .query_wasm_contract_info(config.denom.clone())?;

    // Validate listing data
    validate_listing(deps.as_ref(), attributes.clone(), options.clone())?;

    let listing = Listing {
        id: next_listing_id(deps.storage)?,
        active,
        price: Cw20Coin {
            address: config.denom.to_string(),
            amount: price,
        },
        attributes,
        options,
    };

    listings().save(deps.storage, listing.id, &listing)?;

    Ok(Response::new()
        .add_attribute("action", "create_listing")
        .add_attribute("listing_id", listing.id.to_string()))
}

pub fn execute_update_listing(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    active: bool,
    price: Uint128,
    attributes: Attributes,
    options: Vec<ListingOption>,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    }

    let config = CONFIG.load(deps.storage)?;

    // Validate that the listing does not have any active orders
    validate_empty_orders(deps.as_ref(), id)?;

    // Validate listing data
    validate_listing(deps.as_ref(), attributes.clone(), options.clone())?;

    let listing = listings().update(deps.storage, id, |listing| match listing {
        Some(listing) => {
            let new_listing = Listing {
                id: listing.id,
                active,
                price: Cw20Coin {
                    address: config.denom.to_string(),
                    amount: price,
                },
                attributes,
                options,
            };

            Ok(new_listing)
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("listing"),
        })),
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_listing")
        .add_attribute("listing_id", listing.id.to_string()))
}

pub fn execute_delete_listing(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    }

    // Validate that the listing does not have any active orders
    validate_empty_orders(deps.as_ref(), id)?;

    // Remove the listing
    listings().remove(deps.storage, id)?;

    Ok(Response::new().add_attribute("action", "delete_listing"))
}

pub fn execute_update_order(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    status: OrderStatus,
    tracking: Option<TrackingInfo>,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    };

    let order = orders().update(deps.storage, id, |order| match order {
        Some(order) => {
            if status.index() < order.status.index() {
                return Err(ContractError::CustomError {
                    val: String::from("New status cannot be a previous status"),
                });
            };

            let new_order = Order {
                id: order.id,
                buyer: order.buyer,
                items: order.items,
                status,
                tracking,
            };

            Ok(new_order)
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("order"),
        })),
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_order")
        .add_attribute("order_id", order.id.to_string()))
}

pub fn execute_complete_order(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    };

    let config = CONFIG.load(deps.storage)?;
    let admin_list = ADMIN_LIST.load(deps.storage)?;

    let order = orders().may_load(deps.storage, id)?;

    match order {
        Some(order) => {
            // Cannot complete an order if it has not shipped
            if order.status != OrderStatus::Shipped {
                return Err(ContractError::Unauthorized {});
            }

            orders().remove(deps.storage, order.id)?;

            // Buyer leaves review to every seller
            let msgs: Vec<TrustExecuteMsg> = admin_list
                .admins
                .clone()
                .into_iter()
                .map(|admin| TrustExecuteMsg::RegisterPendingReview {
                    peer: admin.to_string(),
                    reviewer: order.buyer.to_string(),
                    order_id: order.id,
                })
                .collect();

            let buyer_messages = msgs.into_iter().map(|msg| {
                let msg = to_binary(&msg).unwrap();
                WasmMsg::Execute {
                    contract_addr: config.trust_contract.to_string(),
                    msg,
                    funds: vec![],
                }
            });

            // Main seller leaves review to buyer
            let msg: TrustExecuteMsg = TrustExecuteMsg::RegisterPendingReview {
                peer: order.buyer.to_string(),
                reviewer: admin_list.admins.first().unwrap().to_string(),
                order_id: order.id,
            };

            let seller_message = WasmMsg::Execute {
                contract_addr: config.trust_contract.to_string(),
                msg: to_binary(&msg).unwrap(),
                funds: vec![],
            };

            Ok(Response::new()
                .add_attribute("action", "complete_order")
                .add_messages(buyer_messages)
                .add_message(seller_message))
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("order"),
        })),
    }
}

pub fn execute_refund_order(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    };

    let config = CONFIG.load(deps.storage)?;

    let order = orders().may_load(deps.storage, id)?;

    match order {
        Some(order) => {
            let cost = eval_cost(deps.as_ref(), order.items)?;
            orders().remove(deps.storage, order.id)?;

            let msg = send_cw20_tokens(
                &order.buyer,
                &Cw20Coin {
                    address: config.denom.to_string(),
                    amount: cost,
                },
            )?;

            Ok(Response::new()
                .add_attribute("action", "refund_order")
                .add_submessage(msg))
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("order"),
        })),
    }
}

pub fn execute_create_order(
    deps: DepsMut,
    info: MessageInfo,
    items: Vec<OrderItem>,
    balance: Option<Balance>,
    sender: Option<Addr>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let balance = match balance {
        Some(balance) => balance,
        None => Balance::from(info.funds),
    };

    let sender = match sender {
        Some(sender) => sender,
        None => info.sender,
    };

    let cw20_addr = deps.api.addr_validate(config.denom.as_str())?;
    let amount_paid = must_pay(cw20_addr, balance)?;
    let cost = eval_cost(deps.as_ref(), items.clone())?;

    if amount_paid != cost {
        return Err(ContractError::PaymentError(
            cw_utils::PaymentError::NoFunds {},
        ));
    };

    let order = Order {
        id: next_order_id(deps.storage)?,
        buyer: sender,
        items,
        status: OrderStatus::Received,
        tracking: None,
    };

    orders().save(deps.storage, order.id, &order)?;

    Ok(Response::new()
        .add_attribute("action", "create_order")
        .add_attribute("order_id", order.id.to_string()))
}

pub fn execute_cancel_order(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
) -> Result<Response, ContractError> {
    let order = orders().may_load(deps.storage, id)?;

    match order {
        Some(order) => {
            if info.sender != order.buyer {
                return Err(ContractError::Unauthorized {});
            }

            // Cannot cancel an order if it is already being fulfilled
            if order.status != OrderStatus::Received {
                return Err(ContractError::Unauthorized {});
            }

            let config = CONFIG.load(deps.storage)?;

            let cost = eval_cost(deps.as_ref(), order.items)?;
            orders().remove(deps.storage, order.id)?;

            let msg = send_cw20_tokens(
                &order.buyer,
                &Cw20Coin {
                    address: config.denom.to_string(),
                    amount: cost,
                },
            )?;

            Ok(Response::new()
                .add_attribute("action", "cancel_order")
                .add_submessage(msg))
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("order"),
        })),
    }
}
