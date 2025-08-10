use reqwest::Client;
use thiserror::Error;

use crate::api::{
    CreateDnsRecordRequest, CreateDnsRecordResponse, DeleteDnsRecordResponse, DnsRecord,
    ListDnsRecordsResponse, UpdateDnsRecordRequest, UpdateDnsRecordResponse,
};

/// Error type for Simply.com client
#[derive(Debug, Error)]
pub enum SimplyClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error: {0} ({1})")]
    Api(u32, String),
}

/// Async Simply.com API client
pub struct SimplyClient {
    account: String,
    api_key: String,
    base_url: String,
    client: Client,
}

impl SimplyClient {
    /// Creates a new SimplyClient
    pub fn new(account: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            account: account.into(),
            api_key: api_key.into(),
            base_url: "https://api.simply.com/2/".to_string(),
            client: Client::new(),
        }
    }
}

impl SimplyClient {
    /// List DNS records for a (domain)
    pub async fn list_dns_records(
        &self,
        domain: &str,
    ) -> Result<Vec<DnsRecord>, SimplyClientError> {
        let url = format!(
            "{}/my/products/{}/dns/records",
            self.base_url.trim_end_matches('/'),
            domain
        );
        let res = self
            .client
            .get(&url)
            .basic_auth(&self.account, Some(&self.api_key))
            .send()
            .await?;
        let resp: ListDnsRecordsResponse = res.json().await?;
        Ok(resp.records)
    }
    /// Create a DNS record for a product (domain)
    pub async fn create_dns_record(
        &self,
        domain: &str,
        req: CreateDnsRecordRequest,
    ) -> Result<CreateDnsRecordResponse, SimplyClientError> {
        let url = format!(
            "{}/my/products/{}/dns/records",
            self.base_url.trim_end_matches('/'),
            domain
        );
        let res = self
            .client
            .post(&url)
            .basic_auth(&self.account, Some(&self.api_key))
            .json(&req)
            .send()
            .await?;
        let resp = res.json().await?;
        Ok(resp)
    }

    /// Update a DNS record for a product (domain)
    pub async fn update_dns_record(
        &self,
        domain: &str,
        record_id: u32,
        req: UpdateDnsRecordRequest,
    ) -> Result<UpdateDnsRecordResponse, SimplyClientError> {
        let url = format!(
            "{}/my/products/{}/dns/records/{}",
            self.base_url.trim_end_matches('/'),
            domain,
            record_id
        );
        let res = self
            .client
            .put(&url)
            .basic_auth(&self.account, Some(&self.api_key))
            .json(&req)
            .send()
            .await?;
        let resp = res.json().await?;
        Ok(resp)
    }

    /// Delete a DNS record for a product (domain)
    pub async fn delete_dns_record(
        &self,
        domain: &str,
        record_id: u32,
    ) -> Result<DeleteDnsRecordResponse, SimplyClientError> {
        let url = format!(
            "{}/my/products/{}/dns/records/{}",
            self.base_url.trim_end_matches('/'),
            domain,
            record_id
        );
        let res = self
            .client
            .delete(&url)
            .basic_auth(&self.account, Some(&self.api_key))
            .send()
            .await
            .map_err(SimplyClientError::Http)?;
        let resp = res.json().await?;
        Ok(resp)
    }
}
