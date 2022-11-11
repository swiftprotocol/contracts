use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, UniqueIndex};

use super::listing::ListingOptionItem;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Order {
    pub id: u64,
    pub buyer: Addr,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub tracking: Option<TrackingInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderItem {
    pub listing_id: u64,
    pub options: Vec<OrderOption>,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderOption {
    pub option_id: u64,
    pub selected_option: ListingOptionItem,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OrderStatus {
    Received,
    Fulfilling,
    Shipped,
}

impl OrderStatus {
    pub fn index(&self) -> u64 {
        match self {
            OrderStatus::Received => 0,
            OrderStatus::Fulfilling => 1,
            OrderStatus::Shipped => 2,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TrackingInfo {
    pub provider: String,
    pub url: String,
}

// Incrementing ID counter
pub const ORDER_ID_COUNTER: Item<u64> = Item::new("order_id_counter");

// Get next incrementing ID
pub fn next_order_id(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = ORDER_ID_COUNTER.may_load(store)?.unwrap_or_default() + 1;
    ORDER_ID_COUNTER.save(store, &id)?;

    Ok(id)
}

pub const ORDER_NAMESPACE: &str = "orders";
pub struct OrderIndexes<'a> {
    pub id: UniqueIndex<'a, u64, Order>,
}

impl<'a> IndexList<Order> for OrderIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Order>> + '_> {
        let v: Vec<&dyn Index<Order>> = vec![&self.id];
        Box::new(v.into_iter())
    }
}

// Function to get all orders
pub fn orders<'a>() -> IndexedMap<'a, u64, Order, OrderIndexes<'a>> {
    let indexes = OrderIndexes {
        id: UniqueIndex::new(|d| d.id, "orders__id"),
    };
    IndexedMap::new(ORDER_NAMESPACE, indexes)
}
