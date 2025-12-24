# TD2 – WebSocket Programming in Rust (Tokio)

## Overview

This project was developed as part of **TD2** and aims to explore **asynchronous programming in Rust** using **Tokio** and **WebSockets**.

The work progressively covers:
- building a basic WebSocket echo server,
- broadcasting real-time data to multiple clients,
- integrating a real-time web dashboard using WebSockets.

All components are fully functional and tested.

---

## Implemented Features

### 1. WebSocket Echo Server
- WebSocket server built with `tokio-tungstenite`
- Accepts multiple client connections
- Sends a welcome message upon connection
- Echoes back all received messages
- Handles:
  - Text messages
  - Binary messages
  - Ping / Pong frames
  - Close frames
- Robust error handling
- Tested using:
  - Web browser clients
  - CLI tool `websocat`

File: `src/echo_server.rs`

---

### 2. Broadcast WebSocket Server (Pub/Sub)
- Implementation of a **publish/subscribe** pattern
- Uses `tokio::sync::broadcast`
- Periodic simulation of stock prices
- Broadcasts updates to all connected clients
- Concurrent handling of multiple WebSocket clients
- Messages are sent in **JSON format**

File: `src/broadcast_server.rs`

---

### 3. Real-Time Web Dashboard
- HTML / JavaScript client connected via WebSocket
- Displays real-time stock prices
- Automatic UI updates when new data arrives
- Automatic reconnection in case of disconnection
- Clean card-based visual layout

File: `dashboard.html`

---

## Project Structure

├── src/
│   ├── echo_server.rs       
│   ├── broadcast_server.rs   
├── dashboard.html            
├── Cargo.toml
├── Cargo.lock
└── README.md

---

## Technical Stack

- **Rust** (edition 2021)
- **Tokio** – asynchronous runtime
- **tokio-tungstenite** – WebSocket implementation
- **futures-util** – async streams and sinks
- **serde / serde_json** – JSON serialization
- **rand** – data simulation
- **chrono** – timestamp handling
- **env_logger / log** – structured logging
- **HTML / JavaScript** – real-time client interface

---

## How to Run the Project

### 1. Run the Echo Server
```bash
cargo run --bin echo_server

2. Run the Broadcast Server

cargo run --bin broadcast_server

3. Serve the Web Clients

python3 -m http.server 8000

Then open in your browser:
	•	Echo test client: http://127.0.0.1:8000/test_client.html
	•	Real-time dashboard: http://127.0.0.1:8000/dashboard.html

⸻

Results
	•	Fully functional asynchronous WebSocket servers
	•	Real-time broadcast of simulated stock prices
	•	Multiple clients receiving updates simultaneously
	•	Dynamic and responsive web dashboard

This project fulfills all the objectives of TD2.

⸻

Author

Alexandre Fau 
ESILV