use std::collections::HashMap;
use candid::Principal;
use crate::vaults::types::Vault;

#[derive(Default)]
pub struct VaultsManager {
    vaults: HashMap<Principal, Vault>,
}

impl VaultsManager {
    pub fn new() -> Self {
        Self { vaults: HashMap::new() }
    }

    pub fn init(&mut self, entries: Vec<(Principal, Vault)>) {
        self.vaults.clear();
        for (p, v) in entries { self.vaults.insert(p, v); }
    }

    pub fn get_entries(&self) -> Vec<(Principal, Vault)> {
        self.vaults.iter().map(|(p, v)| (*p, v.clone())).collect()
    }

    pub fn create_vault(&mut self, caller: Principal) -> Result<Vault, String> {
        if self.vaults.contains_key(&caller) {
            return Err("Unauthorized State. Vault information already exist.".into());
        }
        let now = ic_cdk::api::time() as i64;
        let vault = Vault {
            principal_id: caller,
            admin_name: "PrimelendRow".into(),
            virtual_balance: 50_000_000,
            system_balance: 50_000_000,
            created_at: now,
            updated_at: now,
        };
        self.vaults.insert(caller, vault.clone());
        Ok(vault)
    }
}
