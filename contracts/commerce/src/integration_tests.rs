#![cfg(test)]

use cosmwasm_std::{coins, to_binary, Addr, Empty, Uint128};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use trust::state::{ReviewResult, TrustScoreParams};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, ReceiveMsg},
    state::{
        listing::{Attributes, ListingOption, ListingOptionItem},
        order::{OrderItem, OrderOption, OrderStatus, TrackingInfo},
    },
};

pub fn contract_commerce() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

pub fn contract_cw20_stake() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_stake::contract::execute,
        cw20_stake::contract::instantiate,
        cw20_stake::contract::query,
    );
    Box::new(contract)
}

pub fn contract_trust() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        trust::contract::execute,
        trust::contract::instantiate,
        trust::contract::query,
    );
    Box::new(contract)
}

const COMMERCE: &str = "contract3";
const CW20: &str = "contract0";
const CW20_STAKE: &str = "contract1";
const TRUST: &str = "contract2";

const ADMIN: &str = "admin";
const BUYER: &str = "buyer";

// Initial contract setup
fn setup_contract() -> App {
    let admin = Addr::unchecked(ADMIN);
    let buyer = Addr::unchecked(BUYER);

    let init_funds = coins(2000, "ujuno");

    let mut router = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &admin, init_funds)
            .unwrap();
    });

    // set up cw20 contract with some tokens
    let cw20_id = router.store_code(contract_cw20());
    let msg = cw20_base::msg::InstantiateMsg {
        name: String::from("Swift Protocol"),
        symbol: String::from("SWIFT"),
        decimals: 6,
        initial_balances: vec![
            Cw20Coin {
                address: admin.to_string(),
                amount: Uint128::new(5000),
            },
            Cw20Coin {
                address: buyer.to_string(),
                amount: Uint128::new(5000),
            },
        ],
        mint: None,
        marketing: None,
    };
    let cw20_addr = router
        .instantiate_contract(cw20_id, admin.clone(), &msg, &[], "SWIFT", None)
        .unwrap();

    // set up cw20 staking contract
    let cw20_stake_id = router.store_code(contract_cw20_stake());
    let msg = cw20_stake::msg::InstantiateMsg {
        token_address: cw20_addr.to_string(),
        owner: None,
        manager: None,
        unstaking_duration: None,
    };
    let cw20_stake_addr = router
        .instantiate_contract(cw20_stake_id, admin.clone(), &msg, &[], "SWIFT_STAKE", None)
        .unwrap();

    // set up commerce codeid
    let commerce_id = router.store_code(contract_commerce());

    // set up trust contract
    let trust_id = router.store_code(contract_trust());
    let msg = trust::msg::InstantiateMsg {
        maintainer: admin.to_string(),
        staking_contract: cw20_stake_addr.to_string(),
        commerce_code_id: commerce_id,
        review_interval: 86400u64,
        max_staked_days: 240,
        max_staked_tokens: Uint128::new(5000),
        max_rating: 50,
        trust_score_params: TrustScoreParams {
            base_score: 500,
            denom_multiplier: 1,
            rating_multiplier: 25,
            stake_amount_denominator: 10,
            min_stake_days: 1,
            rating_floor_denominator: 10,
        },
    };
    let trust_addr = router
        .instantiate_contract(trust_id, admin.clone(), &msg, &[], "TRUST", None)
        .unwrap();

    // set up commerce contract
    router
        .instantiate_contract(
            commerce_id,
            admin.clone(),
            &InstantiateMsg {
                admins: vec![admin.to_string()],
                denom: cw20_addr.to_string(),
                withdrawal_address: admin.to_string(),
                trust_contract: trust_addr.to_string(),
            },
            &[],
            "COMMERCE",
            None,
        )
        .unwrap();

    router
}

#[test]
fn proper_initialization() {
    setup_contract();
}

#[test]
fn try_create_listing() {
    let mut router = setup_contract();

    let authorized = Addr::unchecked(ADMIN);
    let unauthorized = Addr::unchecked(BUYER);

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        authorized,
        Addr::unchecked(COMMERCE),
        &create_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    // This user isn't!
    let err = router.execute_contract(
        unauthorized,
        Addr::unchecked(COMMERCE),
        &create_listing_msg,
        &[],
    );
    assert!(err.is_err());
}

#[test]
fn try_update_listing() {
    let mut router = setup_contract();

    let authorized = Addr::unchecked(ADMIN);
    let unauthorized = Addr::unchecked(BUYER);

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        authorized.clone(),
        Addr::unchecked(COMMERCE),
        &create_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    // Change active to `false`
    let update_listing_msg = ExecuteMsg::UpdateListing {
        id: 1,
        active: false,
        price: Uint128::from(100u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        authorized,
        Addr::unchecked(COMMERCE),
        &update_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    // This user isn't!
    let err = router.execute_contract(
        unauthorized,
        Addr::unchecked(COMMERCE),
        &update_listing_msg,
        &[],
    );
    assert!(err.is_err());
}

#[test]
fn try_delete_listing() {
    let mut router = setup_contract();

    let authorized = Addr::unchecked(ADMIN);
    let unauthorized = Addr::unchecked(BUYER);

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        authorized.clone(),
        Addr::unchecked(COMMERCE),
        &create_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    let delete_listing_msg = ExecuteMsg::DeleteListing { id: 1 };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        authorized,
        Addr::unchecked(COMMERCE),
        &delete_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    // This user isn't!
    let err = router.execute_contract(
        unauthorized,
        Addr::unchecked(COMMERCE),
        &delete_listing_msg,
        &[],
    );
    assert!(err.is_err());
}

#[test]
fn try_create_order() {
    let mut router = setup_contract();

    let seller = Addr::unchecked(ADMIN);
    let buyer = Addr::unchecked(BUYER);

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(seller, Addr::unchecked(COMMERCE), &create_listing_msg, &[]);
    assert!(res.is_ok());

    // Cost should be 2000
    let create_order_msg = ReceiveMsg::CreateOrder {
        items: vec![OrderItem {
            listing_id: 1,
            options: vec![OrderOption {
                option_id: 1,
                selected_option: ListingOptionItem::new(
                    "M",
                    None,
                    Addr::unchecked(CW20).to_string(),
                ),
            }],
            amount: 2,
        }],
    };

    let send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(COMMERCE).to_string(),
        amount: Uint128::new(2000),
        msg: to_binary(&create_order_msg).unwrap(),
    };

    let incorrect_send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(COMMERCE).to_string(),
        amount: Uint128::new(1000),
        msg: to_binary(&create_order_msg).unwrap(),
    };

    // This message sends funds
    let res = router.execute_contract(buyer.clone(), Addr::unchecked(CW20), &send_msg, &[]);
    assert!(res.is_ok());

    // Verify that the buyer has sent the 2000
    let buyer_balance = Cw20Contract(Addr::unchecked(CW20))
        .balance::<_, _, Empty>(&router, buyer.clone())
        .unwrap();
    assert_eq!(buyer_balance, Uint128::new(3000));

    // This message does not send any funds
    let err = router.execute_contract(buyer, Addr::unchecked(CW20), &incorrect_send_msg, &[]);
    assert!(err.is_err());
}

#[test]
fn try_cancel_order() {
    let mut router = setup_contract();

    let seller = Addr::unchecked(ADMIN);
    let buyer = Addr::unchecked(BUYER);
    let unauthorized = Addr::unchecked("buyer2");

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(seller, Addr::unchecked(COMMERCE), &create_listing_msg, &[]);
    assert!(res.is_ok());

    // Cost should be 2000
    let create_order_msg = ReceiveMsg::CreateOrder {
        items: vec![OrderItem {
            listing_id: 1,
            options: vec![OrderOption {
                option_id: 1,
                selected_option: ListingOptionItem::new(
                    "M",
                    None,
                    Addr::unchecked(CW20).to_string(),
                ),
            }],
            amount: 2,
        }],
    };

    let send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(COMMERCE).to_string(),
        amount: Uint128::new(2000),
        msg: to_binary(&create_order_msg).unwrap(),
    };

    // This message sends funds
    let res = router.execute_contract(buyer.clone(), Addr::unchecked(CW20), &send_msg, &[]);
    assert!(res.is_ok());

    // Verify that the buyer has sent the 2000
    let buyer_balance = Cw20Contract(Addr::unchecked(CW20))
        .balance::<_, _, Empty>(&router, buyer.clone())
        .unwrap();
    assert_eq!(buyer_balance, Uint128::new(3000));

    let cancel_order_msg = ExecuteMsg::CancelOrder { id: 1 };

    // This is sent by the creator of the order
    let res = router.execute_contract(
        buyer.clone(),
        Addr::unchecked(COMMERCE),
        &cancel_order_msg,
        &[],
    );
    assert!(res.is_ok());

    // This is sent by an unauthorized party
    let err = router.execute_contract(
        unauthorized,
        Addr::unchecked(COMMERCE),
        &cancel_order_msg,
        &[],
    );
    assert!(err.is_err());

    // Verify that the buyer has gotten the funds back
    let buyer_balance = Cw20Contract(Addr::unchecked(CW20))
        .balance::<_, _, Empty>(&router, buyer)
        .unwrap();
    assert_eq!(buyer_balance, Uint128::new(5000));
}

#[test]
fn try_update_order() {
    let mut router = setup_contract();

    let seller = Addr::unchecked(ADMIN);
    let buyer = Addr::unchecked(BUYER);
    let unauthorized = Addr::unchecked("seller2");

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        seller.clone(),
        Addr::unchecked(COMMERCE),
        &create_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    // Cost should be 2000
    let create_order_msg = ReceiveMsg::CreateOrder {
        items: vec![OrderItem {
            listing_id: 1,
            options: vec![OrderOption {
                option_id: 1,
                selected_option: ListingOptionItem::new(
                    "M",
                    None,
                    Addr::unchecked(CW20).to_string(),
                ),
            }],
            amount: 2,
        }],
    };

    let send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(COMMERCE).to_string(),
        amount: Uint128::new(2000),
        msg: to_binary(&create_order_msg).unwrap(),
    };

    let incorrect_send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(COMMERCE).to_string(),
        amount: Uint128::new(1000),
        msg: to_binary(&create_order_msg).unwrap(),
    };

    // This message sends funds
    let res = router.execute_contract(buyer.clone(), Addr::unchecked(CW20), &send_msg, &[]);
    assert!(res.is_ok());

    // Verify that the buyer has sent the 2000
    let buyer_balance = Cw20Contract(Addr::unchecked(CW20))
        .balance::<_, _, Empty>(&router, buyer.clone())
        .unwrap();
    assert_eq!(buyer_balance, Uint128::new(3000));

    // This message does not send any funds
    let err = router.execute_contract(
        buyer.clone(),
        Addr::unchecked(CW20),
        &incorrect_send_msg,
        &[],
    );
    assert!(err.is_err());

    let update_order_msg = ExecuteMsg::UpdateOrder {
        id: 1,
        status: OrderStatus::Fulfilling,
        tracking: Some(TrackingInfo {
            provider: String::from("CANADA POST"),
            url: String::from("https://canadapost.ca/tracking/test"),
        }),
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(seller, Addr::unchecked(COMMERCE), &update_order_msg, &[]);
    assert!(res.is_ok());

    // This user isn't!
    let err = router.execute_contract(
        unauthorized,
        Addr::unchecked(COMMERCE),
        &update_order_msg,
        &[],
    );
    assert!(err.is_err());

    let cancel_order_msg = ExecuteMsg::CancelOrder { id: 1 };

    // Buyer can't cancel an order if it is fulfilling
    let res = router.execute_contract(buyer, Addr::unchecked(COMMERCE), &cancel_order_msg, &[]);
    assert!(res.is_err());
}

#[test]
fn try_complete_order() {
    let mut router = setup_contract();

    let seller = Addr::unchecked(ADMIN);
    let buyer = Addr::unchecked(BUYER);

    let create_listing_msg = ExecuteMsg::CreateListing {
        active: true,
        price: Uint128::from(1000u128),
        attributes: Attributes {
            name: String::from("WHITE TAPED SLEEVE T-SHIRT"),
            images: vec![String::from(
                "https://i.ibb.co/Dp3bbDT/image-b98a8387-b183-4339-bb73-609c119be18a-1600x.jpg",
            )],
            description: None,
        },
        options: vec![ListingOption::new(
            1,
            "SIZE",
            None,
            vec![
                ListingOptionItem::new("M", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new("L", None, Addr::unchecked(CW20).to_string()),
                ListingOptionItem::new(
                    "XL",
                    Some(Uint128::from(200u128)),
                    Addr::unchecked(CW20).to_string(),
                ),
            ],
        )],
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        seller.clone(),
        Addr::unchecked(COMMERCE),
        &create_listing_msg,
        &[],
    );
    assert!(res.is_ok());

    // Cost should be 2000
    let create_order_msg = ReceiveMsg::CreateOrder {
        items: vec![OrderItem {
            listing_id: 1,
            options: vec![OrderOption {
                option_id: 1,
                selected_option: ListingOptionItem::new(
                    "M",
                    None,
                    Addr::unchecked(CW20).to_string(),
                ),
            }],
            amount: 2,
        }],
    };

    let send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(COMMERCE).to_string(),
        amount: Uint128::new(2000),
        msg: to_binary(&create_order_msg).unwrap(),
    };

    // This message sends funds
    let res = router.execute_contract(buyer.clone(), Addr::unchecked(CW20), &send_msg, &[]);
    assert!(res.is_ok());

    // Verify that the buyer has sent the 2000
    let buyer_balance = Cw20Contract(Addr::unchecked(CW20))
        .balance::<_, _, Empty>(&router, buyer.clone())
        .unwrap();
    assert_eq!(buyer_balance, Uint128::new(3000));

    let update_order_msg = ExecuteMsg::UpdateOrder {
        id: 1,
        status: OrderStatus::Shipped,
        tracking: Some(TrackingInfo {
            provider: String::from("CANADA POST"),
            url: String::from("https://canadapost.ca/tracking/test"),
        }),
    };

    // This user is authorized to execute the message
    let res = router.execute_contract(
        seller.clone(),
        Addr::unchecked(COMMERCE),
        &update_order_msg,
        &[],
    );
    assert!(res.is_ok());

    // Complete the order
    let complete_order_msg = ExecuteMsg::CompleteOrder { id: 1 };

    // This is sent by an unauthorized party
    let err = router.execute_contract(
        buyer.clone(),
        Addr::unchecked(COMMERCE),
        &complete_order_msg.clone(),
        &[],
    );
    assert!(err.is_err());

    // This user is authorized to execute the message
    let res = router.execute_contract(
        seller.clone(),
        Addr::unchecked(COMMERCE),
        &complete_order_msg,
        &[],
    );
    assert!(res.is_ok());

    let res: trust::response::PendingReviewsResponse = router
        .wrap()
        .query_wasm_smart(
            Addr::unchecked(TRUST),
            &trust::msg::QueryMsg::PendingReviewsByReviewer {
                reviewer: buyer.to_string(),
            },
        )
        .unwrap();

    println!("{:?}", res);

    // Buyer leaves a review to the seller
    let buyer_review_msg = trust::msg::ExecuteMsg::Review {
        address: seller.to_string(),
        review: ReviewResult::ThumbsUp,
    };

    let res = router.execute_contract(buyer, Addr::unchecked(TRUST), &buyer_review_msg, &[]);

    assert!(res.is_ok());

    let res: trust::response::TrustInfoResponse = router
        .wrap()
        .query_wasm_smart(
            Addr::unchecked(TRUST),
            &trust::msg::QueryMsg::TrustInfo {
                address: seller.to_string(),
            },
        )
        .unwrap();

    println!("{:?}", res);

    // Stake some tokens!
    let stake_msg = cw20_stake::msg::ReceiveMsg::Stake {};
    let send_msg = Cw20ExecuteMsg::Send {
        contract: Addr::unchecked(CW20_STAKE).to_string(),
        amount: Uint128::from(5u128),
        msg: to_binary(&stake_msg).unwrap(),
    };

    // This message sends funds to the staking contract
    let res = router.execute_contract(seller.clone(), Addr::unchecked(CW20), &send_msg, &[]);
    assert!(res.is_ok());

    // Update staking info for seller
    let update_staking_msg = trust::msg::ExecuteMsg::UpdateStakingInfo {
        address: seller.to_string(),
    };

    let mut count = 0;

    loop {
        if count > 14 {
            break;
        };

        let res = router.execute_contract(
            seller.clone(),
            Addr::unchecked(TRUST),
            &update_staking_msg,
            &[],
        );

        assert!(res.is_ok());
        count += 1;
    }

    let res: trust::response::TrustInfoResponse = router
        .wrap()
        .query_wasm_smart(
            Addr::unchecked(TRUST),
            &trust::msg::QueryMsg::TrustInfo {
                address: seller.to_string(),
            },
        )
        .unwrap();

    println!("{:?}", res);
}
