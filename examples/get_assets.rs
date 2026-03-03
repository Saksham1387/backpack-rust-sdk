use backpack_sdk_rust::BackpackClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the client. No API key needed for public endpoints.
    let client = BackpackClient::with_signer()?;

    let res = client.get_depth("SOL_USDC", None).await?;

    println!("{:?}", res);
    Ok(())
}
