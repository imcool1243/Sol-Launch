# Sol-Launch User Guide

## Getting Started

### Welcome to Sol-Launch

Sol-Launch is a secure and fair memecoin launch platform on Solana. This guide will help you navigate the platform and understand how to create, manage, and trade tokens securely.

## Table of Contents

1. [Wallet Setup](#wallet-setup)
2. [Creating a Launch](#creating-a-launch)
3. [Managing Your Vault](#managing-your-vault)
4. [Trading](#trading)
5. [Security Features](#security-features)
6. [Troubleshooting](#troubleshooting)

---

## Wallet Setup

### Supported Wallets

Sol-Launch currently supports the Phantom wallet. Additional wallet support may be added in the future.

### Setting Up Phantom Wallet

1. **Install Phantom Wallet**
   - Visit https://phantom.app/
   - Download and install the browser extension
   - Create a new wallet or import an existing one

2. **Connect to Devnet**
   - Open Phantom wallet settings
   - Select "Devnet" as the network
   - This is important for testing purposes

3. **Fund Your Wallet**
   - Request SOL airdrop on devnet
   - You'll need SOL for transaction fees
   - Request airdrop: https://faucet.solana.com/

4. **Connect to Sol-Launch**
   - Navigate to the Sol-Launch website
   - Click "Connect Wallet"
   - Approve the connection in Phantom

---

## Creating a Launch

### Understanding Launch Parameters

Before creating a launch, you need to understand the key parameters:

#### Token Mint Address
- The address of your token mint on Solana
- Must be created before creating a launch
- Can be created using Solana CLI or SPL Token

#### Max Buy Amount
- Maximum tokens a user can buy in a single transaction
- Prevents large purchases that could manipulate the market
- Example: 1000 tokens per transaction

#### Max Wallet Amount
- Maximum tokens a single wallet can hold
- Prevents whale accumulation
- Example: 5000 tokens per wallet

#### Cooldown Seconds
- Minimum time between trades for a user
- Prevents high-frequency trading
- Example: 60 seconds between trades

#### Total Supply
- Total number of tokens that will be available
- Includes tokens in vault and any distributed tokens
- Example: 1,000,000 tokens

#### Sniper Protection
- Enables anti-sniper protection features
- Prevents early trading disable attacks
- Recommended: Always enabled for launches

#### Min Trading Duration
- Minimum time trading must be active before can be disabled
- Prevents quick pump-and-dump schemes
- Example: 300 seconds (5 minutes)

### Step-by-Step Launch Creation

1. **Navigate to Create Launch**
   - Click "Create Launch" in the navigation
   - Ensure your wallet is connected

2. **Enter Token Mint Address**
   - Paste your token mint address
   - Verify the address is correct
   - The system will validate the mint

3. **Configure Trading Parameters**
   - Set max buy amount (recommended: 1000)
   - Set max wallet amount (recommended: 5000)
   - Set cooldown period (recommended: 60 seconds)
   - Set total supply (your token's total supply)

4. **Configure Security Features**
   - Enable sniper protection (recommended)
   - Set minimum trading duration (recommended: 300 seconds)
   - These features protect against malicious activities

5. **Create Launch**
   - Review all parameters
   - Click "Create Launch"
   - Approve the transaction in your wallet
   - Wait for confirmation

6. **Save Launch Information**
   - Copy your launch pubkey
   - Save it for future reference
   - This is needed for vault management and trading

---

## Managing Your Vault

### Understanding the Vault

The vault is where your tokens are stored before distribution. It's a secure, PDA-based account that only you (the authority) can control.

### Depositing Tokens

1. **Navigate to Vault Management**
   - Select your launch from the dashboard
   - Click "Vault Management"

2. **Enter Deposit Amount**
   - Enter the number of tokens to deposit
   - Ensure you have sufficient tokens in your wallet
   - The system will validate your token account

3. **Confirm Deposit**
   - Click "Deposit Tokens"
   - Approve the transaction
   - Wait for confirmation

4. **Verify Deposit**
   - Check the vault balance updated
   - Verify the transaction succeeded
   - Your tokens are now in the vault

### Withdrawing Tokens

1. **Navigate to Vault Management**
   - Select your launch from the dashboard
   - Click "Vault Management"

2. **Enter Withdraw Amount**
   - Enter the number of tokens to withdraw
   - Ensure sufficient vault balance
   - Tokens will be sent to your wallet

3. **Confirm Withdrawal**
   - Click "Withdraw Tokens"
   - Approve the transaction
   - Wait for confirmation

4. **Verify Withdrawal**
   - Check vault balance decreased
   - Verify tokens received in your wallet
   - Transaction completed successfully

### Vault Security Tips

- ✅ Only deposit tokens you intend to distribute
- ✅ Keep sufficient tokens for expected trading volume
- ✅ Monitor vault balance regularly
- ✅ Never share your launch authority private key
- ✅ Use sniper protection for all launches

---

## Trading

### Enabling Trading

Before users can trade, you must enable trading:

1. **Navigate to Trading Interface**
   - Select your launch from the dashboard
   - Click "Trading"

2. **Enable Trading**
   - Click "Enable Trading"
   - Approve the transaction
   - Trading is now active

3. **Monitor Trading**
   - Users can now execute trades
   - Monitor total traded volume
   - Watch for any suspicious activity

### Executing Trades

Users can trade when trading is enabled:

1. **Navigate to Trading Interface**
   - Select your launch
   - Ensure trading is enabled

2. **Enter Trade Amount**
   - Enter amount to buy
   - Must be within max buy limit
   - Must not exceed max wallet limit

3. **Execute Trade**
   - Click "Execute Trade"
   - Approve the transaction
   - Tokens are transferred from vault to your wallet

4. **Cooldown Period**
   - Wait for cooldown period before next trade
   - Cooldown prevents high-frequency trading
   - Minimum 1 second between trades

### Disabling Trading

You can disable trading if needed:

1. **Navigate to Trading Interface**
   - Select your launch
   - Click "Trading"

2. **Disable Trading**
   - Click "Disable Trading"
   - Note: Sniper protection may prevent immediate disable
   - Wait for minimum trading duration if enabled

---

## Security Features

### Anti-Sniper Protection

**What it does**:
- Prevents early trading disable attacks
- Ensures fair trading duration
- Protects against pump-and-dump schemes

**How it works**:
- Enforces minimum trading duration
- Prevents quick trading disable
- Allows fair distribution period

**Best practices**:
- Always enable for launches
- Set appropriate minimum duration (300s recommended)
- Monitor trading patterns

### Anti-Bot Rate Limiting

**What it does**:
- Prevents high-frequency trading bots
- Ensures fair trading opportunities
- Reduces front-running risks

**How it works**:
- 1-second minimum between trades
- Combined with configurable cooldown
- Trade count tracking per user

**Best practices**:
- Keep enabled for all launches
- Set appropriate cooldown (60s recommended)
- Monitor trading frequency

### PDA Vault Security

**What it does**:
- Secure vault ownership using PDA
- Prevents unauthorized vault access
- Eliminates single point of failure

**How it works**:
- Vault owned by launch PDA (not wallet)
- PDA-based signing for transfers
- Programmatic vault authority

**Best practices**:
- Never share authority private key
- Monitor vault operations
- Use proper deposit/withdraw procedures

---

## Troubleshooting

### Common Issues

#### 1. Wallet Connection Issues

**Problem**: Cannot connect wallet
**Solution**:
- Ensure Phantom is installed
- Check network settings (use Devnet)
- Refresh the page and try again
- Check Phantom's network selection

#### 2. Transaction Failures

**Problem**: Transactions failing
**Solution**:
- Check wallet has sufficient SOL
- Verify network is Devnet
- Check transaction parameters
- Review error messages for details

#### 3. Launch Creation Issues

**Problem**: Cannot create launch
**Solution**:
- Verify token mint address is correct
- Ensure wallet has sufficient SOL
- Check all parameters are valid
- Review error messages

#### 4. Trading Issues

**Problem**: Cannot execute trades
**Solution**:
- Ensure trading is enabled
- Check within trading limits
- Verify cooldown period elapsed
- Check vault has sufficient tokens

#### 5. Vault Issues

**Problem**: Cannot deposit/withdraw tokens
**Solution**:
- Verify you are the launch authority
- Check token account exists
- Ensure sufficient balance
- Review error messages

### Getting Help

If you encounter issues not covered here:

1. **Check Documentation**: Review README.md and DEPLOYMENT.md
2. **Review Security Audit**: Check SECURITY_AUDIT_REPORT.md
3. **Check Program Logs**: Use monitoring scripts
4. **Contact Support**: Reach out through official channels

---

## Best Practices

### For Launch Creators

1. **Security First**
   - Always enable sniper protection
   - Use appropriate trading limits
   - Monitor trading activity
   - Keep authority key secure

2. **Fair Distribution**
   - Set reasonable max buy limits
   - Use appropriate cooldown periods
   - Monitor for suspicious activity
   - Protect against whales

3. **Transparency**
   - Communicate launch parameters clearly
   - Provide clear instructions
   - Share trading status updates
   - Be responsive to community

### For Traders

1. **Security**
   - Only use official launch interface
   - Verify launch parameters
   - Never share private keys
   - Use official wallet software

2. **Trading**
   - Respect trading limits
   - Be patient with cooldowns
   - Understand the risks
   - Only trade what you can afford to lose

3. **Best Practices**
   - Do thorough research
   - Understand the token economics
   - Monitor launch activity
   - Report suspicious behavior

---

## Advanced Features

### Launch Phases

Sol-Launch uses a phase-based system:

1. **Ready Phase** (1)
   - Launch is initialized
   - Trading not yet enabled
   - Vault management available

2. **Active Phase** (2)
   - Trading is enabled
   - Users can execute trades
   - Normal trading operations

3. **Paused Phase** (3)
   - Trading is disabled
   - Vault management still available
   - Can return to Active phase

### Event Monitoring

All operations emit events for transparency:
- LaunchInitialized: Launch creation
- TokensDeposited: Token deposits
- TokensWithdrawn: Token withdrawals
- TradingEnabled: Trading activation
- TradingDisabled: Trading deactivation
- TradeExecuted: Trade execution

---

## Conclusion

Sol-Launch provides a secure and fair platform for memecoin launches on Solana. By following this guide and understanding the security features, you can safely create and manage token launches.

Remember to always prioritize security, follow best practices, and stay informed about the platform's features and updates.

For technical documentation, see README.md and DEPLOYMENT.md.
For security details, see SECURITY_AUDIT_REPORT.md.