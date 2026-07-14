#!/bin/bash

# Sirgriff Staking - Local Testing Script
# This script sets up a local Solana test environment and runs tests

set -e

echo "🧪 Starting local Solana validator..."

# Start local validator in background
solana-test-validator &
VALIDATOR_PID=$!

# Wait for validator to start
sleep 5

# Check if validator is running
if ! ps -p $VALIDATOR_PID > /dev/null; then
    echo "❌ Failed to start validator"
    exit 1
fi

echo "✓ Validator started (PID: $VALIDATOR_PID)"

# Set network to localhost
export SOLANA_RPC_URL="http://localhost:8899"

# Airdrop SOL to test account
echo "💰 Airdropping SOL to test account..."
solana airdrop 10 --url http://localhost:8899

# Build and deploy
echo "🔨 Building smart contract..."
cargo build-bpf

echo "📦 Deploying smart contract..."
SOLANA_RPC_URL=http://localhost:8899 solana program deploy \
  target/deploy/sirgriff_staking.so \
  --url http://localhost:8899

# Run tests
echo "🧪 Running tests..."
cargo test --lib

# Cleanup
echo "🧹 Cleaning up..."
kill $VALIDATOR_PID

echo "✅ All tests completed!"
