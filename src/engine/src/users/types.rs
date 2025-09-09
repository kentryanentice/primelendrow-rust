use candid::{CandidType, Deserialize};
use candid::Principal;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum UserType { Pending, Verifying, User, Admin }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum UserLevel { L1, L2, L100 }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum UserBadge { Normal, Verified, Administrator }

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct User {
    pub principal_id: Principal,
    pub profile: String,
    pub primary_id: Vec<u8>,
    pub username: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub mobile: String,
    pub user_type: UserType,
    pub user_level: UserLevel,
    pub user_badge: UserBadge,
    pub created_at: i64,
    pub updated_at: i64,
}