# Sol-Launch Deployment Guide

Complete deployment guide for the secure single token launch platform.

## Overview

This guide covers deploying the Sol-Launch smart contract to Solana networks (devnet, testnet, mainnet) for secure single token launches with enhanced anti-sniper protection.

## Prerequisites

### Required Tools
- **Solana CLI**: Install from https://docs.solana.com/cli/install-solana-cli-tools
- **Anchor Framework**: Install from https://www.anchor-lang.com/docs/installation
- **Node.js & npm**: Required for frontend development
- **Git**: For version control

### Wallet Setup
```bash
# Create a new keypair (if you don't have one)
solana-keygen new

# Set as default wallet
solana config set --keypair ~/.config/solana/id.json

# Get your public key
solana address
```

## Single Token Launch Workflow

### Step 1: Create/Configure Token
Use your existing SPL token or create a new one:
```bash
# Create new token mint
spl-token create-token

# Create token account
spl-token create-account <TOKEN_MINT>

# Mint tokens
spl-token mint <TOKEN_MINT> <AMOUNT>
```

### Step 2: Initialize Launch with Enhanced Security
Initialize the secure launch with advanced anti-sniper features:
```javascript
const launchParams = {
  tokenMint: 'YOUR_TOKEN_MINT_ADDRESS',
  maxBuy: 1000,
  maxWallet: 5000,
  cooldownSeconds: 60,
  totalSupply: 1000000,
  sniperProtectionEnabled: true,
  minTradingDuration: 300,
  // Enhanced anti-sniper features
  walletBlacklistEnabled: false,
  progressiveLimitsEnabled: true,
  initialMaxBuy: 500,
  initialMaxWallet: 2500,
  limitIncreaseInterval: 300, // 5 minutes
  limitIncreaseMultiplier: 1,
  antiScamEnabled: true,
  maxTradesPerUser: 20,
};

const result = await solanaClient.initializeLaunch(launchParams);
```

### Step 3: Deposit Tokens
Deposit tokens to the secure vault:
```javascript
const depositAmount = 500000; // Amount to make available for trading
await solanaClient.depositTokens(launchPubkey, depositAmount);
```

### Step 4: Configure Wallet Blacklist (Optional)
Set up wallet blacklist if needed:
```javascript
if (needsBlacklist) {
  // Initialize blacklist state
  await solanaClient.initializeBlacklist(launchPubkey);
  
  // Add suspicious wallets
  await solanaClient.addToBlacklist(launchPubkey, suspiciousWallet);
}
```

### Step 5: Enable Trading
Enable trading to start the fair launch:
```javascript
await solanaClient.enableTrading(launchPubkey);
```

### Step 6: Monitor Launch
Monitor the launch using the dashboard:
```javascript
const launchStatus = await solanaClient.getLaunchStatus(launchPubkey);
console.log('Launch Status:', launchStatus);
```

## Anti-Sniper Configuration Guide

### Progressive Limits Strategy
Recommended configuration for progressive limits:
```javascript
progressiveLimitsEnabled: true,
initialMaxBuy: 500,           // Start at 50% of final limit
initialMaxWallet: 2500,       // Start at 50% of final limit
limitIncreaseInterval: 300,   // Increase every 5 minutes
limitIncreaseMultiplier: 1,   // 1x increase per interval
```

### Anti-Scam Protection
Recommended anti-scam settings:
```javascript
antiScamEnabled: true,
maxTradesPerUser: 20,         // Limit to 20 trades per user
```

### Wallet Blacklist
Use blacklist for known bad actors:
```javascript
walletBlacklistEnabled: true,
// Then add specific wallets post-deployment
```

## Enhanced Security Features

### Progressive Limits
- Starts with lower buy/wallet limits
- Gradually increases over time
- Discourages early sniping
- Formula: `current_limit = initial_limit * (1 + multiplier * intervals_elapsed)`

### Anti-Scam Protection
- Limits trades per user
- Prevents bot accumulation
- Configurable per-user limits
- Protects against coordinated attacks

### Wallet Blacklist
- Block known bot/scammer wallets
- Authority-controlled management
- Up to 50 blacklisted wallets
- Real-time enforcement

## Devnet Deployment

### 1. Configure for Devnet
```bash
# Set cluster to devnet
solana config set --url devnet

# Verify configuration
solana config get
```

### 2. Fund Your Wallet
```bash
# Request airdrop (if needed)
solana airdrop 1

# Check balance
solana balance
```

### 3. Build the Program
```bash
# Build the smart contract
anchor build

# This creates the target/deploy/sol_launch.so file
```

### 4. Deploy to Devnet
```bash
# Using the deployment script
./scripts/deploy.sh

# Or manually
anchor deploy --provider-cluster devnet
```

### 5. Verify Deployment
```bash
# Check program is deployed
solana program show 2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj

# Get program ID
solana program dump 2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj
```

## Frontend Deployment

### 1. Install Dependencies
```bash
cd frontend
npm install
```

### 2. Configure Environment
The frontend is already configured for devnet in `src/utils/solanaClient.js`:
```javascript
this.connection = new Connection('https://api.devnet.solana.com');
```

### 3. Build for Production
```bash
npm run build
```

### 4. Deploy to Hosting
```bash
# Example: Deploy to Vercel
npm install -g vercel
vercel

# Or deploy to Netlify
npm install -g netlify-cli
netlify deploy --prod
```

## Testing on Devnet

### 1. Test Smart Contract
```bash
# Run unit tests
anchor test

# Run integration tests (when implemented)
anchor test --skip-local-validator
```

### 2. Test Frontend
1. Connect your wallet (Phantom)
2. Create a new launch
3. Deposit tokens to vault
4. Enable trading
5. Execute trades
6. Verify all functionality

## Mainnet Deployment (Future)

### 1. Configure for Mainnet
```bash
# Set cluster to mainnet
solana config set --url mainnet-beta

# Verify configuration
solana config get
```

### 2. Fund Your Wallet
- Purchase SOL from an exchange
- Transfer to your wallet
- Ensure sufficient balance for deployment

### 3. Build and Deploy
```bash
# Build the program
anchor build

# Deploy to mainnet
anchor deploy --provider-cluster mainnet-beta
```

### 4. Update Frontend Configuration
Change the RPC URL in `frontend/src/utils/solanaClient.js`:
```javascript
this.connection = new Connection('https://api.mainnet-beta.solana.com');
```

## Monitoring

### Program Logs
```bash
# Monitor program logs
solana logs 2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj
```

### Account Data
```bash
# Get launch state
solana account <LAUNCH_PUBKEY>

# Get vault balance
solana account <VAULT_PUBKEY>
```

## Troubleshooting

### Common Issues

#### 1. Insufficient Funds
```bash
# Check balance
solana balance

# Request airdrop (devnet only)
solana airdrop 1
```

#### 2. Program ID Mismatch
```bash
# Update Anchor.toml with correct program ID
# Update frontend IDL with correct program ID
```

#### 3. Wallet Connection Issues
- Ensure wallet is properly configured
- Check network settings in wallet
- Verify wallet has sufficient SOL

#### 4. Deployment Failures
```bash
# Clean build
anchor clean

# Rebuild
anchor build

# Redeploy
anchor deploy --provider-cluster devnet
```

## Security Considerations

### Before Mainnet Deployment
- ✅ Complete security audit
- ✅ Test all functionality on devnet
- ✅ Verify all security features work correctly
- ✅ Test with real users (beta testing)
- ✅ Ensure proper error handling
- ✅ Verify event emission and logging
- ✅ Test anti-sniper and anti-bot measures
- ✅ Verify vault security
- ✅ Test with significant volume

### Mainnet Checklist
- [ ] Sufficient test coverage
- [ ] Security audit completed
- [ ] Performance testing done
- [ ] Frontend thoroughly tested
- [ ] Error handling verified
- [ ] Monitoring setup
- [ ] Backup procedures
- [ ] Incident response plan

## Rollback Procedures

### Smart Contract Rollback
1. Disable trading on existing launches
2. Deploy previous version if needed
3. Migrate users to new program
4. Update frontend configuration

### Frontend Rollback
1. Revert to previous commit
2. Redeploy frontend
3. Clear browser cache
4. Verify functionality

## Support

For deployment issues:
- Check Solana documentation: https://docs.solana.com/
- Check Anchor documentation: https://www.anchor-lang.com/docs/
- Review program logs: `solana logs <PROGRAM_ID>`
- Check Solana status: https://status.solana.com/