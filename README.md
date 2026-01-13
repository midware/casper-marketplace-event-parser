Casper Marketplace Event Parser
Rust microservice for parsing smart contract events from Mystra.io NFT Marketplace with royalties support on Casper Network.
​

Project Overview
This repository hosts a lightweight Rust microservice designed to monitor and parse events emitted by the Mystra.io NFT marketplace smart contract on Casper Network. It handles events like listings, sales, offers, cancellations, and royalty distributions, normalizing them for easy integration with backends, databases, or analytics pipelines.
​

The service connects to Casper nodes or Sidecar for real-time event streaming via SSE (Server-Sent Events).
​

Key Features
Real-time listening to Casper event streams with automatic reconnection.

Parsing of marketplace-specific events (e.g., List, Buy, Offer, RoyaltyPaid) per Casper Event Standard.
​

Output in structured JSON format for downstream consumers (HTTP, Kafka, PostgreSQL).

Configurable via environment variables or TOML for contract addresses, RPC endpoints, and filters.

High performance with async Rust (Tokio) and zero-copy parsing where possible.

Prerequisites
Rust 1.75+ (stable channel).

Casper Network RPC endpoint (e.g., mainnet/testnet node or Sidecar).
​

Optional: Docker for containerized deployment.

Quick Start
Clone the repo:

git clone https://github.com/midware/casper-marketplace-event-parser
cd casper-marketplace-event-parser
Install dependencies:

cargo build --release
Configure (create .env or use env vars):

CASPER_RPC_URL=https://rpc.mainnet.casper.network
MARKETPLACE_CONTRACT_HASH=your-contract-hash-here
OUTPUT_WEBHOOK_URL=http://your-backend/webhook  # Optional
Run the service:

cargo run --release
Or via Docker:

docker build -t casper-event-parser .
docker run -e CASPER_RPC_URL=... casper-event-parser
Configuration
Parameter	Description	Default	Example
CASPER_RPC_URL	Casper node or Sidecar events endpoint 
​	-	https://sidecar.mainnet.casper.network/events
MARKETPLACE_CONTRACT_HASH	Hash of the marketplace contract 
​	-	marketplace-1234...
FILTER_EVENTS	Comma-separated event names to parse	all	List,Buy,RoyaltyPaid
LOG_LEVEL	Logging verbosity	info	debug
Full config in config.toml or env vars.

Supported Events
List: NFT listed for sale.

Buy: NFT purchased (incl. royalties).

Cancel: Listing cancelled.

Offer: Bid placed.

RoyaltyPaid: Royalty distribution to creators.
​

Example parsed output:

json
{
  "event_type": "Buy",
  "timestamp": 1736784000,
  "nft_token_id": "nft-123",
  "buyer": "account-hash-abc...",
  "seller": "account-hash-def...",
  "price": "1000000000000000000",  // 1 CSPR
  "royalties": [{"creator": "...", "amount": "50000000000000000"}]  // 5%
}
Deployment
Docker: Use provided Dockerfile.

Kubernetes/Helm: Scale with replicas for high-volume networks.

Monitoring: Exposes Prometheus metrics on /metrics.

Architecture

Casper Node/Sidecar (SSE Events)
          ↓
     Event Listener (Tokio)
          ↓
   Event Parser (Custom Schemas)
          ↓
Output Sink (HTTP/Kafka/DB)
Contributing
Fork, create a feature branch, and submit a PR. Focus on tests and benchmarks.

Authors:
Kamil Szymoniak & Damian Sarnecki

License
MIT License.
