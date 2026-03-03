// examples/get_assets.rs
//
// Run with: cargo run --example get_assets
//
// This file demonstrates how a user of your SDK would use it.
// It also doubles as a live integration test — if the API changes
// shape, this will break, alerting you to update your types.

use backpack_sdk_rust::BackpackClient;

// #[tokio::main] is a macro that:
//   1. Sets up the Tokio async runtime
//   2. Lets you use .await in main()
// Without this, Rust's main() can't be async.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the client. No API key needed for public endpoints.
    let client = BackpackClient::new();

    println!("Fetching assets from Backpack Exchange...\n");

    // Call our endpoint. The ? propagates any error up to main,
    // which will print it and exit with a non-zero status code.
    let assets = client.get_collateral().await?;

    println!("Found {} assets:\n", assets.len());

    for asset in &assets {
        println!("──────────────────────────");
        println!("Symbol:      {:?}", asset);
    }

    Ok(())
}
