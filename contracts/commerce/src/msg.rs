use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;

use crate::{
    response::*,
    state::{
        config::Marketing,
        listing::{Attributes, ListingOption},
        order::{OrderItem, OrderStatus, TrackingInfo},
    },
};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub denom: String,
    pub withdrawal_address: String,
    pub trust_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// --- ADMINISTRATION ---
    /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin.
    UpdateAdmins {
        admins: Vec<String>,
    },
    /// UpdateConfig will make changes to the site's configuration.
    UpdateConfig {
        /// Address of the CW20 contract to be used as a denom for this site
        denom: String,
        /// Address to which funds held by the contract can be withdrawn
        withdrawal_address: String,
        /// Address of the trust contact
        trust_contract: String,
    },
    /// UpdateMarketing will make changes to the social/marketing aspects of the site.
    UpdateMarketing {
        marketing: Marketing,
    },
    /// Withdraw all or a specific amount of currently available funds.
    Withdraw {
        amount: Option<Uint128>,
    },

    /// --- SELLER-FACING ---
    /// CreateListing will create a new item to be displayed on the site.
    CreateListing {
        active: bool,
        price: Uint128,
        attributes: Attributes,
        options: Vec<ListingOption>,
    },
    /// UpdateListing will make modifications to an existing listing.
    UpdateListing {
        id: u64,
        active: bool,
        price: Uint128,
        attributes: Attributes,
        options: Vec<ListingOption>,
    },
    /// DeleteListing will remove an existing listing from the site.
    DeleteListing {
        id: u64,
    },
    /// UpdateOrder will update the status and the tracking info of an order.
    UpdateOrder {
        id: u64,
        status: OrderStatus,
        tracking: Option<TrackingInfo>,
    },
    /// CompleteOrder will remove an order and mark it as completed.
    CompleteOrder {
        id: u64,
    },
    /// RefundOrder will completely cancel an order and refund the buyer.
    RefundOrder {
        id: u64,
    },

    /// --- BUYER-FACING ---
    /// CreateOrder will create a new order for one or more items on the site.
    CreateOrder {
        items: Vec<OrderItem>,
    },
    /// CancelOrder will cancel an order & refund the buyer.
    /// Can only be called when the `Received` status is active.
    /// An order cannot be refunded once it is being fulfilled.
    CancelOrder {
        id: u64,
    },

    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
pub enum ReceiveMsg {
    CreateOrder { items: Vec<OrderItem> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// --- ADMINISTRATION ---
    /// Returns the site's configuration
    #[returns(ConfigResponse)]
    Config {},
    /// Returns a list of administrators
    #[returns(AdminListResponse)]
    AdminList {},
    /// Checks permissions of the caller on this contract
    #[returns(cw1::CanExecuteResponse)]
    CanExecute { sender: String },
    /// Returns the amount of tokens that can be withdrawn from the contract
    #[returns(BalanceResponse)]
    WithdrawableBalance {},
    /// Returns the amount of tokens that are locked up in active orderes
    #[returns(BalanceResponse)]
    LockedBalance {},

    /// --- SITE INFO ---
    /// Returns the site's marketing info
    /// Marketing defines the style of the site
    /// It is fetched every time the site is opened,
    /// eliminating the need for the frontend
    /// to be redeployed to make changes to the styling.
    /// This includes featured items, the logo, etc...
    #[returns(MarketingResponse)]
    Marketing {},

    /// --- USER-FACING ---
    /// Get single or all orders
    #[returns(OrdersResponse)]
    Orders {},
    #[returns(OrderResponse)]
    Order { id: u64 },
    #[returns(OrderCostResponse)]
    OrderCost { id: u64 },
    /// Get single or all listings
    #[returns(ListingsResponse)]
    Listings {},
    #[returns(ListingResponse)]
    Listing { id: u64 },
}

#[cfg(any(test, feature = "test-utils"))]
impl AdminListResponse {
    /// Utility function for converting message to its canonical form, so two messages with
    /// different representation but same semantical meaning can be easly compared.
    pub fn canonical(mut self) -> Self {
        self.admins.sort();
        self.admins.dedup();
        self
    }
}
