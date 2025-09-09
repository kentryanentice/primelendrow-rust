use candid::Principal;
use ic_cdk::{api::msg_caller, storage};
use ic_cdk_macros::{post_upgrade, pre_upgrade, update};
use std::cell::RefCell;

use crate::users::{manager::UsersManager, types::User};
use crate::vaults::{manager::VaultsManager, types::Vault};

thread_local! {
    static USERS_MANAGER: RefCell<UsersManager> = RefCell::new(UsersManager::new());
    static VAULTS_MANAGER: RefCell<VaultsManager> = RefCell::new(VaultsManager::new());
}

const ADMINISTRATOR1: &str = "4gbnk-pwauj-bpju5-aie6k-3nhje-r65no-nkk5o-p2w6d-moygy-i3dii-cae";
const ADMINISTRATOR2: &str = "xcr7h-tl77o-s6leb-dupns-eutzd-n6rl7-pk5qz-xfkih-stclv-uty4q-iae";
const ADMINISTRATOR_LOCAL: &str = "tghg6-j4n42-aednl-7ua2a-pvnbn-6ikqw-lcu55-5rxd5-unr6j-wf25b-tqe";

fn assert_authorized(principal_id: Principal, err_msg: &str) -> Result<(), String> {
    if msg_caller() != principal_id {
        Err(err_msg.to_string())
    } else {
        Ok(())
    }
}

fn is_admin(principal_id: &Principal) -> bool {
    let id = principal_id.to_text();
    id == ADMINISTRATOR1 || id == ADMINISTRATOR2 || id == ADMINISTRATOR_LOCAL
}

#[pre_upgrade]
fn pre_upgrade() {
    let user_entries = USERS_MANAGER.with(|m| m.borrow().get_entries());
    let vault_entries = VAULTS_MANAGER.with(|m| m.borrow().get_entries());
    storage::stable_save((user_entries, vault_entries)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (user_entries, vault_entries) = storage::stable_restore::<(Vec<(Principal, User)>, Vec<(Principal, Vault)>)>().unwrap();
    USERS_MANAGER.with(|m| m.borrow_mut().init(user_entries));
    VAULTS_MANAGER.with(|m| m.borrow_mut().init(vault_entries));
}

#[update]
pub fn create_user(principal_id: Principal) -> Result<User, String> {
    let caller = msg_caller();
    assert_authorized(principal_id, "Unauthorized Access. User generation has been rejected.")?;
    USERS_MANAGER.with(|m| m.borrow_mut().create_user(caller))
}

#[update]
pub fn update_user(principal_id: Principal, primary_id: Vec<u8>, first_name: String, middle_name: String, last_name: String, mobile: String) -> Result<User, String> {
    let caller = msg_caller();
    assert_authorized(principal_id, "Unauthorized Access. User upgrade has been rejected.")?;
    USERS_MANAGER.with(|m| m.borrow_mut().update_user(
        caller, primary_id, first_name, middle_name, last_name, mobile,
    ))
}

#[update]
pub fn get_user(principal_id: Principal) -> Result<User, String> {
    let caller = msg_caller();
    assert_authorized(principal_id, "Unauthorized Access. User fetching has been rejected.")?;
    USERS_MANAGER.with(|m| m.borrow().get_user(caller))
}

#[update]
pub fn verify_user(principal_id: Principal, user_principal: Principal) -> Result<User, String> {
    let caller = msg_caller();
    assert_authorized(principal_id, "Unauthorized Access. User upgrade has been rejected.")?;

    if !matches!(caller.to_text().as_str(), ADMINISTRATOR1 | ADMINISTRATOR_LOCAL) {
        return Err("Unauthorized State. User upgrade has been rejected.".to_string());
    }

    USERS_MANAGER.with(|m| m.borrow_mut().verify_user(caller, user_principal))
}

#[update]
pub fn get_all_users(principal_id: Principal) -> Result<Vec<User>, String> {
    let caller = msg_caller();
    assert_authorized(principal_id, "Unauthorized Access. User fetching has been rejected.")?;

    if !is_admin(&caller) {
        return Err("Unauthorized Access. User fetching has been rejected.".to_string());
    }

    Ok(USERS_MANAGER.with(|m| m.borrow().get_all_users()))
}

#[update]
pub fn create_vault(principal_id: Principal) -> Result<Vault, String> {
    let caller = msg_caller();
    assert_authorized(principal_id, "Unauthorized Access. Vault generation has been rejected.")?;
    VAULTS_MANAGER.with(|m| m.borrow_mut().create_vault(caller))
}