pub mod engine;
pub mod users;
pub mod vaults;

use crate::users::types::User;
use crate::vaults::types::Vault;
use candid::Principal;

pub use engine::{
    create_user,
    update_user,
    get_user,
    verify_user,
    get_all_users,
    create_vault,
};

ic_cdk::export_candid!();