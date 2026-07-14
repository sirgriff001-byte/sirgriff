#!/bin/bash

# Sirgriff Staking - Deployment Script
# This script deploys the smart contract to the Solana network

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 Sirgriff Token Staking - Deployment Script${NC}"
echo ""

# Check if network argument provided
if [ -z "$1" ]; then
  NETWORK="devnet"
else
  NETWORK=$1
fi

echo -e "${YELLOW}Network: ${GREEN}$NETWORK${NC}"

# Set RPC endpoint based on network
case $NETWORK in
  mainnet)
    RPC_ENDPOINT="https://api.mainnet-beta.solana.com"
    ;;
  testnet)
    RPC_ENDPOINT="https://api.testnet.solana.com"
    ;;
  devnet)
    RPC_ENDPOINT="https://api.devnet.solana.com"
    ;;
  localhost)
    RPC_ENDPOINT="http://localhost:8899"
    ;;
  *)
    echo -e "${RED}Invalid network: $NETWORK${NC}"
    echo "Usage: ./deploy.sh [mainnet|testnet|devnet|localhost]"
    exit 1
    ;;
esac

echo -e "${YELLOW}RPC Endpoint: ${GREEN}$RPC_ENDPOINT${NC}"

# Build the program
echo ""
echo -e "${YELLOW}Building smart contract...${NC}"
cargo build-bpf --manifest-path=Cargo.toml

if [ $? -eq 0 ]; then
  echo -e "${GREEN}✓ Build successful${NC}"
else
  echo -e "${RED}✗ Build failed${NC}"
  exit 1
fi

# Deploy the program
echo ""
echo -e "${YELLOW}Deploying to $NETWORK...${NC}"

SOLANA_RPC_URL=$RPC_ENDPOINT solana program deploy \
  --network $NETWORK \
  --url $RPC_ENDPOINT \
  target/deploy/sirgriff_staking.so

if [ $? -eq 0 ]; then
  echo -e "${GREEN}✓ Deployment successful${NC}"
else
  echo -e "${RED}✗ Deployment failed${NC}"
  exit 1
fi

echo ""
echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Update PROGRAM_ID in client/src/constants.ts"
echo "2. Run integration tests"
echo "3. Deploy frontend"
