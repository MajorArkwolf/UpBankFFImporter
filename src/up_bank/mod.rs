use color_eyre::eyre::{eyre, Result};
use reqwest::header;

use self::{accounts::Account, categories::Categorie, tags::Tag, transactions::Transaction};

pub mod accounts;
pub mod categories;
pub mod general;
pub mod pagination;
pub mod tags;
pub mod transactions;

#[derive(Debug, Clone)]
pub struct UpBank {
    client: reqwest::Client,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    pub categories: Vec<Categorie>,
    pub tags: Vec<Tag>,
}

fn generate_url(path: &str) -> String {
    format!("https://api.up.com.au/api/v1/{path}")
}

impl UpBank {
    pub fn create(access_token: String) -> Result<Self> {
        let access_token = format!("Bearer {access_token}",);
        let mut auth_value = header::HeaderValue::from_str(access_token.as_str())?;
        auth_value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/json"),
        );

        headers.insert(header::AUTHORIZATION, auth_value);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            accounts: Vec::new(),
            transactions: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
        })
    }

    pub async fn ping(&self) -> Result<()> {
        let request_url = generate_url("util/ping");
        let response = self.client.get(request_url).send().await?;

        if response.status() == 200 {
            Ok(())
        } else {
            Err(eyre!(
                "ping failed with code: {}, resp: {:?}",
                response.status(),
                response.headers()
            ))
        }
    }

    pub async fn populate_data(&mut self) -> Result<()> {
        self.accounts = self.get_accounts().await?;
        self.transactions = self.get_all_transactions().await?;
        self.categories = self.get_all_categories().await?;
        self.tags = self.get_all_tags().await?;
        Ok(())
    }

    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        let mut accounts: Vec<Account> = vec![];

        let mut request_url = generate_url("accounts");

        loop {
            let mut account_data = self
                .client
                .get(request_url)
                .send()
                .await?
                .json::<accounts::AccountsResponse>()
                .await?;

            accounts.append(&mut account_data.data);

            match account_data.links.next {
                Some(next_url) => request_url = next_url,
                None => break,
            }
        }

        Ok(accounts)
    }

    pub async fn get_all_transactions(&self) -> Result<Vec<Transaction>> {
        let mut transactions: Vec<Transaction> = vec![];

        let mut request_url = generate_url("transactions");

        loop {
            let mut transaction_data = self
                .client
                .get(request_url)
                .send()
                .await?
                .json::<transactions::TransactionResponse>()
                .await?;

            transactions.append(&mut transaction_data.data);

            match transaction_data.links.next {
                Some(next_url) => request_url = next_url,
                None => break,
            }
        }

        Ok(transactions)
    }

    pub async fn get_all_tags(&self) -> Result<Vec<tags::Tag>> {
        let mut tags: Vec<tags::Tag> = vec![];

        let mut request_url = generate_url("tags");

        loop {
            let mut tag_data = self
                .client
                .get(request_url)
                .send()
                .await?
                .json::<tags::TagsResponse>()
                .await?;

            tags.append(&mut tag_data.data);

            match tag_data.links.next {
                Some(next_url) => request_url = next_url,
                None => break,
            }
        }

        Ok(tags)
    }

    pub async fn get_all_categories(&self) -> Result<Vec<categories::Categorie>> {
        let request_url = generate_url("categories");
        let catergorie_data = self
            .client
            .get(request_url)
            .send()
            .await?
            .json::<categories::CategoriesResponse>()
            .await?;

        Ok(catergorie_data.data)
    }
}
