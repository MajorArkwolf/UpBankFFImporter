use color_eyre::eyre::{eyre, Result};
use reqwest::header;

use self::accounts::{Account, AccountResponse, AccountsResponse};

mod accounts;
mod general;

#[derive(Debug)]
pub struct FireFly {
    client: reqwest::Client,
    base_url: String,
}

// /api/v1/search/accounts
// /api/v1/search/transactions
fn generate_url(base: &str, path: &str) -> String {
    format!("http://{base}/api/v1/{path}")
}

impl FireFly {
    pub fn create(access_token: String, base_url: String) -> Result<Self> {
        let access_token = format!("Bearer {access_token}",);
        let mut auth_value = header::HeaderValue::from_str(access_token.as_str())?;
        auth_value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/vnd.api+json"),
        );

        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        headers.insert("Authorization", auth_value);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, base_url })
    }

    pub async fn get_account(&self, id: &str) -> Result<Account> {
        let account = self
            .client
            .get(generate_url(&self.base_url, &format!("accounts/{id}")))
            .send()
            .await?
            .json::<AccountResponse>()
            .await?
            .data
            .ok_or(eyre!("account not found."))?;
        Ok(account)
    }
}
