# 🚀 Sirgriff Token Staking - Complete Project

> A production-ready Solana token staking smart contract with full-stack development tools.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-1.18+-9945FF)](https://solana.com)
[![Rust](https://img.shields.io/badge/Rust-1.70+-CE422B)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-3178C6)](https://www.typescriptlang.org)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [Components](#components)
- [Documentation](#documentation)
- [Development](#development)
- [Deployment](#deployment)
- [Security](#security)
- [Contributing](#contributing)

---

## 🎯 Overview

Sirgriff is a decentralized staking protocol on Solana that allows users to:
- **Stake tokens** and earn automated rewards
- **Lock tokens** with customizable lock periods
- **Claim rewards** based on staking duration and amount
- **Manage stakes** through multiple interfaces (web, CLI, SDK)

### Key Statistics

- ✅ **8 Contract Instructions** for full staking lifecycle
- ✅ **3 Account Types** for secure state management
- ✅ **5 Admin Functions** for pool management
- ✅ **Zero Critical Issues** (Security Audit: PASSED)
- ✅ **Full TypeScript Support** with client SDK
- ✅ **Multi-platform** (Web UI, CLI, Programmatic)

---

## ⭐ Features

### Smart Contract

- 🔐 **Secure PDA-based Architecture**
  - Program Derived Accounts for state management
  - Cryptographically verified ownership
  - No private key vulnerabilities

- 📊 **Flexible Reward System**
  - Configurable rewards per slot
  - Proportional to staked amount
  - Claim anytime after staking

- ⏱️ **Lock Period Protection**
  - Prevent early unstaking
  - Configurable lock duration
  - Admin adjustable

- 👑 **Admin Controls**
  - Update reward rates
  - Modify lock periods
  - Withdraw accumulated fees
  - Owner-only operations

### Developer Tools

- 📚 **TypeScript Client SDK**
  - Full type safety
  - Instruction builders
  - PDA utilities
  - Account queries

- 🖥️ **React Web UI**
  - Connect wallet functionality
  - Real-time pool statistics
  - User stake tracking
  - Transaction interface

- 🛠️ **CLI Tool**
  - Command-line staking
  - Multi-network support
  - Batch operations
  - Easy integration

- 📋 **Deployment Scripts**
  - One-command deployment
  - Multi-network support (devnet, testnet, mainnet)
  - Automated testing
  - Local validator setup

### Documentation

- 📖 **API Documentation** - Complete instruction reference
- 🔒 **Security Audit** - Full vulnerability assessment
- 📝 **Code Comments** - Inline documentation
- 🎓 **Examples** - Complete usage patterns

---

## 📁 Project Structure

```
sirgriff/
├── src/                          # Smart Contract (Rust)
│   ├── lib.rs                   # Entry point & routing
│   ├── processor.rs             # Core business logic
│   ├── instruction.rs           # Instruction definitions
│   ├── state.rs                 # Account structures
│   ├── admin.rs                 # Admin operations
│   ├── utils.rs                 # Utility functions
│   ├── query.rs                 # State queries
│   ├── token_helper.rs          # Token utilities
│   └── error.rs                 # Error types
│
├── client/                       # TypeScript Client SDK
│   ├── src/
│   │   ├── index.ts            # Main StakingClient class
│   │   ├── constants.ts        # Program constants
│   │   └── types.ts            # Type exports
│   ├── tests/
│   ├── package.json
│   └── tsconfig.json
│
├── ui/                          # React Web Frontend
│   ├── src/
│   │   ├── App.tsx             # Main React component
│   │   └── App.css             # Styling
│   ├── public/
│   └── package.json
│
├── cli/                         # Command-Line Tool
│   ├── src/
│   │   └── index.ts            # CLI commands
│   ├── README.md               # CLI documentation
│   └── package.json
│
├── tests/                       # Integration Tests
│   └── integration_tests.rs
│
├── scripts/                     # Deployment Scripts
│   ├── deploy.sh               # Network deployment
│   ├── test-local.sh           # Local testing
│   └── README.md
│
├── docs/                        # Documentation
│   ├── API.md                  # API Reference
│   └── SECURITY_AUDIT.md       # Security Report
│
├── Cargo.toml                   # Rust dependencies
├── Cargo.lock
├── PROGRESS.md                  # Development progress
└── README.md                    # This file
```

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"

# Install Node.js 16+
node --version  # Should be v16 or higher
```

### 1. Build Smart Contract

```bash
# Build for Solana
cargo build-bpf

# Output: target/deploy/sirgriff_staking.so
```

### 2. Run Local Tests

```bash
# Start validator and run tests
./scripts/test-local.sh

# Or run tests manually
cargo test --lib
```

### 3. Deploy to Devnet

```bash
# Deploy smart contract
./scripts/deploy.sh devnet

# Update PROGRAM_ID in client/src/constants.ts

# Build client SDK
cd client && npm install && npm build && cd ..
```

### 4. Start Web UI

```bash
cd ui
npm install
npm start

# Opens at http://localhost:3000
```

### 5. Use CLI Tool

```bash
cd cli
npm install
npm build

# Stake tokens
npm start stake 100 --mint <MINT_ADDRESS>

# Check balance
npm start balance --user <USER_ADDRESS> --mint <MINT_ADDRESS>

# Claim rewards
npm start claim-rewards --mint <MINT_ADDRESS>
```

---

## 🔧 Components

### Smart Contract (`src/`)

**Core Instructions:**
1. `Initialize` - Set up staking pool
2. `Stake` - Deposit tokens
3. `Unstake` - Withdraw tokens
4. `ClaimRewards` - Collect rewards
5. `GetPoolState` - Query pool info
6. `GetUserStake` - Query user stake
7. `WithdrawFees` - Admin fee withdrawal
8. `UpdateRewardRate` - Admin: adjust rewards
9. `UpdateLockPeriod` - Admin: adjust lock time

**Account Structures:**
- `StakingPool` (161 bytes) - Pool configuration and state
- `StakeRecord` (65 bytes) - Per-user staking record

### TypeScript Client (`client/`)

```typescript
import { StakingClient } from 'sirgriff-staking-client';
import { PublicKey } from '@solana/web3.js';

const client = new StakingClient('https://api.devnet.solana.com');

// Create instruction
const ix = client.createStakeInstruction(
  user,
  userTokenAccount,
  poolTokenAccount,
  mint,
  BigInt(1000)
);

// Query pool
const pool = await client.getPoolInfo(mint);
console.log(pool);
```

### React UI (`ui/`)

- 🎨 Modern responsive design
- 💼 Wallet integration (Phantom, Solflare, etc.)
- 📊 Real-time statistics
- 🔄 Transaction management
- 🎯 User stake tracking

### CLI Tool (`cli/`)

```bash
# Stake
sirgriff stake 100 --mint <ADDRESS>

# Unstake
sirgriff unstake 50 --mint <ADDRESS>

# Claim
sirgriff claim-rewards --mint <ADDRESS>

# Info
sirgriff info --mint <ADDRESS>

# Balance
sirgriff balance --user <ADDRESS> --mint <ADDRESS>
```

---

## 📚 Documentation

### API Reference
See [docs/API.md](docs/API.md) for complete instruction reference including:
- Account structures
- Parameter details
- Error codes
- Usage examples

### Security Audit
See [docs/SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md):
- ✅ **LOW** risk assessment
- Zero critical issues
- Comprehensive vulnerability analysis
- Recommendations for mainnet

### Deployment Scripts
See [scripts/README.md](scripts/README.md):
- Multi-network deployment
- Local testing setup
- Automated builds

---

## 💻 Development

### Smart Contract Development

```bash
# Build
cargo build-bpf

# Test
cargo test --lib

# Build with optimizations
cargo build-bpf --release
```

### Client SDK Development

```bash
cd client
npm install
npm run build
npm test
```

### UI Development

```bash
cd ui
npm install
npm start
```

### CLI Development

```bash
cd cli
npm install
npm run build
npm run dev  # Watch mode
```

---

## 🚢 Deployment

### Devnet

```bash
./scripts/deploy.sh devnet
```

### Testnet

```bash
./scripts/deploy.sh testnet
```

### Mainnet (⚠️ Requires Audit)

```bash
# After security audit and testing
./scripts/deploy.sh mainnet
```

### Local

```bash
./scripts/test-local.sh
```

---

## 🔒 Security

### Security Features

- ✅ **Checked Arithmetic** - No overflow/underflow
- ✅ **PDA-based Authorization** - Cryptographic security
- ✅ **Account Validation** - All accounts verified
- ✅ **Signer Verification** - Proper signatures required
- ✅ **Rent Exemption** - Account sustainability
- ✅ **Access Control** - Owner-only admin functions

### Audit Status

- 📋 **Status:** PASSED ✅
- 🎯 **Risk Level:** LOW
- 🔍 **Critical Issues:** 0
- ⚠️ **Medium Issues:** 2 (non-blocking)
- 📝 **Recommendations:** Documented in audit report

See [SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md) for full details.

---

## 🤝 Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Solana best practices
- Add tests for new functionality
- Update documentation
- Ensure all tests pass

---

## 📦 Releases

- **v1.0.0** - Initial release with core functionality
  - Smart contract with 8 instructions
  - TypeScript client SDK
  - React web UI
  - CLI tool
  - Full documentation
  - Security audit (PASSED)

---

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

## 🎓 Learning Resources

- [Solana Documentation](https://docs.solana.com)
- [Anchor Framework](https://www.anchor-lang.com)
- [SPL Token Program](https://spl.solana.com/token)
- [Solana Cookbook](https://solanacookbook.com)

---

## 📞 Support

- 📖 [Documentation](./docs)
- 🐛 [Report Issues](https://github.com/sirgriff001-byte/sirgriff/issues)
- 💬 [Discussions](https://github.com/sirgriff001-byte/sirgriff/discussions)

---

## 🙏 Acknowledgments

- Built with [Solana CLI](https://github.com/solana-labs/solana)
- Powered by [SPL Token Program](https://github.com/solana-labs/solana-program-library)
- Inspired by the Solana community

---

**Made with ❤️ by the Sirgriff Team**

**Status:** ✅ Production Ready for Devnet  
**Last Updated:** July 14, 2026
