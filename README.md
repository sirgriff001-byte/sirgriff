# Sirgriff Token Staking Smart Contract

A Solana-based smart contract for staking tokens and earning rewards.

## Overview

This is a decentralized staking program on the Solana blockchain that allows users to:
- Stake tokens to earn rewards
- Claim staking rewards based on time staked
- Unstake tokens with lock period validation
- Manage multiple user stake records

## Project Structure

```
src/
├── lib.rs           # Program entry point
├── instruction.rs   # Instruction definitions
├── processor.rs     # Instruction processing logic
├── state.rs         # Account state structures
└── error.rs         # Custom error types
```

## Key Features

### 1. **Initialize Pool**
Set up a staking pool with configurable parameters:
- `reward_per_slot`: Number of reward tokens per slot
- `lock_period`: Minimum time (in slots) tokens must be staked

### 2. **Stake Tokens**
Users can deposit tokens into the pool and start earning rewards
- Tracks staked amount
- Records staking start time
- Updates pool statistics

### 3. **Claim Rewards**
Users earn rewards based on:
- Time staked (slots elapsed)
- Amount staked
- Pool reward rate

### 4. **Unstake Tokens**
Withdraw staked tokens after meeting lock period requirements
- Validates lock period has expired
- Returns tokens to user
- Updates stake records

## Data Structures

### StakingPool
- Pool owner
- Associated token mint
- Total staked amount
- Reward configuration
- Reward distribution account

### StakeRecord
- User's staked amount
- Staking start time
- Last reward claim time
- Total rewards claimed

## Building

```bash
cargo build-bpf
```

## Deploying

```bash
solana program deploy target/deploy/sirgriff_staking.so
```

## Testing

```bash
cargo test
```

## Safety

- ✅ Input validation on all parameters
- ✅ Account ownership verification
- ✅ Rent exemption checks
- ✅ Overflow/underflow protection
- ✅ Signer verification on sensitive operations

## Future Enhancements

- [ ] Configurable staking tiers
- [ ] Early unstake penalties
- [ ] Governance token rewards
- [ ] Multiple reward token support
- [ ] Automated reward distribution
