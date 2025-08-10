use serde::{Deserialize, Serialize};

/// Represents a DNS record as returned by the Simply.com DNS API.
///
/// Fields map directly to the API response schema. For details, refer to the official API docs.
/// See: https://www.simply.com/en/docs/api/
#[derive(Debug, serde::Deserialize)]
pub struct DnsRecord {
    /// Unique identifier for the DNS record.
    pub record_id: DnsRecordId,
    /// The DNS record name (subdomain), e.g. "hello" in "hello.example.com".
    pub name: String,
    /// Time to live (TTL) in seconds for the DNS record.
    pub ttl: u32,
    /// Data field for the DNS record (e.g., IP address for "A" record, target domain for "CNAME", etc.).
    pub data: String,
    /// Type of DNS record ("A", "CNAME", "MX", etc.).
    #[serde(rename = "type")]
    pub record_type: String,
    /// Priority value for records that require it (e.g., MX, SRV); optional.
    pub priority: Option<u32>,
    /// Optional comment or metadata for the record.
    pub comment: Option<String>,
}

impl From<DnsRecordResponse> for DnsRecord {
    fn from(value: DnsRecordResponse) -> Self {
        DnsRecord {
            record_id: DnsRecordId {
                id: value.record_id,
            },
            name: value.name,
            ttl: value.ttl,
            data: value.data,
            record_type: value.record_type,
            priority: value.priority,
            comment: value.comment,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct DnsRecordResponse {
    /// Unique identifier for the DNS record.
    pub record_id: u32,
    /// The DNS record name (subdomain), e.g. "hello" in "hello.example.com".
    pub name: String,
    /// Time to live (TTL) in seconds for the DNS record.
    pub ttl: u32,
    /// Data field for the DNS record (e.g., IP address for "A" record, target domain for "CNAME", etc.).
    pub data: String,
    /// Type of DNS record ("A", "CNAME", "MX", etc.).
    #[serde(rename = "type")]
    pub record_type: String,
    /// Priority value for records that require it (e.g., MX, SRV); optional.
    pub priority: Option<u32>,
    /// Optional comment or metadata for the record.
    pub comment: Option<String>,
}

/// Response for listing DNS records from the API.
#[derive(Debug, Deserialize)]
pub(crate) struct ListDnsRecordsResponse {
    /// The list of DNS records returned for a domain.
    pub records: Vec<DnsRecordResponse>,
}

/// Request payload for creating a DNS record via the API.
#[derive(Debug, Serialize)]
pub struct CreateDnsRecordRequest {
    /// Type of DNS record to create ("A", "CNAME", "MX", etc.).
    #[serde(rename = "type")]
    pub record_type: String,
    /// The DNS record name (subdomain), e.g. "hello" in "hello.example.com".
    pub name: String,
    /// Data (e.g., IP address or target value).
    pub data: String,
    /// Priority value for records that require it.
    pub priority: Option<u32>,
    /// Time to live (TTL) for the record, in seconds.
    pub ttl: Option<u32>,
    /// Optional comment or metadata for the record.
    pub comment: Option<String>,
}

/// Response for creating a DNS record via the API.
#[derive(Debug, Deserialize)]
pub(crate) struct CreateDnsRecordResponse {
    /// List of created record IDs (if any).
    pub record: Option<Vec<DnsRecordId>>, // Record id(s) created
}

/// Structure representing the ID of a newly created DNS record.
#[derive(Debug, Deserialize)]
pub struct DnsRecordId {
    /// ID of the added DNS record.
    pub(crate) id: u32,
}

/// Request payload for updating an existing DNS record via the API.
#[derive(Debug, Serialize)]
pub struct UpdateDnsRecordRequest {
    /// Type of DNS record to update ("A", "CNAME", "MX", etc.).
    #[serde(rename = "type")]
    pub record_type: String,
    /// The DNS record name (subdomain), e.g. "hello" in "hello.example.com".
    pub name: String,
    /// New data for the record (IP, target, etc.).
    pub data: String,
    /// Priority value for records that require it.
    pub priority: Option<u32>,
    /// Time to live (TTL) for the record, in seconds.
    pub ttl: Option<u32>,
}

/// Response for deleting a DNS record via the API.
#[derive(Debug, Deserialize)]
pub(crate) struct GeneralResponse {
    /// Message from the API, e.g. "success" or error details.
    pub message: Option<String>,
}
