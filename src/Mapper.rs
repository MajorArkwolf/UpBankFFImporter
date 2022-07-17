use super::fire_fly::FireFly;
use super::up_bank::UpBank;
use color_eyre::eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMap {
    pub up_account_id: String,
    pub fire_fly_account_id: String,
}

impl AccountMap {
    pub async fn validate(&self, up_bank: UpBank, fire_fly: FireFly) -> Result<()> {
        up_bank
            .accounts
            .iter()
            .find(|&x| x.id == self.up_account_id)
            .ok_or(eyre!(
                "Up Bank did not have a account id that matched the one supplied"
            ))?;

        fire_fly.get_account(&self.fire_fly_account_id).await?;

        Ok(())
    }
}
