use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

/// CommerceContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct CommerceContract(pub Addr);

impl CommerceContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }
}
