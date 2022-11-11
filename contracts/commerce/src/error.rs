use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid order")]
    InvalidOrder {},

    #[error("Not enough images")]
    NotEnoughImages {},

    #[error("At least one active order")]
    ActiveOrder {},

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("CCE: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
