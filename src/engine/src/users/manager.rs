use std::collections::HashMap;
use candid::Principal;
use crate::users::types::*;

#[derive(Default)]
pub struct UsersManager {
    users: HashMap<Principal, User>,
}

impl UsersManager {
    pub fn new() -> Self {
        Self { users: HashMap::new() }
    }

    pub fn init(&mut self, entries: Vec<(Principal, User)>) {
        self.users.clear();
        for (p, u) in entries { self.users.insert(p, u); }
    }

    pub fn get_entries(&self) -> Vec<(Principal, User)> {
        self.users.iter().map(|(p, u)| (*p, u.clone())).collect()
    }

    pub fn create_user(&mut self, caller: Principal) -> Result<User, String> {
        if self.users.contains_key(&caller) {
            return Err("User information has already been registered.".into());
        }
        let now = ic_cdk::api::time() as i64;
        let caller_text = caller.to_text();
        let short_caller: String = caller_text.chars().take(5).collect();
        let username = "User_".to_string() + &short_caller;
        let user = User {
            principal_id: caller,
            profile: String::new(),
            primary_id: vec![],
            username,
            first_name: String::new(),
            middle_name: String::new(),
            last_name: String::new(),
            mobile: String::new(),
            user_type: UserType::Admin,
            user_level: UserLevel::L100,
            user_badge: UserBadge::Verified,
            created_at: now,
            updated_at: now,
        };
        self.users.insert(caller, user.clone());
        Ok(user)
    }

    pub fn update_user(&mut self, caller: Principal, primary_id: Vec<u8>, first: String, mid: String, last: String, mobile: String) -> Result<User, String> {
        let existing_user = self.users.get(&caller).ok_or("Unauthorized Access. User cannot be found.")?.clone();
        if !(matches!(existing_user.user_type, UserType::Pending | UserType::Verifying) || matches!(existing_user.user_level, UserLevel::L1) || matches!(existing_user.user_badge, UserBadge::Normal)) {
            return Err("Unauthorized Update. User upgrade has been rejected.".into());
        }

        if self.users.values().any(|u| u.first_name == first && u.middle_name == mid && u.last_name == last) {
            return Err("Update Failed. Fullname has already been taken.".into());
        }
        if self.users.values().any(|u| u.mobile == mobile) {
            return Err("Update Failed. Mobile No. has already been taken.".into());
        }

        let user = User {
            principal_id: caller,
            profile: existing_user.profile.clone(),
            primary_id,
            username: existing_user.username.clone(),
            first_name: first,
            middle_name: mid,
            last_name: last,
            mobile,
            user_type: UserType::Verifying,
            user_level: UserLevel::L1,
            user_badge: UserBadge::Normal,
            created_at: existing_user.created_at,
            updated_at: ic_cdk::api::time() as i64,
        };
        self.users.insert(caller, user.clone());
        Ok(user)
    }

    pub fn get_user(&self, caller: Principal) -> Result<User, String> {
        self.users.get(&caller).cloned().ok_or("Unauthorized State. User information cannot be found.".into())
    }

    pub fn verify_user(&mut self, caller: Principal, target: Principal) -> Result<User, String> {
        let caller_user = self.users.get(&caller).ok_or("Unauthorized State. Admin information cannot be found.")?;

        if !(matches!(caller_user.user_type, UserType::Admin) || matches!(caller_user.user_level, UserLevel::L100) || matches!(caller_user.user_badge, UserBadge::Verified)) {
            return Err("Unauthorized Admin State. User verification has been rejected.".into());
        }

        let existing_user = self.users.get(&target).ok_or("Unauthorized State. User information cannot be found.")?.clone();
        if !(matches!(existing_user.user_type, UserType::Verifying) && matches!(existing_user.user_level, UserLevel::L1) && matches!(existing_user.user_badge, UserBadge::Normal)) {
            return Err("Unauthorized User State. User verification has been rejected.".into());
        }

        let user = User {
            user_type: UserType::User,
            user_level: UserLevel::L2,
            user_badge: UserBadge::Verified,
            updated_at: ic_cdk::api::time() as i64,
            ..existing_user
        };
        self.users.insert(target, user.clone());
        Ok(user)
    }

    pub fn get_all_users(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }
}