use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use cw_storage_plus::Item;

use super::listing::Listing;

#[cw_serde]
pub struct Config {
    /// Address of the CW20 contract to be used as a denom for this site
    pub denom: Addr,

    /// Address to which funds held by the contract can be withdrawn
    pub withdrawal_address: Addr,

    /// Address of the trust contact
    pub trust_contract: Addr,
}

#[cw_serde]
pub struct Marketing {
    /// Name of the site
    pub name: String,

    /// Copyright info for the site (optional)
    /// Format: "Josef's Store 2022"
    /// The frontend will insert the © symbol
    pub copyright: Option<String>,

    /// URL to hosted logo for the site (optional)
    /// The frontend will fall back to a text logo based on `name`
    pub logo: Option<String>,

    /// Array of featured listings (optional)
    /// These will appear at the top of the site
    pub featured_listings: Vec<Listing>,

    /// Array of socials (optional)
    /// See `Network` enum below for supported socials
    pub socials: Vec<Social>,
}

#[cw_serde]
pub struct Social {
    pub network: Network,
    pub url: String,
}

#[cw_serde]
pub enum Network {
    Twitter,
    Facebook,
    GitHub,
    LinkedIn,
    Instagram,
    YouTube,
    Reddit,
    Medium,
    Discord,
    TikTok,
    Twitch,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const MARKETING: Item<Marketing> = Item::new("marketing");
