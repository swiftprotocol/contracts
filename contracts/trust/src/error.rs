use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidContract, expected codeID: {expected_code_id}")]
    InvalidContract { expected_code_id: String },

    #[error("AwaitingReview, from reviewer: {reviewer}")]
    AwaitingReview { reviewer: String },

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("CCE: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
