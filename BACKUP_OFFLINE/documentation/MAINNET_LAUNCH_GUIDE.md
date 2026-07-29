# Mainnet Launch Guide

Complete guide for launching your Solana token with Sol-Launch anti-sniper protections on mainnet.

## Overview

This guide covers the complete process from token creation to mainnet launch with anti-sniper protections, followed by DEX liquidity integration for open trading.

## Prerequisites

- Solana CLI installed and configured
- Anchor CLI installed
- Phantom wallet with mainnet SOL
- Token created (see TOKEN_CREATION_GUIDE.md)
- Phantom configured (see PHANTOM_SETUP_GUIDE.md)
- Sufficient SOL for deployment and operations (~5-10 SOL)

## Step 1: Configure for Mainnet

```bash
# Set Solana CLI to mainnet
solana config set --url mainnet-beta

# Verify configuration
solana config get

# Check wallet balance
solana balance

# If insufficient, transfer SOL to your wallet
```

## Step 2: Deploy Smart Contract to Mainnet

```bash
# Build the program
anchor build

# Deploy to mainnet
anchor deploy --provider-cluster mainnet-beta

# Note the program ID
anchor keys list

# Verify deployment
solana program show <PROGRAM_ID>
```

**Program ID**: `2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj`

## Step 3: Create Your Token

Follow TOKEN_CREATION_GUIDE.md for complete token creation steps:

```bash
# Create token mint
spl-token create-token

# Save the token mint address
TOKEN_MINT=<your_token_mint_address>

# Create token account
spl-token create-account $TOKEN_MINT

# Mint initial supply
spl-token mint $TOKEN_MINT <AMOUNT>

# Revoke mint authority (recommended)
spl-token authorize $TOKEN_MINT mint --disable

# Revoke freeze authority (optional)
spl-token authorize $TOKEN_MINT freeze --disable
```

## Step 4: Initialize Sol-Launch with Anti-Sniper Protections

### Configure anti-sniper parameters:
- **max_buy**: Maximum tokens per transaction (e.g., 1000)
- **max_wallet**: Maximum tokens per wallet (e.g., 5000)
- **cooldown_seconds**: Minimum time between trades (e.g., 60)
- **progressive_limits_enabled**: Enable gradual limit increases
- **initial_max_buy**: Starting buy limit (e.g., 500)
- **initial_max_wallet**: Starting wallet limit (e.g., 2500)
- **limit_increase_interval**: Time between limit increases (e.g., 300 seconds)
- **limit_increase_multiplier**: Multiplier for increases (e.g., 1)
- **anti_scam_enabled**: Enable max trades per user limit
- **max_trades_per_user**: Maximum trades per user (e.g., 20)
- **wallet_blacklist_enabled**: Enable wallet blacklist functionality

### Initialize launch:
```bash
# Use the frontend or script to initialize
# This calls initialize_launch with your parameters
```

### Recommended configuration for mainnet:
```javascript
{
  tokenMint: "<YOUR_TOKEN_MINT_ADDRESS>",
  maxBuy: 1000,
  maxWallet: 5000,
  cooldownSeconds: 60,
  totalSupply: 1000000,
  sniperProtectionEnabled: true,
  minTradingDuration: 300,
  walletBlacklistEnabled: false,
  progressiveLimitsEnabled: true,
  initialMaxBuy: 500,
  initialMaxWallet: 2500,
  limitIncreaseInterval: 300,
  limitIncreaseMultiplier: 1,
  antiScamEnabled: true,
  maxTradesPerUser: 20
}
```

## Step 5: Deposit Tokens to Vault

```bash
# Deposit tokens to Sol-Launch vault
# Amount: Typically 50-80% of total supply
# Example: Deposit 500,000 tokens for trading
```

## Step 6: Enable Protected Trading

```bash
# Enable trading through Sol-Launch
# This starts the anti-sniper protected trading period
# Users can now trade through the contract with protections
```

## Step 7: Monitor Protected Trading Period

### During protected trading:
1. Monitor trading volume
2. Watch for suspicious activity
3. Check that anti-sniper protections are working
4. Monitor trader participation
5. Be ready to pause if issues arise

### Emergency controls:
- **Pause trading**: Use `pause_launch` if issues detected
- **Disable trading**: Use `disable_trading` for emergency stop
- **Blacklist wallets**: Add suspicious wallets to blacklist if needed

## Step 8: End Protected Trading Period

### After protection period:
1. Disable trading through Sol-Launch
2. Users can now trade on DEXs
3. Add liquidity to Raydium (see DEX_LIQUIDITY_GUIDE.md)
4. Enable open trading on DEXs

### Why end protected trading:
- Fair distribution achieved
- Anti-sniper protections no longer needed
- DEX liquidity provides better market
- Normal market forces take over

## Step 9: Add DEX Liquidity

Follow DEX_LIQUIDITY_GUIDE.md for complete liquidity setup:

```bash
# Visit Raydium
# Connect Phantom wallet
# Create liquidity pool with your token
# Add SOL and token liquidity
# Example: 50 SOL + 500,000 tokens
```

## Step 10: Enable Open Trading

### After liquidity is added:
1. Your token is now tradeable on Raydium
2. Jupiter will aggregate your token for swaps
3. Users can swap SOL for your token
4. Normal market trading begins

### Token availability:
- Users can find your token on Raydium
- Jupiter will show your token in swap interface
- Phantom users can add your token and trade
- Your token is now fully integrated in Solana ecosystem

## Step 11: Monitor Launch

### Monitor trading:
1. Track trading volume on DEXs
2. Monitor price action
3. Watch for unusual activity
4. Check community feedback

### Monitor protections:
1. Review protected trading performance
2. Check if anti-sniper measures were effective
3. Analyze trader participation
4. Document any issues for future launches

## Mainnet Security Checklist

### Pre-launch:
- [ ] Smart contract deployed to mainnet
- [ ] Token created with proper authorities
- [ ] Mint authority revoked (recommended)
- [ ] Token metadata created
- [ ] Token verified in Phantom
- [ ] Anti-sniper parameters configured
- [ ] Sol-Launch initialized with token
- [ ] Tokens deposited to vault
- [ ] Sufficient SOL for operations

### During launch:
- [ ] Trading enabled through contract
- [ ] Anti-sniper protections active
- [ ] Trading monitored for suspicious activity
- [ ] Emergency controls ready if needed
- [ ] Community communication clear

### Post-launch:
- [ ] Trading disabled after protection period
- [ ] DEX liquidity added
- [ ] Open trading enabled
- [ ] Token trading on DEXs
- [ ] Monitoring continued
- [ ] Issues documented

## Security Best Practices for Mainnet

### Wallet security:
- Use hardware wallet for authority
- Never share private keys
- Use multisig if team-managed
- Keep seed phrase secure

### Smart contract security:
- Verify program ID is correct
- Monitor for unusual activity
- Have emergency procedures ready
- Keep documentation updated

### Trading security:
- Monitor for manipulation
- Watch for wash trading
- Check for unusual patterns
- Be prepared to intervene if needed

## Mainnet vs Devnet Differences

### Mainnet considerations:
- Real financial stakes involved
- Higher transaction costs
- Market volatility
- Real users and real money
- Security is critical

### Testing recommendations:
- Complete devnet verification first
- Test with small amounts initially
- Monitor continuously
- Have rollback procedures ready

## Troubleshooting Mainnet Issues

### Deployment issues:
- Check mainnet status
- Verify sufficient SOL balance
- Check program ID consistency
- Try again if network congested

### Trading issues:
- Monitor network congestion
- Check RPC endpoint status
- Verify sufficient liquidity
- Monitor for unusual activity

### Security issues:
- Pause trading immediately if suspicious activity
- Review authorization settings
- Check for unauthorized access
- Document and report security incidents

## Post-Launch Activities

### Ongoing monitoring:
1. Monitor trading volume and price
2. Watch for unusual patterns
3. Check community feedback
4. Monitor smart contract performance

### Improvements:
1. Document lessons learned
2. Improve anti-sniper parameters if needed
3. Enhance monitoring systems
4. Prepare for future launches

## Mainnet Launch Timeline

### Typical timeline:
1. **Preparation (1-2 weeks)**: Token creation, testing, configuration
2. **Deployment (1 day)**: Smart contract deployment, initialization
3. **Protected trading (hours to days)**: Anti-sniper protected trading period
4. **Liquidity addition (1 day)**: DEX liquidity setup
5. **Open trading (ongoing)**: Normal DEX trading

### Why this timeline:
- Ensures fair distribution
- Prevents sniper abuse
- Allows for smooth transition to open market
- Provides time for monitoring and adjustments

## Final Verification

### Launch success criteria:
- [ ] Token created and verified
- [ ] Smart contract deployed and working
- [ ] Anti-sniper protections active during protected period
- [ ] Users can trade within protection limits
- [ ] Normal users cannot abuse protections
- [ ] Token successfully transitions to DEX trading
- [ ] No critical security issues
- [ ] Community feedback positive

## Important Notes

- **This is a single token launch system** - designed for one secure launch
- **Anti-sniper protections are time-limited** - after protection period, normal DEX trading applies
- **No marketplace or exchange functionality** - standard Solana DEXs handle trading
- **Phantom compatible** - your token works like any other SPL token
- **Standard SPL token** - your token is a normal Solana token with enhanced launch protection

## Support and Resources

### Documentation:
- TOKEN_CREATION_GUIDE.md - Token creation steps
- PHANTOM_SETUP_GUIDE.md - Phantom wallet setup
- DEX_LIQUIDITY_GUIDE.md - DEX liquidity setup
- DEVNET_VERIFICATION.md - Devnet testing steps
- This guide - Complete mainnet launch process

### Solana resources:
- Solana documentation: https://docs.solana.com/
- Raydium documentation: https://docs.raydium.io/
- Jupiter documentation: https://jup.ag/docs
- Phantom support: https://phantom.app/help

### Emergency contacts:
- Have Solana dev contacts if needed
- Monitor Solana status: https://status.solana.com/
- Keep Anchor documentation handy

## Success Criteria

### Launch is successful when:
1. Token is created and verified on mainnet
2. Sol-Launch anti-sniper protections work correctly
3. Fair distribution achieved during protected period
4. Token successfully transitions to DEX trading
5. Users can trade token normally on Solana DEXs
6. No critical security issues or exploits
7. Community is satisfied with the launch process

## Final Reminder

**The goal is a secure, fair single token launch** with anti-sniper protections during the initial trading phase, followed by normal DEX trading. The system is designed to be simple, secure, and effective without unnecessary marketplace complexity.