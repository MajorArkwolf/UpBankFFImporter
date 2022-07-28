use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMap {
    pub up_account_id: String,
    pub fire_fly_account_id: String,
}

impl AccountMap {
    pub fn create(up_account_id: String, fire_fly_account_id: String) -> Self {
        Self {up_account_id, fire_fly_account_id}
    }
}
