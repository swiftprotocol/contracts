use cosmwasm_schema::cw_serde;
use cw20::Cw20Coin;

use crate::state::config::{Config, Marketing};
use crate::state::listing::Listing;
use crate::state::order::Order;

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct MarketingResponse {
    pub marketing: Marketing,
}

#[cw_serde]
pub struct AdminListResponse {
    pub admins: Vec<String>,
}

#[cw_serde]
pub struct ListingsResponse {
    pub listings: Vec<Listing>,
}

#[cw_serde]
pub struct ListingResponse {
    pub listing: Option<Listing>,
}

#[cw_serde]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
}

#[cw_serde]
pub struct OrderResponse {
    pub order: Option<Order>,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Cw20Coin,
}
