# Casper Marketplace Event Parser 🧾

Rust microservice for parsing smart contract events from **Mystra.io NFT Marketplace** with royalties support on **Casper Network**.

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-brightorange.svg?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Casper Network](https://img.shields.io/badge/Casper-Network-blue.svg?style=flat)](https://cspr.network)
[![Docker](https://img.shields.io/badge/Docker-ready-blue.svg?style=flat&logo=docker)](https://www.docker.com/)

## 🚀 Project Overview

**`casper-marketplace-event-parser`** is a lightweight **Rust microservice** designed to monitor and parse events emitted by the Mystra.io NFT marketplace smart contract on Casper Network. It handles events like listings, sales, offers, cancellations, and royalty distributions, normalizing them for easy integration with backends, databases, or analytics pipelines.

The service connects to Casper nodes or Sidecar for real-time event streaming via **SSE (Server-Sent Events)**.

## ✨ Key Features

- 🔴 **Real-time** listening to Casper event streams with automatic reconnection
- 📊 **Parsing** of marketplace-specific events (`List`, `Buy`, `Offer`, `RoyaltyPaid`) per Casper Event Standard
- 💾 **Structured output** in JSON format for downstream consumers (HTTP, Kafka, PostgreSQL)
- ⚙️ **Configurable** via environment variables or TOML for contract addresses, RPC endpoints, and filters
- ⚡ **High performance** with async Rust (Tokio) and zero-copy parsing where possible
- 📈 **Prometheus metrics** for monitoring
- 🐳 **Docker-ready** deployment

## 📋 Prerequisites

```bash
# Rust 1.75+ (stable channel)
rustc --version

# Optional: Docker for containerized deployment
docker --version


🎯 Quick Start
1. Clone & Build
bash
git clone https://github.com/midware/casper-marketplace-event-parser
cd casper-marketplace-event-parser
cargo build --release
2. Configure
Create .env file:

CASPER_RPC_URL=https://rpc.mainnet.casper.network
MARKETPLACE_CONTRACT_HASH=your-contract-hash-here
OUTPUT_WEBHOOK_URL=http://your-backend/webhook  # Optional
3. Run
bash
cargo run --release
Docker:

bash
docker build -t casper-event-parser .
docker run -e CASPER_RPC_URL=... casper-event-parser
⚙️ Configuration
Parameter	Description	Default	Example
CASPER_RPC_URL	Casper node/Sidecar events endpoint	-	https://sidecar.mainnet.casper.network/events
MARKETPLACE_CONTRACT_HASH	Marketplace contract hash	-	marketplace-1234...
FILTER_EVENTS	Comma-separated event names	all	List,Buy,RoyaltyPaid
LOG_LEVEL	Logging verbosity	info	debug
OUTPUT_WEBHOOK_URL	HTTP webhook endpoint	-	http://localhost:8080/events
BATCH_SIZE	Events batch size	100	50
Full config available in config.toml or environment variables.

📤 Supported Events
Event	Description
List	NFT listed for sale
Buy	NFT purchased (incl. royalties)
Cancel	Listing cancelled
Offer	Bid placed
RoyaltyPaid	Royalty distribution to creators
Example parsed output:

json
{
  "event_type": "Buy",
  "timestamp": 1736784000,
  "block_height": 1234567,
  "nft_token_id": "nft-123",
  "collection": "mystra-collection-xyz",
  "buyer": "account-hash-abc123...",
  "seller": "account-hash-def456...",
  "price": "1000000000000000000",
  "royalties": [
    {
      "creator": "account-hash-xyz789...",
      "amount": "50000000000000000"
    }
  ]
}
🏗️ Architecture

graph TD
    A[Casper Node/Sidecar<br/>SSE Events] --> B[Event Listener<br/>Tokio Async]
    B --> C[Event Parser<br/>Custom Schemas]
    C --> D[Output Sink<br/>HTTP/Kafka/DB]
    D --> E[Backend/Database<br/>Analytics Pipeline]
    style A fill:#e1f5fe
    style E fill:#f3e5f5
🚀 Deployment Options
Docker

FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/casper-marketplace-event-parser /usr/local/bin/
CMD ["casper-marketplace-event-parser"]
bash
docker build -t casper-event-parser .
docker run -d --name parser \
  -e CASPER_RPC_URL=https://rpc.mainnet.casper.network \
  -p 8080:8080 \
  casper-event-parser
Kubernetes

apiVersion: apps/v1
kind: Deployment
metadata:
  name: casper-event-parser
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: parser
        image: casper-event-parser:latest
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
        env:
        - name: CASPER_RPC_URL
          value: "https://rpc.mainnet.casper.network"
Monitoring
Exposes Prometheus metrics on /metrics endpoint:

http://localhost:8080/metrics
🧪 Testing
bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Benchmarks
cargo bench
🤝 Contributing
Fork the repo

Create feature branch (git checkout -b feature/AmazingFeature)

Commit changes (git commit -m 'Add some AmazingFeature')

Push (git push origin feature/AmazingFeature)

Open Pull Request

Focus areas: tests, benchmarks, documentation.

📈 Performance
Metric	Value
Events/sec	10,000+
Memory usage	~50MB
CPU usage	<10% single core
Latency	<50ms event-to-output
🔗 Related Projects
Casper Event Standard

Mystra.io NFT Marketplace

Casper Docs: Events

Authors:
Kamil Szymoniak & Damian Sarnecki

📄 License
This project is licensed under the MIT License
