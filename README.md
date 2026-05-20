# backpack-sdk-rust

Unofficial Rust SDK for the [Backpack Exchange](https://backpack.exchange/) API.

## Features

- Public REST endpoints (markets, tickers, depth, klines, trades, etc.)
- Authenticated endpoints (account info, borrow/lend, collateral)
- WebSocket streaming for real-time data
- Ed25519 signing for authenticated requests

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
backpack-sdk-rust = "0.1.0"
```

### Public REST API

```rust
use backpack_sdk_rust::BackpackClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BackpackClient::new();

    let assets = client.get_assests().await?;
    let markets = client.get_markets(None).await?;
    let depth = client.get_depth("SOL_USDC", None).await?;
    let ticker = client.get_ticker("SOL_USDC", None).await?;
    let klines = client.get_klines("SOL_USDC", "1h", 1700000000000, None, None).await?;
    let trades = client.get_recent_trades("SOL_USDC", Some(100)).await?;

    Ok(())
}
```

### Authenticated REST API

```rust
use backpack_sdk_rust::BackpackClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BackpackClient::with_signer("your-base64-private-key")?;
    let account = client.get_account().await?;
    Ok(())
}
```

### WebSocket Streaming

```rust
use backpack_sdk_rust::BackpackClient;
use serde::Deserialize;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TradeEvent {
    symbol: String,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "t")]
    trade_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BackpackClient::new();
    let (tx, mut rx) = mpsc::channel::<TradeEvent>(256);

    tokio::spawn(async move {
        client.subscribe("trade.BTC_USDC_PERP", tx).await.unwrap();
    });

    while let Some(trade) = rx.recv().await {
        println!("{:?}", trade);
    }
    Ok(())
}
```

## Available Methods

| Method | Description | Auth |
|---|---|---|
| `get_assests()` | Get all assets | No |
| `get_markets(type)` | Get markets, optionally filtered by type | No |
| `get_market(symbol)` | Get a single market | No |
| `get_depth(symbol, limit)` | Get order book depth | No |
| `get_ticker(symbol, interval)` | Get ticker for a symbol | No |
| `get_tickers(interval)` | Get all tickers | No |
| `get_klines(symbol, interval, start, end, price_type)` | Get kline/candlestick data | No |
| `get_recent_trades(symbol, limit)` | Get recent trades | No |
| `get_historical_trades(symbol, limit, offset)` | Get historical trades | No |
| `get_funding_rates(symbol, limit, offset)` | Get funding rates | No |
| `get_open_interest(symbol)` | Get open interest | No |
| `get_mark_prices(symbol, market_type)` | Get mark prices | No |
| `get_prediction_events(...)` | Get prediction events | No |
| `get_borrow_lend_markets()` | Get borrow/lend markets | No |
| `get_borrow_lend_market_history(interval, symbol)` | Get borrow/lend market history | No |
| `get_borrow_lend_apy()` | Get borrow/lend APY | No |
| `get_collateral()` | Get collateral info | No |
| `get_account()` | Get account info | Yes |
| `subscribe(stream, tx)` | Subscribe to a WebSocket stream | No |
| `subscribe_multiple(streams, tx)` | Subscribe to multiple WebSocket streams | No |

## API Key Setup

1. Generate an Ed25519 keypair
2. Upload the public key to Backpack Exchange under Settings > API
3. Pass the base64-encoded private key to `BackpackClient::with_signer()`
