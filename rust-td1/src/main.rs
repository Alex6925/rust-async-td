use chrono::Utc;
use rand::Rng;
use std::str::FromStr;
use tokio::{signal, time::{sleep, Duration, interval}};
use tracing::{info, error};
use tracing_subscriber;
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::types::BigDecimal;

#[derive(Debug)]
struct StockPrice {
    symbol: String,
    price: f64,
    source: String,
    timestamp: i64,
}

async fn fetch_mock_price(symbol: &str) -> StockPrice {
    info!("Fetching mock price for {}", symbol);
    sleep(Duration::from_millis(400)).await;

    let mut rng = rand::rng();
    let price = rng.random_range(100.0..200.0);

    StockPrice {
        symbol: symbol.to_string(),
        price,
        source: "mock".to_string(),
        timestamp: Utc::now().timestamp(),
    }
}

async fn save_price(pool: &PgPool, price: &StockPrice) -> Result<(), sqlx::Error> {
    let price_decimal = BigDecimal::from_str(&price.price.to_string())
        .expect("Failed to convert price to BigDecimal");

    sqlx::query!(
        r#"
        INSERT INTO stock_prices (symbol, price, source, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        price.symbol,
        price_decimal,
        price.source,
        price.timestamp,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .compact()
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("✅ Connected to database.");

    let mut ticker = interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let price = fetch_mock_price("AAPL").await;
                match save_price(&pool, &price).await {
                    Ok(_) => info!("💾 Saved price: {:?}", price),
                    Err(e) => error!("❌ Failed to save price: {:?}", e),
                }
            }
            _ = signal::ctrl_c() => {
                info!("Ctrl+C detected, shutting down gracefully.");
                break;
            }
        }
    }

    Ok(())
}