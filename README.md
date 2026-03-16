# Stellar Soroban Todo List Contract

This project is a simple Todo List smart contract written in Rust for the Stellar Soroban platform. It demonstrates how to build, test, and deploy a contract to the Stellar testnet.
https://stellar.expert/explorer/testnet/tx/f4b78c8c371885df442572f1b9548a8b9b6ec84b95bc198b960aae970a2d74c5

## Features
- Add todo items
- Remove todo items by index
- Update todo items by index
- Retrieve all todo items

## Directory Structure
```
todo/
├── contracts/
│   └── hello-world/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── target/
│   └── wasm32-unknown-unknown/
│       └── release/
│           └── hello_world.wasm
```

## Prerequisites
- Rust toolchain (`cargo`)
- Soroban CLI (`cargo install --locked soroban-cli`)
- A funded Stellar testnet account (see below)

## Build the Contract
From the contract directory:
```sh
cargo build --target wasm32-unknown-unknown --release
```
The compiled WASM will be at:
```
target/wasm32-unknown-unknown/release/hello_world.wasm
```

## Deploy to Stellar Testnet
1. **Generate a testnet identity (if needed):**
   ```sh
   soroban config identity generate alice
   ```
2. **Fund your testnet account:**
   - Get your address:
     ```sh
     soroban config identity address alice
     ```
   - Visit https://friendbot.stellar.org/?addr=YOUR_ADDRESS to fund it.
3. **Deploy the contract:**
   ```sh
   soroban contract deploy \
     --wasm target/wasm32v1-none/release/hello_world.wasm \
     --source-account alice \
     --network testnet \
     --alias hello_world
   ```

## Deployed Contract ID
```
hello_world
```

## Testing
Run unit tests with:
```sh
cargo test
```

## References
- [Stellar Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts/)
- [Deploy to Testnet Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/deploy-to-testnet)

---

*This project is for educational purposes and demonstrates basic Soroban contract development and deployment.*
