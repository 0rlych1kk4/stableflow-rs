![Rust](https://img.shields.io/badge/Rust-API-orange)

# stableflow-rs

Rust SDK for programmable stablecoin wallets using Circle Dev-Controlled Wallets.

This project explores building a Rust-native infrastructure layer for:
- Stablecoin payments
- Wallet orchestration
- Treasury automation
- Cross-chain programmable finance

Currently implemented:

- Circle API client
- Wallet set operations
- Wallet creation flow (in progress)
- Entity Secret management module
- Transaction request scaffolding

## Architecture

stableflow-rs is organized as a modular Rust SDK.

---
## Quick Start

```bash
cargo run --example create_wallet
```
---
**Environment variables required:**

- CIRCLE_API_KEY=your_api_key
- CIRCLE_ENTITY_SECRET=your_entity_secret
- CIRCLE_WALLET_SET_ID=your_wallet_set_id
---

## Status
Early development.
The current goal is to implement full support for:
- Dev-controlled wallets
- Transaction creation
- Token transfers
- Stablecoin payment flows
---

## Roadmap
- Native Rust wallet creation
- Transaction submission
- Token balance queries
- Stablecoin payment flows
- Async streaming for transaction monitoring
---

## License
Licensed under either of:

- MIT License
- Apache License, Version 2.0

at your option.
---
