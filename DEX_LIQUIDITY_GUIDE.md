# DEX Liquidity Guide

This guide explains how to add liquidity to your token on Solana DEXs so users can trade it after the Sol-Launch protected trading period.

## Overview

Sol-Launch provides anti-sniper protection during the initial trading phase. After this protection period, your token can be traded on Solana DEXs like Raydium, Jupiter, and Orca.

## Prerequisites

- Token created (see TOKEN_CREATION_GUIDE.md)
- Phantom wallet configured (see PHANTOM_SETUP_GUIDE.md)
- Sufficient SOL for liquidity and transaction fees
- Token mint address available

## Supported Solana DEXs

### Major DEXs:
1. **Raydium** - Most popular for new token launches
2. **Jupiter** - Aggregator for best prices
3. **Orca** - Professional DEX with advanced features
4. **Meteora** - Concentrated liquidity

### Recommended for new tokens:
- **Raydium** for easy liquidity pool creation
- **Jupiter** for swap aggregation after liquidity exists

## Step 1: Prepare for Liquidity

### Calculate liquidity needed:
- Determine initial liquidity amount (SOL + token pair)
- Common ratio: 50 SOL + 50% of token supply
- Minimum for meaningful liquidity: 1 SOL + 1,000,000 tokens
- Consider transaction costs for adding liquidity

### Verify token supply:
```bash
# Check your token supply
spl-token supply <TOKEN_MINT_ADDRESS>

# Check your token balance
spl-token balance <TOKEN_MINT_ADDRESS>
```

### Ensure sufficient SOL:
- Need SOL for transaction fees
- Need SOL for liquidity pairing
- Reserve ~0.1 SOL for fees and operations

## Step 2: Create Liquidity Pool on Raydium

### Access Raydium:
1. Visit https://raydium.io/
2. Connect Phantom wallet
3. Switch to correct network (devnet/mainnet)
4. Navigate to "Liquidity" section

### Create liquidity pool:
1. Click "Create Pool"
2. Select your token from the list
3. Enter SOL amount and token amount
4. Review and confirm pool creation
5. Sign transaction in Phantom

### Pool creation example:
- SOL amount: 50 SOL
- Token amount: 500,000 tokens
- Pool fee tier: 0.3% (standard)
- Your token should appear in pool list

## Step 3: Add Liquidity to Existing Pool

### If pool already exists:
1. Visit Raydium liquidity page
2. Find your token's pool
3. Click "Add Liquidity"
4. Enter SOL and token amounts
5. Sign transaction

### Liquidity provision tips:
- Add liquidity in balanced proportions
- Consider impermanent loss
- Start with smaller amounts, add more if needed
- Monitor pool performance

## Step 4: Verify Liquidity

### Check pool status:
1. Visit Raydium pool page for your token
2. Verify pool shows correct price
3. Check liquidity depth
4. Ensure pool is active

### Test swap functionality:
1. Try swapping SOL for your token
2. Try swapping your token for SOL
3. Verify prices are reasonable
4. Check for slippage

## Step 5: Jupiter Integration

### Jupiter aggregator:
1. Visit https://jup.ag/
2. Connect Phantom wallet
3. Jupiter automatically finds best prices across DEXs
4. Your token will appear once liquidity exists on any DEX
5. Users can swap using Jupiter for best prices

### Why Jupiter:
- Best price aggregation across DEXs
- Most popular swap interface on Solana
- Works with your Raydium liquidity
- Provides better user experience

## Step 6: Alternative DEX Options

### Orca (for advanced users):
1. Visit https://www.orca.so/
2. Connect Phantom wallet
3. Create concentrated liquidity pool
4. Orca offers advanced liquidity features
5. Good for experienced liquidity providers

### Meteora (concentrated liquidity):
1. Visit https://app.meteora.app/
2. Connect Phantom wallet
3. Create dynamic liquidity pool
4. Meteora offers advanced liquidity options
5. Suitable for sophisticated liquidity strategies

## Step 7: Sol-Launch Integration Timeline

### Typical workflow:
1. **Create token** (TOKEN_CREATION_GUIDE.md)
2. **Initialize Sol-Launch** with anti-sniper protections (MAINNET_LAUNCH_GUIDE.md)
3. **Run protected trading period** through Sol-Launch contract
4. **Add DEX liquidity** after protection period
5. **Enable open trading** on DEXs

### Why this order:
- Protected trading ensures fair distribution
- DEX liquidity comes after fair distribution
- Prevents snipers from getting early DEX access
- Ensures anti-sniper protections are effective

## Step 8: Monitor Trading

### Monitor pool performance:
1. Check pool depth regularly
2. Monitor trading volume
3. Watch for unusual activity
4. Adjust liquidity if needed

### Monitor price stability:
1. Track price changes
2. Look for extreme volatility
3. Consider adding more liquidity if needed
4. Monitor for potential manipulation

## Liquidity Provision Best Practices

### Security:
1. Never share private keys
2. Use hardware wallet for large amounts
3. Verify pool addresses carefully
4. Double-check transaction details

### Strategy:
1. Start with reasonable liquidity amounts
2. Add more liquidity gradually if needed
3. Consider impermanent loss
4. Monitor pool performance

### Timing:
1. Add liquidity after Sol-Launch protection period
2. Consider market conditions
3. Avoid adding during high volatility
4. Test on devnet first

## Troubleshooting Liquidity Issues

### Pool creation failed:
- Check if pool already exists
- Verify sufficient token balance
- Ensure sufficient SOL balance
- Check network status

### Swap not working:
- Verify pool has sufficient liquidity
- Check if pool is active
- Ensure token is listed on DEX
- Try refreshing DEX interface

### Price issues:
- Check pool liquidity depth
- Verify correct token pair
- Consider adding more liquidity
- Monitor for manipulation

### Transaction failed:
- Check wallet SOL balance
- Verify network status
- Increase slippage tolerance
- Try increasing transaction fee

## Liquidity Management

### Adding more liquidity:
1. Visit DEX liquidity page
2. Select your token pool
3. Click "Add Liquidity"
4. Enter amounts and confirm

### Removing liquidity:
1. Visit DEX liquidity page
2. Select your token pool
3. Click "Remove Liquidity"
4. Enter amounts and confirm
5. Receive liquidity provider tokens

### Emergency measures:
1. If pool has issues, remove liquidity temporarily
2. Consider different DEX if Raydium has problems
3. Monitor for unusual activity
4. Have contingency plan ready

## Mainnet vs Devnet

### Devnet testing:
1. Test liquidity creation on devnet first
2. Verify all operations work correctly
3. Test swap functionality
4. Check for any issues

### Mainnet deployment:
1. Use larger liquidity amounts for real trading
2. Consider gas costs for mainnet
3. Monitor real market conditions
4. Be prepared for higher volatility

## Liquidity Amount Recommendations

### Minimum viable liquidity:
- **Small launch**: 1 SOL + 100,000 tokens
- **Medium launch**: 10 SOL + 1,000,000 tokens
- **Large launch**: 50 SOL + 5,000,000 tokens

### For meaningful trading:
- Need sufficient liquidity for reasonable swaps
- Consider 24h trading volume expectations
- Start smaller, add more if needed
- Monitor and adjust based on demand

## Next Steps

After liquidity setup:
1. Test swap functionality on devnet
2. Verify price discovery works
3. Monitor pool performance after launch
4. Follow MAINNET_LAUNCH_GUIDE.md for complete process

## Important Notes

- **Sol-Launch controls initial trading** - DEX liquidity comes after protection period
- **Standard DEX operations apply** - Sol-Launch doesn't interfere with DEX functionality
- **Anti-sniper protections are time-limited** - after protection period, normal DEX trading works
- **Multiple DEXs supported** - Raydium, Jupiter, Orca, Meteora all work with standard SPL tokens
- **No custom exchange needed** - existing Solana DEX infrastructure is sufficient

## Security Considerations

### Liquidity security:
1. Use hardware wallet for large liquidity amounts
2. Verify pool addresses carefully
3. Start with smaller amounts, add more gradually
4. Monitor for unusual activity

### Smart contract security:
1. Liquidity provider tokens represent your position
2. Keep LP tokens secure (hardware wallet)
3. Understand impermanent loss risks
4. Be cautious of DeFi exploits

### Trading security:
1. Be aware of phishing sites
2. Always verify URL before connecting wallet
3. Double-check transaction details
4. Use reputable DEXs only