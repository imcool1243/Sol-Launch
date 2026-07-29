# Phantom Setup Guide

This guide explains how to set up Phantom wallet for the Sol-Launch token launch and ensure compatibility.

## Phantom Wallet Installation

### Install Phantom Extension:
1. Visit https://phantom.app/
2. Download browser extension (Chrome, Brave, Firefox, Edge)
3. Install and create new wallet
4. **Backup your seed phrase securely**

### Mobile App:
1. Download Phantom app from App Store or Google Play
2. Create wallet or sync with desktop extension

## Network Configuration

### Switch to Solana Network:
1. Open Phantom wallet
2. Click network selector (top left)
3. Select "Solana"
4. For testing, use "Solana Devnet"
5. For mainnet, use "Solana"

### Connect to RPC (if needed):
1. Click gear icon in Phantom
2. Add custom RPC
3. Use Solana devnet/mainnet RPC
4. Devnet: https://api.devnet.solana.com
5. Mainnet: https://api.mainnet-beta.solana.com

## Wallet Backup

### Secure Your Wallet:
1. **Write down seed phrase** on paper
2. Store in secure location (safe deposit box)
3. Never share seed phrase with anyone
4. Never enter seed phrase in websites or forms
5. Consider hardware wallet for mainnet

### Hardware Wallet Integration:
1. Ledger: Connect via USB, open Phantom, select Ledger
2. Trezor: Connect via USB, open Phantom, select Trezor
3. Follow on-screen prompts to approve transactions

## Token Addition in Phantom

### Add Custom Token:
1. Open Phantom wallet
2. Click "Add Token" (bottom left)
3. Paste token mint address
4. Click "Add"
5. Token should appear in your wallet

### Verify Token Details:
1. Click on the token in Phantom
2. Check token name, symbol, and decimals
3. Verify balance displays correctly
4. Check that transfers work

## Sol-Launch Compatibility

### Phantom works seamlessly with Sol-Launch:

1. **Token Display**: Your token will display with standard SPL format
2. **Trading**: Users can trade your token through Phantom after launch
3. **Swaps**: Phantom integrates with DEXs (Jupiter, Raydium) for swapping
4. **Anti-Sniper**: Sol-Launch protections work at the contract level, not wallet level

### What Sol-Launch Does NOT Affect:
- Does not modify Phantom wallet functionality
- Does not require special Phantom settings
- Does not interfere with normal token operations
- Does not affect DEX swaps after trading protection period

## Testing Phantom Compatibility

### Test Token Operations:
1. Send small amount of your token to another wallet
2. Receive tokens from another wallet
3. Add token to Phantom on another device
4. Verify all operations work correctly

### Test with Sol-Launch:
1. After Sol-Launch initialization, test trading through the contract
2. Verify Phantom shows updated balances
3. Check that normal transfers still work outside contract
4. Ensure DEX swaps work after protection period

## Phantom Configuration for Mainnet

### Security Settings:
1. Enable "Auto-approve" only for trusted sites
2. Enable "Phantom" for security notifications
3. Set up transaction notifications
4. Keep app updated

### Network Settings:
1. Use mainnet-beta for production
2. Ensure sufficient SOL balance for gas fees
3. Verify RPC endpoint is reliable
4. Consider using dedicated RPC for better performance

## Troubleshooting Phantom Issues

### Token Not Appearing:
- Check network is correct (Solana, not other networks)
- Verify token mint address is correct
- Ensure you have a token account for this mint
- Try refreshing Phantom wallet

### Transaction Failed:
- Check if wallet has sufficient SOL balance
- Verify network is not congested
- Check if RPC endpoint is working
- Try increasing fee settings in Phantom

### Wallet Not Connecting:
- Check browser extension is enabled
- Try refreshing the page
- Check if Phantom is unlocked
- Try different browser if issue persists

### Balance Not Updating:
- Wait for network confirmation
- Refresh wallet balance
- Check network status
- Verify transaction was successful

## Phantom Security Best Practices

### Protection Measures:
1. Never share seed phrase or private key
2. Always verify transaction details before signing
3. Use hardware wallet for significant amounts
4. Enable transaction notifications
5. Be cautious of phishing sites

### Safe Usage:
1. Only connect to trusted websites
2. Verify URL before connecting wallet
3. Review transaction details carefully
4. Disconnect wallet after using suspicious sites
5. Use separate wallet for testing vs mainnet

## Phantom Integration with Sol-Launch

### How Users Will Interact:
1. Users will connect Phantom to Sol-Launch frontend
2. Users will sign transactions to participate in protected trading
3. After protection period, users can trade normally through DEXs
4. Phantom will display token balances normally throughout

### User Experience:
- **During protected trading**: Users interact with Sol-Launch contract
- **After protection period**: Users can trade freely on DEXs
- **Token visibility**: Token appears normally in Phantom
- **Transfers**: Standard SPL token transfers work normally

## Phantom Wallet for Specific Roles

### For Token Authority:
- Use hardware wallet (Ledger/Trezor)
- Keep seed phrase secure
- Use dedicated wallet for token operations
- Test all operations before mainnet

### For Liquidity Provider:
- Use wallet with sufficient SOL balance
- Ensure wallet can handle multiple transactions
- Consider using separate wallet for liquidity
- Test liquidity operations on devnet first

### For Normal Users:
- Any Phantom wallet works with standard tokens
- No special configuration needed
- Normal SPL token operations apply
- Sol-Launch protections work transparently

## Next Steps

After Phantom setup:
1. Test token operations with your created token
2. Verify Phantom can send/receive your token
3. Follow DEX_LIQUIDITY_GUIDE.md to add liquidity
4. Follow MAINNET_LAUNCH_GUIDE.md for complete launch process

## Important Notes

- **Phantom is fully compatible** with Sol-Launch and standard SPL tokens
- **No special Phantom configuration** needed for Sol-Launch
- **Anti-sniper protections work at contract level**, not wallet level
- **Users can trade normally** after protection period through Phantom-connected DEXs
- **Standard SPL token operations** work throughout the entire process