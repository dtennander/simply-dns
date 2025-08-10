use reqwest::Client;
use thiserror::Error;

use crate::api::{
    CreateDnsRecordRequest, CreateDnsRecordResponse, DnsRecord, DnsRecordId, GeneralResponse,
    ListDnsRecordsResponse, UpdateDnsRecordRequest,
};

/// Error type for the Simply.com DNS API client.
///
/// Represents possible errors when interacting with the Simply.com DNS API.
#[derive(Debug, Error)]
pub enum SimplyClientError {
    /// There was an error with the HTTP request (network, invalid response, etc.).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    /// The response could not be parsed from JSON. Usually returned if the Simply.com API returns an unexpected or malformed JSON body.
    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    /// The Simply.com API returned an error status or message. The first value is the status code, the second is the message returned by the API.
    #[error("API error: {0} ({1})")]
    Api(u32, String),
}

/// Async client for the Simply.com DNS API.
///
/// Provides methods to interact with DNS records using the Simply.com API.
/// See: https://www.simply.com/en/docs/api/
///
/// Example usage:
/// ```rust
/// let client = SimplyClient::new("account", "api_key");
/// // ...
/// ```
pub struct SimplyClient {
    account: String,
    api_key: String,
    base_url: String,
    client: Client,
}

impl SimplyClient {
    /// Create a new Simply.com DNS API client instance.
    ///
    /// # Arguments
    /// * `account` - Your Simply.com account identifier.
    /// * `api_key` - The API key for authentication.
    ///
    /// For usage details, see: https://www.simply.com/en/docs/api/
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
    /// List all DNS records for a given domain.
    ///
    /// # Arguments
    /// * `domain` - The domain to list DNS records for.
    ///
    /// See: https://www.simply.com/en/docs/api/
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
        Ok(resp.records.into_iter().map(|r| r.into()).collect())
    }
    /// Create a new DNS record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The domain to create the DNS record under.
    /// * `req` - The DNS record request payload.
    ///
    /// See: https://www.simply.com/en/docs/api/
    pub async fn create_dns_record(
        &self,
        domain: &str,
        req: CreateDnsRecordRequest,
    ) -> Result<Vec<DnsRecordId>, SimplyClientError> {
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
        let resp: CreateDnsRecordResponse = res.json().await?;
        Ok(resp.record.unwrap_or_default())
    }

    /// Update an existing DNS record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The domain the DNS record belongs to.
    /// * `record_id` - The ID of the DNS record to update.
    /// * `req` - The updated DNS record payload.
    ///
    /// See: https://www.simply.com/en/docs/api/
    pub async fn update_dns_record(
        &self,
        domain: &str,
        record_id: DnsRecordId,
        req: UpdateDnsRecordRequest,
    ) -> Result<(), SimplyClientError> {
        let url = format!(
            "{}/my/products/{}/dns/records/{}",
            self.base_url.trim_end_matches('/'),
            domain,
            record_id.id,
        );
        let res = self
            .client
            .put(&url)
            .basic_auth(&self.account, Some(&self.api_key))
            .json(&req)
            .send()
            .await?;
        let status = res.status();
        if !status.is_success() {
            let resp: GeneralResponse = res.json().await?;
            return Err(SimplyClientError::Api(
                status.as_u16().into(),
                resp.message.unwrap_or_default(),
            ));
        }
        Ok(())
    }

    /// Delete a DNS record for a domain.
    ///
    /// # Arguments
    /// * `domain` - The domain the DNS record belongs to.
    /// * `record_id` - The ID of the DNS record to delete.
    ///
    /// See: https://www.simply.com/en/docs/api/
    pub async fn delete_dns_record(
        &self,
        domain: &str,
        record_id: DnsRecordId,
    ) -> Result<(), SimplyClientError> {
        let url = format!(
            "{}/my/products/{}/dns/records/{}",
            self.base_url.trim_end_matches('/'),
            domain,
            record_id.id,
        );
        let res = self
            .client
            .delete(&url)
            .basic_auth(&self.account, Some(&self.api_key))
            .send()
            .await
            .map_err(SimplyClientError::Http)?;
        let status = res.status();
        if !status.is_success() {
            let resp: GeneralResponse = res.json().await?;
            return Err(SimplyClientError::Api(
                status.as_u16().into(),
                resp.message.unwrap_or_default(),
            ));
        }
        Ok(())
    }
}
