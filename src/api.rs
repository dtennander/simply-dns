use serde::{Deserialize, Serialize};

/// DNS Record (matches API schema)
#[derive(Debug, serde::Deserialize)]
pub struct DnsRecord {
    pub record_id: u32,
    pub name: String,
    pub ttl: u32,
    pub data: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub priority: Option<u32>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListDnsRecordsResponse {
    pub records: Vec<DnsRecord>,
}

#[derive(Debug, Serialize)]
pub struct CreateDnsRecordRequest {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub data: String,
    pub priority: Option<u32>,
    pub ttl: Option<u32>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDnsRecordResponse {
    pub status: u32,
    pub message: String,
    pub record: Option<Vec<CreateDnsRecordId>>, // Record id(s) created
}

#[derive(Debug, Deserialize)]
pub struct CreateDnsRecordId {
    pub id: u32,
}

#[derive(Debug, Serialize)]
pub struct UpdateDnsRecordRequest {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub data: String,
    pub priority: Option<u32>,
    pub ttl: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDnsRecordResponse {
    pub status: Option<u32>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteDnsRecordResponse {
    pub status: u32,
    pub message: String,
}
