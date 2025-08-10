# simply-dns

An async Rust client for the [Simply.com DNS API](https://www.simply.com/se/docs/api/).

## Example

```rust
use simply_dns::{SimplyClient, api::{CreateDnsRecordRequest}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimplyClient::new("your_account", "your_api_key");
    let domain = "example.com";

    // List DNS records
    let records = client.list_dns_records(domain).await?;
    println!("DNS records: {:?}", records);

    // Create a new DNS record
    let create_req = CreateDnsRecordRequest {
        record_type: "A".to_string(),
        name: "www".to_string(),
        data: "192.168.1.1".to_string(),
        priority: None,
        ttl: Some(3600),
        comment: Some("Created via simply-dns".to_string()),
    };
    let create_resp = client.create_dns_record(domain, create_req).await?;
    println!("Create response: {:?}", create_resp);

    // Update a DNS record
    // let update_req = ...
    // client.update_dns_record(domain, record_id, update_req).await?;

    // Delete a DNS record
    // client.delete_dns_record(domain, record_id).await?;

    Ok(())
}
```

## Contributions

All contributions are appreciated!
