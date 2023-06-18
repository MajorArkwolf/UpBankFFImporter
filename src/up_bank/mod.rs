use chrono::{Local, NaiveDateTime, TimeZone, Utc};
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
    pub categories: Vec<Categorie>,
    pub tags: Vec<Tag>,
}

fn generate_url(path: &str) -> String {
    format!("https://api.up.com.au/api/v1/{path}")
}

impl UpBank {
    pub fn create(access_token: String) -> Result<Self> {
        if access_token.is_empty() {
            return Err(eyre!("Up Bank access token was not set"));
        }

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

    pub async fn get_all_transactions(
        &self,
        start_date: Option<chrono::naive::NaiveDate>,
        end_date: Option<chrono::naive::NaiveDate>,
    ) -> Result<Vec<Transaction>> {
        let mut transactions: Vec<Transaction> = vec![];

        let mut params: Vec<(String, String)> = vec![];

        let time = Utc::now().naive_local().time();

        if let Some(date) = start_date {
            let date_time = Local
                .from_local_datetime(&NaiveDateTime::new(date, time))
                .unwrap();
            let date_filter = ("filter[since]".to_string(), date_time.to_rfc3339());
            params.push(date_filter);
        }

        if let Some(date) = end_date {
            let date_time = Local
                .from_local_datetime(&NaiveDateTime::new(date, time))
                .unwrap();
            let date_filter = ("filter[until]".to_string(), date_time.to_rfc3339());
            params.push(date_filter);
        }

        let mut request_url = generate_url("transactions");

        loop {
            let mut transaction_data = self
                .client
                .get(request_url)
                .query(&params)
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
