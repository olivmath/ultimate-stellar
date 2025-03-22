# Ultimate Stellar - People CRUD Contract

This project implements a CRUD (Create, Read, Update, Delete) contract for managing People records on the Stellar blockchain using Soroban smart contracts.

## Prerequisites

- Rust (latest stable version)
- Soroban CLI
- Stellar Development Environment
- Git

## Installation Requirements

1. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Stellar CLI:

```bash
cargo install --locked stellar-cli --features opt
```

3. Install Wasm Target:

```bash
rustup target add wasm32-unknown-unknown
```

4. Autocompletion

```bash
source <(stellar completion --shell zsh) # bash
echo "source <(stellar completion --shell zsh)" >> ~/.zshrc # bash
```

## Configure keys and Faucet

```bash
stellar keys generate --global alice --network testnet --fund
```

## Project Structure

5. Create a People Data Base Project

```
stellar contract init --name people-db .
```

6. Your project folders structures

```bash
ultimate-stellar/
├── Cargo.toml
├── README.md
└── contracts
    └── people-db
        ├── Cargo.toml
        ├── Makefile
        └── src
            ├── lib.rs
            └── test.rs

4 directories, 6 files
```

7. Build your Smartcontracts

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing the Project

Run the test suite:

```bash
cargo test
```

## Deploying the Contract

1. Start a local Stellar network:

```bash
soroban-cli local
```

2. Deploy the contract:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/people.wasm \
  --source YOUR_SECRET_KEY
```

## Contract Functions

The People contract provides the following functions:

- `create_person(name: String, age: i32) -> Address`: Creates a new person record
- `get_person(address: Address) -> Person`: Retrieves a person's information
- `update_person(address: Address, name: String, age: i32)`: Updates a person's information
- `delete_person(address: Address)`: Removes a person record

## Interacting with the Contract

1. Create a new person:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  -- create_person "John Doe" 30
```

2. Get person details:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  -- get_person PERSON_ADDRESS
```

3. Update person:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  -- update_person PERSON_ADDRESS "John Smith" 31
```

4. Delete person:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source YOUR_SECRET_KEY \
  -- delete_person PERSON_ADDRESS
```

## Development

To modify the contract:

1. Navigate to the contract directory:

```bash
cd contracts/people
```

2. Edit the source code in `src/lib.rs`

3. Rebuild and redeploy the contract following the steps above

## License

This project is licensed under the MIT License.
