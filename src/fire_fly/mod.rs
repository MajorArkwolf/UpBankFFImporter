use color_eyre::eyre::{eyre, Result};
use reqwest::header;

#[derive(Debug)]
pub struct FireFly {
    client: reqwest::Client,
    base_url: String,
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

        headers.insert(header::AUTHORIZATION, auth_value);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, base_url })
    }
}
