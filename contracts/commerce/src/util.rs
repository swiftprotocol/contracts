use cosmwasm_std::{to_binary, Addr, Api, Deps, StdError, StdResult, SubMsg, Uint128, WasmMsg};
use cw20::{Balance, Cw20Coin, Cw20ExecuteMsg};
use cw_utils::PaymentError;

use crate::{
    state::{
        admins::ADMIN_LIST,
        config::CONFIG,
        listing::{listings, Attributes, ListingOption},
        order::{orders, OrderItem},
    },
    ContractError,
};

/// Verify that an address is authorized to execute a privileged operation
pub fn can_execute(deps: Deps, sender: &str) -> StdResult<bool> {
    let cfg = ADMIN_LIST.load(deps.storage)?;
    let can = cfg.is_admin(sender);
    Ok(can)
}

/// Validate that an array of addresses is composed of only valid addresses
pub fn map_validate(api: &dyn Api, admins: &[String]) -> StdResult<Vec<Addr>> {
    admins.iter().map(|addr| api.addr_validate(addr)).collect()
}

/// Returns the amount if matches CW20 address and non-zero amount. Errors otherwise.
pub fn must_pay(denom: Addr, balance: Balance) -> Result<Uint128, PaymentError> {
    match balance {
        Balance::Cw20(token) => {
            if token.address == denom {
                Ok(token.amount)
            } else {
                Err(PaymentError::MissingDenom(String::from(denom)))
            }
        }
        Balance::Native(_) => Err(PaymentError::MissingDenom(String::from(denom))),
    }
}

// Verify the validity of listing data
pub fn validate_listing(
    deps: Deps,
    attributes: Attributes,
    options: Vec<ListingOption>,
) -> Result<(), ContractError> {
    // Need at least 1 image to create a listing
    if attributes.images.is_empty() {
        return Err(ContractError::NotEnoughImages {});
    }

    let config = CONFIG.load(deps.storage)?;

    // Verify that the Cw20 token address in each option's cost is correct
    for option in options {
        for option_item in option.options {
            if let Some(cost) = option_item.cost {
                if cost.address != config.denom {
                    return Err(ContractError::PaymentError(
                        cw_utils::PaymentError::MissingDenom(config.denom.to_string()),
                    ));
                }
            }
        }
    }

    Ok(())
}

// Verify that a listing has no active orders
pub fn validate_empty_orders(deps: Deps, listing_id: u64) -> Result<(), ContractError> {
    let listing = listings().may_load(deps.storage, listing_id)?;

    match listing {
        Some(listing) => {
            let orders = orders()
                .idx
                .id
                .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .map(|res| res.map(|item| item.1))
                .collect::<StdResult<Vec<_>>>()?;

            for order in orders {
                for item in order.items {
                    if item.listing_id == listing.id {
                        return Err(ContractError::ActiveOrder {});
                    }
                }
            }

            Ok(())
        }
        None => Err(ContractError::Std(StdError::NotFound {
            kind: String::from("listing"),
        })),
    }
}

// Evaluates the total cost of an order
pub fn eval_cost(deps: Deps, items: Vec<OrderItem>) -> Result<Uint128, ContractError> {
    let mut total_cost: u128 = 0;

    for item in items {
        let listing = listings().may_load(deps.storage, item.listing_id)?;
        match listing {
            Some(listing) => {
                let mut total_item_cost: u128 = 0;
                // Add the price of the listing to the total item cost
                total_item_cost += listing.price.amount.u128();

                // For every option, first verify that it is valid
                // then add its cost to the total if it exists
                for option in item.options {
                    if !listing
                        .options
                        .iter()
                        .any(|listing_option| listing_option.id == option.option_id)
                    {
                        return Err(ContractError::InvalidOrder {});
                    }

                    if let Some(cost) = option.selected_option.cost {
                        total_item_cost += cost.amount.u128()
                    }
                }

                // Finally, add this item's total cost multiplied by its quantity
                // to the total cost of the order
                total_cost += total_item_cost * (item.amount as u128)
            }
            None => return Err(ContractError::InvalidOrder {}),
        }
    }

    Ok(Uint128::from(total_cost))
}

// Send Cw20 tokens to another address
pub fn send_cw20_tokens(to: &Addr, balance: &Cw20Coin) -> StdResult<SubMsg> {
    let msg = Cw20ExecuteMsg::Transfer {
        recipient: to.into(),
        amount: balance.amount,
    };
    let exec = SubMsg::new(WasmMsg::Execute {
        contract_addr: balance.address.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    });

    Ok(exec)
}
