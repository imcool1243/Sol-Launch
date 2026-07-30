# Sol-Launch 🚀

**Solana Secure Token Launch System with Anti-Sniper Protection**

A production-ready Solana smart contract system for fair, secure single token launches with comprehensive anti-sniper protections.

## 🎯 Features

### Anti-Sniper Protections
- **Max Buy Limits**: Maximum tokens allowed per transaction
- **Max Wallet Limits**: Maximum tokens allowed per wallet  
- **Cooldown Timers**: Minimum time between trades (configurable)
- **Progressive Limits**: Gradual increase of buy/wallet limits over time
- **Anti-Scam Protection**: Maximum trades per user limit
- **Anti-Bot Rules**: 1-second minimum between trades

### Launch Management
- **Trading Control**: Enable/disable trading functionality
- **Launch Phases**: Ready, Active, Paused states
- **Authority Management**: Secure authority transfer
- **Vault Security**: PDA-based token vault with authority signing
- **Token Management**: Deposit/withdraw tokens from vault

### Security Features
- **Authority Protection**: Authority-only critical operations
- **PDA Security**: Program Derived Addresses for vault and launch accounts
- **Event Logging**: Comprehensive audit trail for all operations
- **Overflow Protection**: Safe arithmetic operations throughout
- **Validation**: Comprehensive input validation and constraints

## 🏗️ Architecture

### Smart Contract
- **Framework**: Anchor
- **Language**: Rust
- **Network**: Solana Mainnet
- **Program ID**: Generated on deployment (see deployment guide)

### Core Instructions
1. `initialize_launch` - Initialize launch with anti-sniper parameters
2. `deposit_tokens` - Deposit tokens to vault for trading
3. `withdraw_tokens` - Withdraw tokens from vault
4. `enable_trading` - Enable protected trading
5. `disable_trading` - Disable trading (with sniper protection constraints)
6. `execute_trade` - Execute protected token trade
7. `pause_launch` - Pause launch temporarily
8. `resume_launch` - Resume paused launch
9. `start_launch` - Start launch with delay
10. `transfer_authority` - Transfer launch authority

## 📋 Requirements

- Solana CLI 1.18+
- Anchor CLI 0.30+
- Node.js 18+ (for frontend)
- Rust 1.70+ (for building)

## 🚀 Quick Start

### 1. Clone Repository
```bash
git clone https://github.com/imcool1243/Sol-Launch.git
cd Sol-Launch
```

### 2. Install Dependencies
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Install Node.js dependencies (for frontend)
cd frontend
npm install
```

### 3. Build Smart Contract
```bash
anchor build
```

### 4. Deploy to Mainnet
```bash
solana config set --url mainnet-beta
anchor deploy --provider-cluster mainnet-beta
```

### 5. Create Token
```bash
spl-token create-token
spl-token create-account <TOKEN_MINT>
spl-token mint <TOKEN_MINT> <AMOUNT>
```

### 6. Initialize Launch
```bash
# Use frontend or script with parameters
# See documentation for parameter details
```

## 🔧 Configuration

### Anti-Sniper Parameters
- `max_buy`: Maximum tokens per transaction (u64)
- `max_wallet`: Maximum tokens per wallet (u64)
- `cooldown_seconds`: Minimum seconds between trades (i64)
- `total_supply`: Total token supply (u64)
- `sniper_protection_enabled`: Enable sniper protection (bool)
- `min_trading_duration`: Minimum trading duration (i64)
- `progressive_limits_enabled`: Enable progressive limits (bool)
- `initial_max_buy`: Starting buy limit (u64)
- `initial_max_wallet`: Starting wallet limit (u64)
- `limit_increase_interval`: Time between limit increases (i64)
- `limit_increase_multiplier`: Multiplier for limit increases (u64)
- `anti_scam_enabled`: Enable max trades per user (bool)
- `max_trades_per_user`: Maximum trades per user (u64)

## 📚 Documentation

- [FINAL_RELEASE.md](FINAL_RELEASE.md) - Complete release documentation
- [MAINNET_LAUNCH_GUIDE.md](MAINNET_LAUNCH_GUIDE.md) - Mainnet deployment guide
- [TOKEN_CREATION_GUIDE.md](TOKEN_CREATION_GUIDE.md) - Token creation steps
- [PHANTOM_SETUP_GUIDE.md](PHANTOM_SETUP_GUIDE.md) - Phantom wallet setup
- [DEX_LIQUIDITY_GUIDE.md](DEX_LIQUIDITY_GUIDE.md) - DEX liquidity setup
- [LAUNCH_ASSISTANT_GUIDE.md](LAUNCH_ASSISTANT_GUIDE.md) - Launch assistant usage

## 🔒 Security

### Protections Verified
- ✅ Max Buy Limit Enforcement
- ✅ Max Wallet Limit Enforcement
- ✅ Cooldown Timer Enforcement
- ✅ Progressive Limits Implementation
- ✅ Anti-Scam Protection
- ✅ Anti-Bot Rules
- ✅ Authority Protection
- ✅ PDA Security
- ✅ Overflow Protection
- ✅ Comprehensive Validation

### Security Audit
- No critical bugs found
- No authority bypass mechanisms
- No PDA security issues
- No overflow/underflow vulnerabilities
- No race conditions
- Comprehensive validation throughout

## 🧪 Testing

### Unit Tests
```bash
anchor test
```

### Simulation Tests
```bash
anchor test --skip-local-validator
```

### Build Verification
```bash
anchor build
```

## 🌐 Frontend

### Development
```bash
cd frontend
npm install
npm run dev
```

### Build
```bash
npm run build
```

## 📖 Usage

### Launch Workflow
1. Deploy smart contract to mainnet
2. Create SPL token with desired parameters
3. Initialize Sol-Launch with token mint
4. Deposit tokens to vault (50-80% of supply)
5. Configure anti-sniper parameters
6. Enable trading through Sol-Launch
7. Monitor protected trading period
8. After protection period, add liquidity to Raydium
9. Enable open trading on DEXs
10. Monitor launch continuously

### Security Features
- Protected trading during initial phase
- Progressive limit increases over time
- Anti-bot and anti-scam measures
- Comprehensive event logging
- Authority-controlled operations

## 🤝 Contributing

This is a production-ready system for secure token launches. The code is provided as-is for educational and deployment purposes.

## 📄 License

MIT License - See LICENSE file for details

## ⚠️ Disclaimer

This smart contract system is provided for educational and operational purposes. Users should:
- Thoroughly test on devnet before mainnet deployment
- Understand all security parameters and their implications
- Follow best practices for wallet security
- Have proper backup procedures in place
- Comply with applicable regulations in their jurisdiction

## 🎮 Support

For deployment assistance, refer to the comprehensive documentation in the project repository.

---

**Version**: 1.0.0  
**Status**: Production Ready  
**Network**: Solana Mainnet  
**Program ID**: Generated on deployment