# Blockchain API Testing Guide

This guide provides instructions for testing all available API endpoints of the Rust Blockchain application.

## Prerequisites

- Server running on `http://127.0.0.1:6969` (or your configured HOST:PORT)
- `curl` command-line tool installed
- Optional: `jq` for pretty JSON formatting

## Starting the Server

```bash
cargo run
```

The server will start with the configuration from your `.env` file:
- Default HOST: `127.0.0.1`
- Default PORT: `6969`
- Default DIFFICULTY: `3`

---

## API Endpoints

### 1. Health Check

Check if the server is running and healthy.

**Endpoint:** `GET /health`

```bash
curl -s http://127.0.0.1:6969/health | jq
```

**Expected Response:**
```json
{
  "status": "healthy",
  "service": "blockchain-api"
}
```

---

### 2. Get Blockchain

Retrieve the entire blockchain with all blocks.

**Endpoint:** `GET /chain`

```bash
curl -s http://127.0.0.1:6969/chain | jq
```

**Expected Response:**
```json
{
  "blocks": [
    {
      "transaction": [],
      "prev_hash": "0",
      "current_hash": "7c890dd81e07da6bb2273f3cda4e97286f69321b8a94c2968d66080246d2bb09",
      "timestamp": 1762374603,
      "nonce": 0
    }
  ],
  "difficulty": 3
}
```

---

### 3. Add Block

Add a new block to the blockchain with transactions.

**Endpoint:** `POST /add_block`

**Single Transaction:**
```bash
curl -s -X POST http://127.0.0.1:6969/add_block \
  -H 'Content-Type: application/json' \
  -d '[{"amount": 100.0, "from": "Alice", "to": "Bob"}]' | jq
```

**Multiple Transactions:**
```bash
curl -s -X POST http://127.0.0.1:6969/add_block \
  -H 'Content-Type: application/json' \
  -d '[
    {"amount": 100.0, "from": "Alice", "to": "Bob"},
    {"amount": 50.5, "from": "Bob", "to": "Charlie"},
    {"amount": 25.75, "from": "Charlie", "to": "David"}
  ]' | jq
```

**Expected Response:**
```json
{
  "message": "Block added successfully",
  "block_height": 1,
  "current_difficulty": 3
}
```

**Error Response (Invalid Transaction):**
```bash
curl -s -X POST http://127.0.0.1:6969/add_block \
  -H 'Content-Type: application/json' \
  -d '[{"amount": -10.0, "from": "Alice", "to": "Bob"}]' | jq
```

```json
{
  "error": "Invalid transaction",
  "index": 0,
  "details": "Amount must be positive and from/to addresses cannot be empty"
}
```

---

### 4. Validate Blockchain

Verify the integrity of the entire blockchain.

**Endpoint:** `GET /validate`

```bash
curl -s http://127.0.0.1:6969/validate | jq
```

**Expected Response (Valid Chain):**
```json
{
  "valid": true,
  "message": "Blockchain is valid and integrity verified",
  "chain_length": 2,
  "difficulty": 3
}
```

**Expected Response (Invalid Chain):**
```json
{
  "valid": false,
  "message": "Blockchain validation failed chain may be corrupted",
  "chain_length": 2,
  "difficulty": 3
}
```

---

### 5. Get Latest Block

Retrieve the most recently added block.

**Endpoint:** `GET /latest_block`

```bash
curl -s http://127.0.0.1:6969/latest_block | jq
```

**Expected Response:**
```json
{
  "block": {
    "transaction": [
      {
        "amount": 100.0,
        "from": "Alice",
        "to": "Bob"
      }
    ],
    "prev_hash": "7c890dd81e07da6bb2273f3cda4e97286f69321b8a94c2968d66080246d2bb09",
    "current_hash": "000a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6",
    "timestamp": 1762374650,
    "nonce": 1234
  },
  "block_number": 1,
  "current_difficulty": 3
}
```

---

### 6. Get Difficulty

Get the current mining difficulty level.

**Endpoint:** `GET /get_difficulty`

```bash
curl -s http://127.0.0.1:6969/get_difficulty | jq
```

**Expected Response:**
```json
{
  "current_difficulty": 3,
  "total_blocks": 2
}
```

---

### 7. Update Difficulty

Change the mining difficulty for future blocks.

**Endpoint:** `PUT /update_difficulty`

```bash
curl -s -X PUT http://127.0.0.1:6969/update_difficulty \
  -H 'Content-Type: application/json' \
  -d '{"difficulty": 4}' | jq
```

**Expected Response:**
```json
{
  "message": "Difficulty updated successfully",
  "old_difficulty": 3,
  "new_difficulty": 4
}
```

**Error Response (Invalid Difficulty):**
```bash
curl -s -X PUT http://127.0.0.1:6969/update_difficulty \
  -H 'Content-Type: application/json' \
  -d '{"difficulty": 0}' | jq
```

```json
{
  "error": "Difficulty must be between 1 and 10"
}
```

---

## Complete Testing Script

Here's a complete bash script to test all endpoints in sequence:

```bash
#!/bin/bash

BASE_URL="http://127.0.0.1:6969"

echo "=== Testing Blockchain API ==="
echo

echo "1. Health Check"
curl -s $BASE_URL/health | jq
echo

echo "2. Get Initial Chain"
curl -s $BASE_URL/chain | jq
echo

echo "3. Add First Block"
curl -s -X POST $BASE_URL/add_block \
  -H 'Content-Type: application/json' \
  -d '[{"amount": 100.0, "from": "Alice", "to": "Bob"}]' | jq
echo

echo "4. Add Second Block with Multiple Transactions"
curl -s -X POST $BASE_URL/add_block \
  -H 'Content-Type: application/json' \
  -d '[
    {"amount": 50.0, "from": "Bob", "to": "Charlie"},
    {"amount": 25.0, "from": "Charlie", "to": "David"}
  ]' | jq
echo

echo "5. Get Latest Block"
curl -s $BASE_URL/latest_block | jq
echo

echo "6. Validate Blockchain"
curl -s $BASE_URL/validate | jq
echo

echo "7. Get Current Difficulty"
curl -s $BASE_URL/get_difficulty | jq
echo

echo "8. Update Difficulty to 4"
curl -s -X PUT $BASE_URL/update_difficulty \
  -H 'Content-Type: application/json' \
  -d '{"difficulty": 4}' | jq
echo

echo "9. Get Full Chain"
curl -s $BASE_URL/chain | jq
echo

echo "=== Testing Complete ==="
```

Save this script as `test_api.sh`, make it executable, and run it:

```bash
chmod +x test_api.sh
./test_api.sh
```

---

## Testing with Postman

You can also import these endpoints into Postman:

1. Create a new collection named "Rust Blockchain API"
2. Add requests for each endpoint listed above
3. Set the base URL to `http://127.0.0.1:6969`
4. For POST/PUT requests, set Content-Type to `application/json`

---

## Environment Variables

You can customize the server configuration in `.env`:

```env
DIFFICULTY=3
PORT=6969
HOST=127.0.0.1
BLOCKCHAIN_FILE=blockchain.json
```

After modifying `.env`, restart the server for changes to take effect.

---

## Notes

- Mining time increases exponentially with difficulty level
- Difficulty must be between 1 and 10
- Transaction amounts must be positive
- From/To addresses cannot be empty
- The blockchain is automatically saved to `blockchain.json` after each block addition
- Existing blockchain is loaded on server startup if available

---

## Troubleshooting

**Server won't start:**
- Check if port is already in use: `lsof -i :6969`
- Verify HOST address is valid (use `127.0.0.1` or `0.0.0.0`)

**Can't connect to API:**
- Ensure server is running: `cargo run`
- Check firewall settings
- Verify correct HOST and PORT in requests

**Blocks not being added:**
- Check transaction format (valid JSON)
- Ensure positive amounts
- Verify non-empty from/to addresses
- Higher difficulty = longer mining time
