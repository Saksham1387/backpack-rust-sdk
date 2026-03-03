use backpack_sdk_rust::BackpackClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the client. No API key needed for public endpoints.
    let client = BackpackClient::with_signer("edlNY3TrsB/awIIBYFhPjRnwnGFHoOEiyvNAxTMnpko=")?;

    let res = client.get_account().await?;

    println!("{:?}", res);
    Ok(())
}
