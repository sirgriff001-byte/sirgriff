# Sirgriff Token Staking - API Documentation

## Overview

This document provides comprehensive API documentation for the Sirgriff token staking smart contract on Solana.

## Table of Contents

1. [Program Instructions](#program-instructions)
2. [Account Structures](#account-structures)
3. [PDAs](#pdas)
4. [Error Codes](#error-codes)
5. [Examples](#examples)

---

## Program Instructions

### 0. Initialize

**Description:** Initialize a new staking pool with configuration parameters.

**Instruction Discriminator:** `0`

**Accounts:**
```
0. [signer, writable]   Initializer account (pool owner)
1. [writable]           Staking pool PDA
2. []                   Mint account for staking token
3. [writable]           Pool token account (holds staked tokens)
4. [writable]           Pool reward account (holds rewards)
5. []                   Rent sysvar
6. []                   System program
7. []                   Token program
```

**Parameters:**
```rust
reward_per_slot: u64   // Tokens rewarded per slot
lock_period: u64       // Minimum slots before unstaking
```

**Example:**
```typescript
const ix = client.createInitializeInstruction(
  initializer,
  mint,
  poolTokenAccount,
  poolRewardAccount,
  BigInt(1000),    // 1000 tokens per slot
  BigInt(100)      // 100 slot lock period
);
```

---

### 1. Stake

**Description:** Deposit tokens into the staking pool and begin earning rewards.

**Instruction Discriminator:** `1`

**Accounts:**
```
0. [signer]             User account
1. [writable]           User token account (ATA)
2. [writable]           Pool token account
3. [writable]           User stake record PDA
4. [writable]           Staking pool PDA
5. []                   Mint account
6. []                   System program
7. []                   Token program
```

**Parameters:**
```rust
amount: u64  // Amount of tokens to stake
```

**Example:**
```typescript
const ix = client.createStakeInstruction(
  user,
  userTokenAccount,
  poolTokenAccount,
  mint,
  BigInt(1000)  // Stake 1000 tokens
);
```

---

### 2. Unstake

**Description:** Withdraw staked tokens after lock period expires.

**Instruction Discriminator:** `2`

**Accounts:**
```
0. [signer]             User account
1. [writable]           User token account (ATA)
2. [writable]           Pool token account
3. [writable]           User stake record PDA
4. [writable]           Staking pool PDA
5. []                   Mint account
6. []                   Token program
```

**Parameters:**
```rust
amount: u64  // Amount of tokens to unstake
```

**Errors:**
- `LockPeriodNotExpired` - Lock period has not elapsed
- `InsufficientFunds` - Insufficient staked amount

---

### 3. Claim Rewards

**Description:** Claim accumulated staking rewards.

**Instruction Discriminator:** `3`

**Accounts:**
```
0. [signer]             User account
1. [writable]           User reward account (ATA)
2. [writable]           Pool reward account
3. [writable]           User stake record PDA
4. [writable]           Staking pool PDA
5. []                   Mint account
6. []                   Token program
```

**Calculation:**
```
Rewards = reward_per_slot × (current_slot - last_claim_slot) × staked_amount
```

---

### 4. Get Pool State

**Description:** Query current pool state and statistics.

**Instruction Discriminator:** `4`

**Accounts:**
```
0. []  Staking pool PDA
```

**Returns:**
```rust
pub struct StakingPool {
    pub is_initialized: bool,
    pub owner: [u8; 32],
    pub mint: [u8; 32],
    pub token_account: [u8; 32],
    pub reward_account: [u8; 32],
    pub reward_per_slot: u64,
    pub lock_period: u64,
    pub total_staked: u64,
    pub last_reward_slot: u64,
}
```

---

### 5. Get User Stake

**Description:** Query user's stake record.

**Instruction Discriminator:** `5`

**Accounts:**
```
0. []  User stake record PDA
```

**Returns:**
```rust
pub struct StakeRecord {
    pub is_initialized: bool,
    pub user: [u8; 32],
    pub staked_amount: u64,
    pub staked_at_slot: u64,
    pub last_claim_slot: u64,
    pub total_rewards_claimed: u64,
}
```

---

### 6. Withdraw Fees (Admin)

**Description:** Admin withdraws accumulated reward fees.

**Instruction Discriminator:** `6`

**Accounts:**
```
0. [signer]             Admin account (must be pool owner)
1. [writable]           Admin wallet (ATA)
2. [writable]           Pool reward account
3. [writable]           Staking pool PDA
4. []                   Mint account
5. []                   Token program
```

---

### 7. Update Reward Rate (Admin)

**Description:** Modify the reward rate per slot.

**Instruction Discriminator:** `7`

**Accounts:**
```
0. [signer]             Admin account (must be pool owner)
1. [writable]           Staking pool PDA
```

**Parameters:**
```rust
new_reward_per_slot: u64
```

---

### 8. Update Lock Period (Admin)

**Description:** Modify the lock period requirement.

**Instruction Discriminator:** `8`

**Accounts:**
```
0. [signer]             Admin account (must be pool owner)
1. [writable]           Staking pool PDA
```

**Parameters:**
```rust
new_lock_period: u64
```

---

## Account Structures

### StakingPool (161 bytes)

```
Offset  Type        Field
------  ----        -----
0       bool        is_initialized
1-32    [u8; 32]    owner (Pubkey)
33-64   [u8; 32]    mint (Pubkey)
65-96   [u8; 32]    token_account (Pubkey)
97-128  [u8; 32]    reward_account (Pubkey)
129-136 u64         reward_per_slot
137-144 u64         lock_period
145-152 u64         total_staked
153-160 u64         last_reward_slot
```

### StakeRecord (65 bytes)

```
Offset  Type        Field
------  ----        -----
0       bool        is_initialized
1-32    [u8; 32]    user (Pubkey)
33-40   u64         staked_amount
41-48   u64         staked_at_slot
49-56   u64         last_claim_slot
57-64   u64         total_rewards_claimed
```

---

## PDAs

### Staking Pool

```
seeds = [b"staking_pool", mint.key()]
program_id = SIRGRIFF_PROGRAM_ID
```

### Stake Record

```
seeds = [b"stake_record", user.key(), mint.key()]
program_id = SIRGRIFF_PROGRAM_ID
```

### Pool Token Account

```
seeds = [b"pool_tokens", mint.key()]
program_id = SIRGRIFF_PROGRAM_ID
```

---

## Error Codes

```rust
pub enum StakingError {
    InvalidInstruction = 0,
    NotRentExempt = 1,
    InvalidOwner = 2,
    InvalidMint = 3,
    InvalidTokenAccount = 4,
    InsufficientFunds = 5,
    InvalidStakeAmount = 6,
    LockPeriodNotExpired = 7,
    NoStakedTokens = 8,
    CalculationOverflow = 9,
}
```

---

## Examples

### Complete Staking Workflow

```typescript
import { StakingClient } from 'sirgriff-staking-client';
import { Keypair, PublicKey } from '@solana/web3.js';

const client = new StakingClient('https://api.devnet.solana.com');
const user = Keypair.generate();
const mint = new PublicKey('...');

// 1. Stake tokens
const stakeIx = client.createStakeInstruction(
  user.publicKey,
  userTokenAccount,
  poolTokenAccount,
  mint,
  BigInt(1000)
);

// 2. Get user stake info
const stake = await client.getUserStake(user.publicKey, mint);
console.log('Staked amount:', stake.staked_amount);

// 3. Claim rewards (after some time)
const claimIx = client.createClaimRewardsInstruction(
  user.publicKey,
  userRewardAccount,
  poolRewardAccount,
  mint
);

// 4. Unstake (after lock period)
const unstakeIx = client.createUnstakeInstruction(
  user.publicKey,
  userTokenAccount,
  poolTokenAccount,
  mint,
  BigInt(500)
);
```

---

## Rate Limiting

No rate limiting is enforced at the contract level. However, Solana RPC endpoints may have rate limits.

## Best Practices

1. **Always validate addresses** before sending transactions
2. **Use ProgramError handlers** to catch and display errors
3. **Keep lock periods reasonable** to balance security and usability
4. **Monitor reward rates** to ensure profitability
5. **Use PDAs for account derivation** to avoid mistakes

---

## Support

For issues or questions:
- 📧 Email: support@sirgriff.dev
- 🐛 GitHub Issues: https://github.com/sirgriff001-byte/sirgriff/issues
- 💬 Discord: [Join our Discord](https://discord.gg/sirgriff)
