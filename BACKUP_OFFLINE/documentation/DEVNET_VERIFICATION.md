# Devnet Verification Guide

Minimal verification steps to ensure the Sol-Launch system works correctly on devnet before mainnet deployment.

## Prerequisites

- Solana CLI configured for devnet
- Anchor CLI installed
- Phantom wallet with devnet SOL
- Devnet SOL airdrop capability

## Step 1: Configure for Devnet

```bash
# Set Solana CLI to devnet
solana config set --url devnet

# Verify configuration
solana config get

# Airdrop devnet SOL (if needed)
solana airdrop 2

# Check balance
solana balance
```

## Step 2: Deploy Smart Contract to Devnet

```bash
# Build the program
anchor build

# Deploy to devnet
anchor deploy --provider-cluster devnet

# Note the program ID
anchor keys list
```

## Step 3: Create Test Token on Devnet

```bash
# Create test token mint
spl-token create-token

# Note the token mint address
TOKEN_MINT=<output_from_previous_command>

# Create token account
spl-token create-account $TOKEN_MINT

# Mint test supply
spl-token mint $TOKEN_MINT 1000000000000

# Verify balance
spl-token balance $TOKEN_MINT
```

## Step 4: Initialize Sol-Launch on Devnet

Use the frontend or direct script to initialize Sol-Launch:

```bash
# Or use the test script to initialize
# This would call initialize_launch with the test token
```

### Anti-sniper settings for devnet testing:
- max_buy: 1000 tokens
- max_wallet: 5000 tokens
- cooldown_seconds: 60
- progressive_limits_enabled: true
- initial_max_buy: 500 tokens
- initial_max_wallet: 2500 tokens
- anti_scam_enabled: true
- max_trades_per_user: 20

## Step 5: Deposit Tokens to Vault

```bash
# Deposit tokens to the Sol-Launch vault
# This would call deposit_tokens instruction
# Amount: 500000 tokens (for testing)
```

## Step 6: Enable Trading

```bash
# Enable trading through Sol-Launch
# This would call enable_trading instruction
```

## Step 7: Test Protected Trading

### Test anti-sniper features:
1. Try to buy more than max_buy limit (should fail)
2. Try to exceed max_wallet limit (should fail)
3. Try to trade within cooldown period (should fail)
4. Try normal trade within limits (should succeed)

### Verify protections work:
- Max buy limits enforced
- Max wallet limits enforced
- Cooldown timers working
- Progressive limits increasing over time
- Anti-scam max trades per user enforced

## Step 8: Verify Token in Phantom

### Add token to Phantom:
1. Open Phantom wallet
2. Click "Add Token"
3. Paste your test token mint address
4. Verify token appears with correct details

### Test token operations:
1. Send test tokens to another wallet
2. Receive test tokens from another wallet
3. Verify all operations work correctly

## Step 9: Test Normal Trading (After Protection Period)

### After anti-sniper period:
1. Disable trading through Sol-Launch
2. Add liquidity to Raydium devnet
3. Test swaps through Jupiter/Raydium
4. Verify token trades normally on DEXs

## Verification Checklist

- [ ] Smart contract deployed to devnet
- [ ] Test token created successfully
- [ ] Sol-Launch initialized with test token
- [ ] Tokens deposited to vault
- [ ] Trading enabled through contract
- [ ] Anti-sniper protections tested and working
- [ ] Token appears in Phantom correctly
- [ ] Token transfers work in Phantom
- [ ] Max buy limits enforced
- [ ] Max wallet limits enforced
- [ ] Cooldown timers working
- [ ] Normal users can trade within limits
- [ ] DEX liquidity tested
- [ ] Token swaps work after protection period

## Minimal Testing Required

### Essential tests only:
1. **Token creation**: Verify standard SPL token creation works
2. **Phantom connection**: Verify token appears and transfers work
3. **Anti-sniper basics**: Test that max buy/wallet limits work
4. **Normal trading**: Test that legitimate users can trade

### Skip extensive testing:
- Don't test all edge cases
- Don't test complex scenarios
- Don't run full simulations
- Don't stress test the system

## Clean Up Devnet Testing

### After verification:
1. Reclaim devnet SOL if needed
2. Close devnet liquidity pools
3. Disable test Sol-Launch instance
4. Document any issues found

## Common Devnet Issues

### Airdrop not working:
- Solana devnet sometimes has rate limits
- Try again after a few minutes
- Use alternative faucet if needed

### Deployment fails:
- Check devnet status
- Ensure sufficient SOL balance
- Try again after network stabilizes

### Transaction failures:
- Check devnet network status
- Verify sufficient SOL for fees
- Try increasing transaction fee

## Success Criteria

### Verification successful if:
- Token creation works on devnet
- Phantom can display and transfer token
- Anti-sniper protections work in Sol-Launch
- Normal users can trade within limits
- No critical bugs or security issues found

## Next Steps

After successful devnet verification:
1. Document any issues found
2. Fix any critical bugs
3. Follow MAINNET_LAUNCH_GUIDE.md for mainnet deployment
4. Ensure all documentation is complete
5. Prepare for mainnet launch

## Important Notes

- **Devnet is for verification only** - not for actual token launch
- **Use small amounts for testing** - don't waste devnet resources
- **Test core functionality** - don't test edge cases
- **Focus on anti-sniper protections** - that's the key feature
- **Standard operations should work** - SPL tokens and Phantom are well-tested