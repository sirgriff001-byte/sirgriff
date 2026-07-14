# Sirgriff Token Staking - CLI Documentation

## Installation

```bash
npm install -g sirgriff-staking-cli
```

## Usage

### Stake Tokens
```bash
sirgriff stake 100 \
  --endpoint https://api.devnet.solana.com \
  --keypair ~/.config/solana/id.json \
  --mint <TOKEN_MINT_ADDRESS>
```

### Unstake Tokens
```bash
sirgriff unstake 50 \
  --endpoint https://api.devnet.solana.com \
  --keypair ~/.config/solana/id.json \
  --mint <TOKEN_MINT_ADDRESS>
```

### Claim Rewards
```bash
sirgriff claim-rewards \
  --endpoint https://api.devnet.solana.com \
  --keypair ~/.config/solana/id.json \
  --mint <TOKEN_MINT_ADDRESS>
```

### Get Pool Information
```bash
sirgriff info \
  --endpoint https://api.devnet.solana.com \
  --mint <TOKEN_MINT_ADDRESS>
```

### Get User Balance
```bash
sirgriff balance \
  --endpoint https://api.devnet.solana.com \
  --user <USER_WALLET_ADDRESS> \
  --mint <TOKEN_MINT_ADDRESS>
```

## Options

- `-e, --endpoint <url>` - Solana RPC endpoint (default: devnet)
- `-k, --keypair <path>` - Path to keypair file
- `-m, --mint <address>` - Token mint address
- `-u, --user <address>` - User wallet address

## Examples

```bash
# Stake 100 tokens on devnet
sirgriff stake 100 --mint 4zMMC9srt5Ri5X14GAgipwWeT41MMCjBTYSnooEwVjY

# Check your balance
sirgriff balance --user YOUR_WALLET_ADDRESS --mint 4zMMC9srt5Ri5X14GAgipwWeT41MMCjBTYSnooEwVjY

# Claim rewards
sirgriff claim-rewards --mint 4zMMC9srt5Ri5X14GAgipwWeT41MMCjBTYSnooEwVjY
```
