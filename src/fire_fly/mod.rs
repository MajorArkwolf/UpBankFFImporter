use color_eyre::eyre::{eyre, Result};
use reqwest::header;
use tracing::debug;

use self::accounts::{Account, AccountResponse, AccountsResponse};

pub mod accounts;
pub mod general;
pub mod transaction;

#[derive(Debug, Clone)]
pub struct FireFly {
    client: reqwest::Client,
    base_url: String,
}

fn generate_url(base: &str, path: &str) -> String {
    format!("http://{base}/api/v1/{path}")
}

impl FireFly {
    pub fn create(access_token: String, base_url: String) -> Result<Self> {
        if access_token.is_empty() {
            return Err(eyre!("Firefly access token was empty"));
        }
        if base_url.is_empty() {
            return Err(eyre!("Firefly base url was empty"));
        }

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

    pub async fn get_all_accounts(&self) -> Result<Vec<Account>> {
        let accounts = self
            .client
            .get(generate_url(&self.base_url, "accounts"))
            .send()
            .await?
            .json::<AccountsResponse>()
            .await?
            .data;

        Ok(accounts)
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
            .ok_or_else(|| eyre!("Firefly ID({}), account not found.", id))?;
        Ok(account)
    }

    pub async fn get_account_by_account_number(&self, id: &str) -> Result<Account> {
        let mut url_address = generate_url(&self.base_url, "search/accounts");
        url_address = format!("{}?query={}&type=all&field=number", url_address, id);
        let accounts = self
            .client
            .get(url_address)
            .send()
            .await?
            .json::<AccountsResponse>()
            .await?
            .data;

        if accounts.len() != 1 {
            return Err(eyre!("When trying to find a unique account by account id, {} accounts were found, should have been 1", accounts.len()));
        }

        let account = accounts
            .into_iter()
            .next()
            .ok_or(eyre!("An account should have existed in the array"))?;

        Ok(account)
    }

    pub async fn find_transaction_by_external_id(
        &self,
        id: &str,
    ) -> Result<Vec<transaction::TransactionData>> {
        let mut url_address = generate_url(&self.base_url, "search/transactions");
        url_address = format!("{}?query=external_id_is%3A{}", url_address, id);

        let transactions = self
            .client
            .get(url_address)
            .send()
            .await?
            .json::<transaction::TransactionSearchRequest>()
            .await?;
        debug!(
            "Transaction ({}) returned {} enteries",
            id,
            transactions.data.len()
        );
        Ok(transactions.data)
    }

    pub async fn submit_new_transaction(
        &self,
        transaction: &transaction::TransactionPayload,
    ) -> Result<()> {
        let mut payload = transaction::TransactionInsertRequest {
            error_if_duplicate_hash: false,
            apply_rules: true,
            fire_webhooks: true,
            group_title: "".to_string(),
            transactions: Vec::new(),
        };
        payload.transactions.push(transaction.clone());
        let response = self
            .client
            .post(generate_url(&self.base_url, "transactions"))
            .json(&payload)
            .send()
            .await?;
        let status_code = response.status();
        if status_code != 200 {
            let error_info = response.text().await?;
            return Err(eyre!(
                "Failed to submit transaction({:?}), error code: {}, error: {}",
                transaction,
                status_code,
                error_info
            ));
        }
        Ok(())
    }

    pub async fn update_transaction(&self, transaction: transaction::Transaction) -> Result<()> {
        let address = format!("transactions/{}", transaction.transaction_journal_id);
        let url_address = generate_url(&self.base_url, &address);

        self.client
            .put(url_address)
            .json(&transaction)
            .send()
            .await?;

        Ok(())
    }
}
