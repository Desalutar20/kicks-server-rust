use std::time::Duration;

use reqwest::Client;

use crate::{Error, Result};

pub fn build_http_client() -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| Error::Internal(format!("Failed to build http client: {:#?}", e)))
}
