#!/bin/bash

# Make scripts executable
chmod +x scripts/deploy.sh
chmod +x scripts/test-local.sh

echo "✓ Deploy script: ./scripts/deploy.sh [network]"
echo "✓ Local test script: ./scripts/test-local.sh"
echo ""
echo "Usage:"
echo "  ./scripts/deploy.sh devnet        # Deploy to devnet"
echo "  ./scripts/deploy.sh testnet       # Deploy to testnet"
echo "  ./scripts/deploy.sh mainnet       # Deploy to mainnet"
echo "  ./scripts/test-local.sh           # Run local tests"
