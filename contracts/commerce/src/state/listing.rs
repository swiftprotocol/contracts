use cw20::Cw20Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{StdResult, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, UniqueIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Listing {
    pub id: u64,
    pub active: bool,
    pub price: Cw20Coin,
    pub attributes: Attributes,
    pub options: Vec<ListingOption>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Attributes {
    pub name: String,
    pub description: Option<String>,
    pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListingOption {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub options: Vec<ListingOptionItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ListingOptionItem {
    pub name: String,
    pub cost: Option<Cw20Coin>,
}

impl ListingOption {
    pub fn new(
        id: u64,
        name: &str,
        description: Option<&str>,
        options: Vec<ListingOptionItem>,
    ) -> ListingOption {
        let description = description.map(String::from);

        ListingOption {
            id,
            name: String::from(name),
            description,
            options,
        }
    }
}

impl ListingOptionItem {
    pub fn new(name: &str, cost: Option<Uint128>, denom: String) -> ListingOptionItem {
        let cost = cost.map(|cost| Cw20Coin {
            address: denom,
            amount: cost,
        });

        ListingOptionItem {
            name: String::from(name),
            cost,
        }
    }
}

// Incrementing ID counter
pub const LISTING_ID_COUNTER: Item<u64> = Item::new("listing_id_counter");

// Get next incrementing ID
pub fn next_listing_id(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = LISTING_ID_COUNTER.may_load(store)?.unwrap_or_default() + 1;
    LISTING_ID_COUNTER.save(store, &id)?;

    Ok(id)
}

pub const LISTING_NAMESPACE: &str = "listings";
pub struct ListingIndexes<'a> {
    pub id: UniqueIndex<'a, u64, Listing>,
}

impl<'a> IndexList<Listing> for ListingIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Listing>> + '_> {
        let v: Vec<&dyn Index<Listing>> = vec![&self.id];
        Box::new(v.into_iter())
    }
}

// Function to get all listings
pub fn listings<'a>() -> IndexedMap<'a, u64, Listing, ListingIndexes<'a>> {
    let indexes = ListingIndexes {
        id: UniqueIndex::new(|d| d.id, "listings__id"),
    };
    IndexedMap::new(LISTING_NAMESPACE, indexes)
}
