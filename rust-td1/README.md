# TD – Stock Price Collector (Rust / Async)

## Overview

This project implements an asynchronous stock price collector written in Rust.  
It periodically generates a mock stock price and stores it in a PostgreSQL database.

The goal of this assignment is to demonstrate:
- asynchronous programming with Tokio,
- task scheduling and concurrency,
- database interaction using SQLx,
- clean structure and robust runtime behavior.

---

## Features

- Periodic price generation (every 60 seconds)
- Realistic mock stock price simulation
- Persistent storage in PostgreSQL
- Fully asynchronous execution
- Structured logging with tracing
- Graceful shutdown on Ctrl + C

---

## Project Structure

.
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md

---

## Technical Stack

- Rust
- Tokio – asynchronous runtime
- SQLx – PostgreSQL client
- Tracing – structured logging
- Chrono – time handling
- Rand – random data generation

---

## Database Schema

The application expects the following table:

```sql
CREATE TABLE stock_prices (
    id SERIAL PRIMARY KEY,
    symbol TEXT NOT NULL,
    price NUMERIC NOT NULL,
    source TEXT NOT NULL,
    timestamp BIGINT NOT NULL
);

Configuration

The database connection is configured using an environment variable:

export DATABASE_URL=postgres://user:password@localhost:5432/database_name

How It Works
	1.	The application connects to a PostgreSQL database.
	2.	Every 60 seconds:
	•	a mock stock price is generated for a given symbol,
	•	the price is stored in the database.
	3.	The application listens for Ctrl + C and shuts down gracefully.

⸻

Running the Application

cargo run

INFO  Connected to database.
INFO  Fetching mock price for AAPL
INFO  Saved price: StockPrice { symbol: "AAPL", price: 172.41, source: "mock", timestamp: ... }

Design Choices
	•	BigDecimal is used to store prices to avoid floating-point precision issues.
	•	Tokio + async/await ensures non-blocking execution and scalability.
	•	Tracing provides structured and production-grade logging.
	•	The code is organized to separate concerns (data fetching, persistence, runtime control).

⸻

Author

Alexandre Fau
ESILV