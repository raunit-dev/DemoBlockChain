# DemoBlockChain

A simple blockchain implementation built with Rust and Actix-web, demonstrating core blockchain concepts including block creation, proof-of-work mining, and a RESTful API interface.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [How It Works](#how-it-works)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## 🔍 Overview

DemoBlockChain is an educational blockchain implementation that showcases the fundamental principles of blockchain technology. Built with Rust for performance and safety, it provides a REST API for interacting with the blockchain, allowing users to add transactions, mine blocks, and view the chain state.

## ✨ Features

- **Block Creation**: Create blocks with transaction data
- **Proof of Work**: Implements mining algorithm with adjustable difficulty
- **SHA-256 Hashing**: Cryptographic hashing for block integrity
- **RESTful API**: Easy-to-use HTTP endpoints for blockchain interaction
- **CORS Support**: Cross-origin resource sharing enabled
- **Logging**: Comprehensive logging for debugging and monitoring
- **Transaction Management**: Add and manage transactions before mining
- **Chain Validation**: Ensures blockchain integrity

## 🛠 Technology Stack

- **Language**: Rust (2024 edition)
- **Web Framework**: Actix-web 4.11.0
- **Hashing**: SHA-2 (sha2 crate)
- **Serialization**: Serde & Serde JSON
- **CORS**: Actix-cors 0.7.1
- **Logging**: env_logger 0.11.8 & log 0.4.28
- **Time Management**: Chrono 0.4.42
- **Environment Variables**: dotenv 0.15.0

## 📁 Project Structure

```
DemoBlockChain/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Rust dependencies and project metadata
├── Cargo.lock           # Locked dependency versions
├── .env.example         # Example environment configuration
├── .gitignore          # Git ignore rules
└── README.md           # Project documentation
```

## 📋 Prerequisites

Before running this project, ensure you have the following installed:

- **Rust**: Version 1.70 or higher
  - Install from [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes with Rust installation
- **Git**: For cloning the repository

## 🚀 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/raunit-dev/DemoBlockChain.git
   cd DemoBlockChain
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

## ⚙️ Configuration

Create a `.env` file based on `.env.example`:

```env
# Server configuration
RUST_LOG=info
PORT=8080
```

You can adjust the following settings:
- `RUST_LOG`: Logging level (trace, debug, info, warn, error)
- `PORT`: Server port (default: 8080)

## 💻 Usage

### Running the Application

1. **Development mode**
   ```bash
   cargo run
   ```

2. **Production build**
   ```bash
   cargo build --release
   ./target/release/rust-back
   ```

3. **With logging**
   ```bash
   RUST_LOG=debug cargo run
   ```

The server will start on `http://localhost:8080` (or your configured port).

## 🌐 API Endpoints

### 1. Get Blockchain
```http
GET /blockchain
```
Returns the entire blockchain with all blocks.

**Response:**
```json
{
  "chain": [...],
  "length": 3
}
```

### 2. Add Transaction
```http
POST /transaction
Content-Type: application/json
```

**Request Body:**
```json
{
  "sender": "Alice",
  "recipient": "Bob",
  "amount": 50
}
```

**Response:**
```json
{
  "message": "Transaction added successfully"
}
```

### 3. Mine Block
```http
POST /mine
```
Mines a new block with pending transactions.

**Response:**
```json
{
  "message": "Block mined successfully",
  "block": {...}
}
```

### 4. Get Latest Block
```http
GET /latest
```
Returns the most recent block in the chain.

### 5. Validate Chain
```http
GET /validate
```
Checks if the blockchain is valid.

**Response:**
```json
{
  "valid": true
}
```

## 🔧 How It Works

### Block Structure
Each block contains:
- **Index**: Position in the blockchain
- **Timestamp**: When the block was created
- **Transactions**: List of transactions in the block
- **Proof**: Nonce value from proof-of-work
- **Previous Hash**: Hash of the previous block
- **Hash**: Current block's hash

### Proof of Work
The mining algorithm:
1. Starts with a nonce of 0
2. Calculates hash of block data + nonce
3. Checks if hash meets difficulty requirement (leading zeros)
4. Increments nonce and repeats until valid hash found

### Chain Validation
Validates the blockchain by:
1. Checking each block's hash is correct
2. Verifying each block's previous_hash matches the previous block
3. Ensuring proof-of-work is valid for each block

## 👨‍💻 Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Watch Mode (with cargo-watch)
```bash
cargo install cargo-watch
cargo watch -x run
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available for educational purposes.

## 👤 Author

**raunit-dev**
- GitHub: [@raunit-dev](https://github.com/raunit-dev)

## 🙏 Acknowledgments

- Built as a learning project to understand blockchain fundamentals
- Inspired by the original Bitcoin whitepaper
- Uses Rust for memory safety and performance

---

⭐ Star this repository if you found it helpful!
