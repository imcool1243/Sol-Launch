# Token Creation Guide

This guide explains how to create an SPL token on Solana that will be used with the Sol-Launch anti-sniper protection system.

## Overview

The Sol-Launch smart contract provides **anti-sniper trading protections** for token launches. Token creation is done using standard Solana SPL tools, then the smart contract is initialized to protect the trading phase.

## Prerequisites

- Solana CLI installed
- Phantom wallet installed
- SPL Token CLI installed
- Anchor CLI installed

## Step 1: Create Token Mint

### Create a new token mint:

```bash
# Create new token mint
spl-token create-token

# Output will show your new token mint address, save this!
# Example: Token Mint: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
```

### Create token account:

```bash
# Create token account for your wallet
spl-token create-account <TOKEN_MINT_ADDRESS>

# Or create a specific account
spl-token create-account <TOKEN_MINT_ADDRESS> <YOUR_WALLET_ADDRESS>
```

### Mint initial supply:

```bash
# Mint tokens to your wallet
spl-token mint <TOKEN_MINT_ADDRESS> <AMOUNT>

# Example: Mint 1,000,000 tokens (with 9 decimals)
spl-token mint 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU 1000000000000
```

## Step 2: Token Configuration

### Set decimals (if needed):

```bash
# Most tokens use 9 decimals (like SOL)
# This is set during mint creation, verify with:
spl-token tokenize-account-info <TOKEN_MINT_ADDRESS>
```

### Revoke mint authority (recommended for security):

```bash
# Revoke mint authority to prevent future minting
spl-token authorize <TOKEN_MINT_ADDRESS> mint --disable

# This prevents additional tokens from being minted after launch
```

### Revoke freeze authority (optional):

```bash
# Revoke freeze authority to prevent freezing accounts
spl-token authorize <TOKEN_MINT_ADDRESS> freeze --disable

# This gives users confidence their tokens won't be frozen
```

## Step 3: Token Metadata (Optional but Recommended)

For Phantom to display your token correctly, you should add metadata:

### Options for metadata:

1. **Use Metaplex Standard**: Create metadata on-chain
2. **Use Third-party Service**: Use services like Solana token list
3. **Simple Approach**: Let Phantom fetch basic info

### Basic metadata setup (if using Metaplex):

```bash
# This requires the Metaplex CLI tool
# Create metadata for your token
metaplex create-metadata \
  --keypair <YOUR_KEYPAIR> \
  --mint <TOKEN_MINT_ADDRESS> \
  --name "Your Token Name" \
  --symbol "SYMBOL" \
  --uri "https://your-website.com/metadata.json"
```

### Create metadata.json:

```json
{
  "name": "Your Token Name",
  "symbol": "SYMBOL",
  "description": "Your token description",
  "image": "https://your-website.com/token-logo.png",
  "external_url": "https://your-website.com",
  "attributes": []
}
```

## Step 4: Verify Token in Phantom

1. Open Phantom wallet
2. Click "Add Token"
3. Paste your token mint address
4. Verify token appears with correct name and symbol
5. Send a small amount to test transfer functionality

## Step 5: Prepare for Sol-Launch Integration

Once your token is created and verified:

1. **Save your token mint address** - this will be used in Sol-Launch initialization
2. **Decide on anti-sniper settings** - see MAINNET_LAUNCH_GUIDE.md
3. **Prepare liquidity** - see DEX_LIQUIDITY_GUIDE.md
4. **Initialize Sol-Launch contract** - see MAINNET_LAUNCH_GUIDE.md

## Token Creation Checklist

- [ ] Token mint created
- [ ] Token account created  
- [ ] Initial supply minted
- [ ] Decimals configured (typically 9)
- [ ] Mint authority revoked (recommended)
- [ ] Freeze authority revoked (optional)
- [ ] Metadata created (recommended)
- [ ] Token appears in Phantom
- [ ] Token transfers work correctly
- [ ] Token mint address saved for Sol-Launch initialization

## Security Best Practices

1. **Revoke mint authority** after initial minting
2. **Store private keys securely** (hardware wallet recommended)
3. **Test on devnet first** before mainnet
4. **Keep freeze authority** only if you need emergency freeze capability
5. **Use multisig** for authority if team-managed

## Troubleshooting

### Token not appearing in Phantom:
- Verify token mint address is correct
- Wait a few minutes for network confirmation
- Try adding token manually by mint address
- Check if you have a token account for this mint

### Transfer issues:
- Ensure you have sufficient SOL for transaction fees
- Verify token account exists for receiving wallet
- Check if token is frozen (if freeze authority exists)

### Authority issues:
- Ensure you're using the mint authority wallet
- Check if authority was already revoked
- Verify keypair is correct

## Next Steps

After token creation:
1. Follow DEX_LIQUIDITY_GUIDE.md to add liquidity
2. Follow MAINNET_LAUNCH_GUIDE.md to initialize Sol-Launch protections
3. Test the complete flow on devnet
4. Deploy to mainnet when ready

## Important Notes

- **Token creation is separate from Sol-Launch** - Sol-Launch only protects trading
- **Standard SPL tokens work with Sol-Launch** - no special token features needed
- **Anti-sniper protections apply to trading phase** - token creation is unrestricted
- **Phantom compatibility is standard** - if token works in Phantom, it works with Sol-Launch