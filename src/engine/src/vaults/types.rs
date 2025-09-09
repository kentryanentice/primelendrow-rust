use candid::{CandidType, Deserialize};
use candid::Principal;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Vault {
    pub principal_id: Principal,
    pub admin_name: String,
    pub virtual_balance: u64,
    pub system_balance: u64,
    pub created_at: i64,
    pub updated_at: i64,
}