// use backpack_sdk_rust::BackpackClient;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Create the client. No API key needed for public endpoints.
//     let client = BackpackClient::with_signer()?;

//     let res = client.get_depth("SOL_USDC", None).await?;

//     println!("{:?}", res);
//     Ok(())
// }

use backpack_sdk_rust::BackpackClient;
use serde::Deserialize;
use tokio::sync::mpsc;

// Define the shape of trade events from "trade.<symbol>"
// The "data" field of each WS message gets deserialized into this.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TradeEvent {
    symbol: String,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "m")]
    is_buyer_maker: bool,
    #[serde(rename = "t")]
    trade_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Same client as REST — no separate WS client needed
    let client = BackpackClient::new();

    // User creates the channel — full control over buffer size
    let (tx, mut rx) = mpsc::channel::<TradeEvent>(256);

    println!("Subscribing to SOL_USDC trades...\n");

    // subscribe() is async and loops until disconnection.
    // We spawn it so it runs concurrently with our reading loop below.
    tokio::spawn(async move {
        if let Err(e) = client.subscribe("trade.BTC_USDC_PERP", tx).await {
            eprintln!("Subscription error: {e}");
        }
    });

    // Read events as they arrive — this is in our main task
    println!(
        "{:<10} {:<15} {:<15} {}",
        "SIDE", "PRICE", "QUANTITY", "TRADE ID"
    );
    println!("{}", "─".repeat(55));

    while let Some(trade) = rx.recv().await {
        println!("{:?}", trade);
        let side = if trade.is_buyer_maker { "SELL" } else { "BUY " };
        println!(
            "{:<10} {:<15} {:<15} #{}",
            side, trade.price, trade.quantity, trade.trade_id
        );
    }

    println!("Stream ended.");
    Ok(())
}
