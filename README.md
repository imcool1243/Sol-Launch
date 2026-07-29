# Sol-Launch: Secure Single Token Launch Platform

A production-ready Solana smart contract for secure, fair single token launches with comprehensive anti-sniper protection.

## Overview

Sol-Launch is a specialized secure token launch platform built on Solana using the Anchor framework. It provides fair token distribution, advanced anti-sniper protection, and comprehensive security features to ensure safe, single-token launches.

## Mission

**Core Purpose**: Create a secure Solana token launch system that prevents sniper bots and unfair early accumulation for a SINGLE token launch.

## Features

### Core Functionality
- **Single Token Launch**: Designed for one secure token launch
- **Launch Management**: Initialize, configure, and manage token launches
- **Vault System**: Secure PDA-based token vault with authority controls
- **Trading System**: Fair trading with advanced cooldowns and limits
- **Phase Management**: Control launch phases (Ready → Active → Paused)

### Advanced Anti-Sniper Protection
- **Progressive Limits**: Gradually increase buy/wallet limits over time
- **Anti-Scam Protection**: Limit maximum trades per user
- **Wallet Blacklist**: Block specific wallets from trading
- **Minimum Trading Duration**: Prevent early trading disable attacks
- **Time-based Controls**: Configurable cooldowns and time limits
- **Bot Resistance**: 1-second minimum between trades

### Security Features
- **PDA Vault Security**: Program-derived address vault ownership
- **Authority Controls**: Comprehensive authority validation
- **Overflow Protection**: Checked arithmetic throughout
- **Account Validation**: Comprehensive account security checks
- **Event Logging**: Complete audit trail for transparency

## Architecture

### State Structures

#### LaunchState (Enhanced)
- Authority: Owner of the launch
- Trading Enabled: Trading status
- Phase: Current launch phase
- Max Buy: Maximum per-transaction buy amount
- Max Wallet: Maximum per-wallet holding amount
- Cooldown Seconds: Minimum time between trades
- Token Mint: Associated token mint
- Vault: Token vault address
- Start Timestamp: Trading start time
- Total Supply: Total token supply
- Total Traded: Total tokens traded
- **Enhanced Features**:
  - Wallet Blacklist Enabled: Enable wallet blocking
  - Progressive Limits Enabled: Enable progressive limit increases
  - Initial Max Buy/Wallet: Starting limits for progressive mode
  - Limit Increase Interval: Time between limit increases
  - Limit Increase Multiplier: Multiplier for limit increases
  - Anti-Scam Enabled: Limit trades per user
  - Max Trades Per User: Maximum trades allowed per user
  - Total Traders: Track total unique traders

#### TradeState
- Last Trade Timestamp: Last trade time
- Total Traded: Total tokens purchased by user
- Trade Count: Number of trades executed
- Bump: PDA bump seed

#### BlacklistState (New)
- Launch: Associated launch
- Blacklisted Wallets: Array of blacklisted wallet addresses
- Blacklist Authority: Authority for blacklist management
- Last Updated: Last update timestamp
- Bump: PDA bump seed

### Instructions

#### Initialize Launch (Enhanced)
```rust
pub fn initialize_launch(
    ctx: Context<InitializeLaunch>,
    max_buy: u64,
    max_wallet: u64,
    cooldown_seconds: i64,
    total_supply: u64,
    sniper_protection_enabled: bool,
    min_trading_duration: i64,
    // Enhanced anti-sniper parameters
    wallet_blacklist_enabled: bool,
    progressive_limits_enabled: bool,
    initial_max_buy: u64,
    initial_max_wallet: u64,
    limit_increase_interval: i64,
    limit_increase_multiplier: u64,
    anti_scam_enabled: bool,
    max_trades_per_user: u64,
) -> Result<()>
```

#### Deposit Tokens
```rust
pub fn deposit_tokens(
    ctx: Context<DepositTokens>,
    amount: u64,
) -> Result<()>
```

#### Withdraw Tokens
```rust
pub fn withdraw_tokens(
    ctx: Context<WithdrawTokens>,
    amount: u64,
) -> Result<()>
```

#### Enable Trading
```rust
pub fn enable_trading(ctx: Context<EnableTrading>) -> Result<()>
```

#### Disable Trading
```rust
pub fn disable_trading(ctx: Context<DisableTrading>) -> Result<()>
```

#### Execute Trade (Enhanced)
```rust
pub fn execute_trade(
    ctx: Context<ExecuteTrade>,
    amount: u64,
) -> Result<()>
```
- Uses progressive limits for validation
- Enforces anti-scam max trades per user
- Tracks total traders for analytics
- 1-second minimum between trades (anti-bot)

## Progressive Limits System

The progressive limits system gradually increases buy and wallet limits over time to discourage early sniping:

- **Initial Phase**: Lower limits for early participants
- **Gradual Increase**: Limits increase at configured intervals
- **Cap**: Limits cap at final configured values
- **Formula**: `current_limit = initial_limit * (1 + multiplier * intervals_elapsed)`

Example configuration:
- Initial Max Buy: 500 tokens
- Final Max Buy: 1000 tokens
- Increase Interval: 300 seconds (5 minutes)
- Multiplier: 1 (1x increase per interval)

## Security Features

### 1. Progressive Limits
- Starts with lower buy/wallet limits
- Gradually increases over time
- Discourages early sniping
- Fairer distribution

### 2. Anti-Scam Protection
- Limits trades per user
- Prevents bot accumulation
- Configurable per-user limits
- Protects against coordinated attacks

### 3. Wallet Blacklist
- Block known bot/scammer wallets
- Authority-controlled management
- Up to 50 blacklisted wallets
- Real-time enforcement

### 4. Minimum Trading Duration
- Prevents early trading disable
- Ensures fair distribution period
- Configurable minimum duration
- Protects against pump-and-dump

### 5. Bot Resistance
- 1-second minimum between trades
- Combined with configurable cooldown
- Prevents high-frequency trading
- Additional behavioral protections

### 6. PDA Vault Security
- Vault owned by launch PDA (not authority wallet)
- Secure token transfers using PDA signing
- Prevents unauthorized vault access

### 7. Authority Controls
- Authority-only operations
- Authority change validation
- Comprehensive permission checks

## Phases

### Phase Constants
- `LAUNCH_PHASE_READY` (1): Launch ready state
- `LAUNCH_PHASE_ACTIVE` (2): Trading active
- `LAUNCH_PHASE_PAUSED` (3): Trading paused

### Valid Transitions
- READY → ACTIVE
- ACTIVE → PAUSED
- PAUSED → ACTIVE

## Events

The program emits comprehensive events for all operations:
- `LaunchInitialized`: Launch creation
- `TokensDeposited`: Token deposits to vault
- `TokensWithdrawn`: Token withdrawals from vault
- `TradingEnabled`: Trading activation
- `TradingDisabled`: Trading deactivation
- `LaunchStarted`: Launch with delay
- `TradeExecuted`: Trade execution

## Error Handling

Comprehensive error types:
- `Unauthorized`: Authority validation failures
- `InvalidConfig`: Configuration errors
- `TradingNotEnabled`: Trading status errors
- `InvalidTradeAmount`: Invalid trade parameters
- `MaxBuyExceeded`: Per-transaction limit exceeded
- `MaxWalletExceeded`: Per-wallet limit exceeded
- `CooldownNotElapsed`: Cooldown period not elapsed
- `InsufficientVaultBalance`: Vault balance insufficient
- `InvalidMint`: Token mint mismatch
- `VaultMismatch`: Vault account mismatch
- `InvalidVaultAuthority`: Vault authority validation
- `InvalidUserTokenAccount`: User token account validation
- `InvalidPhaseTransition`: Invalid phase change
- `NotReady`: Launch not ready for operation
- `AlreadyEnabled`: Operation already enabled
- `Overflow`: Arithmetic overflow
- `MaxTradesExceeded`: User exceeded trade limit
- `WalletBlacklisted`: Wallet is blacklisted

## Testing

### Unit Tests
- Configuration validation
- Trade request validation with progressive limits
- Phase transitions
- Anti-bot rate limiting
- Security feature validation
- State structure validation
- Space calculation verification
- Progressive limits calculation

### Test Results
- **Status**: ✅ All passing (14/14)
- **Coverage**: Comprehensive security and functionality testing

## Development

### Build
```bash
anchor build
```

### Test
```bash
anchor test
```

### Deploy
```bash
anchor deploy
```

## Project Structure

```
sol-launch/
├── programs/
│   └── sol-launch/
│       ├── src/
│       │   ├── lib.rs              # Main program logic
│       │   ├── constants.rs        # Program constants
│       │   ├── error.rs            # Error definitions
│       │   ├── events.rs           # Event definitions
│       │   ├── instructions.rs     # Instruction implementations
│       │   ├── state/              # State structures
│       │   │   ├── launch_state.rs  # Enhanced launch state
│       │   │   ├── buyer_state.rs   # Trade state
│       │   │   ├── blacklist.rs     # Wallet blacklist
│       │   │   └── mod.rs
│       │   └── utils.rs            # Utility functions
│       ├── Cargo.toml
│       └── tests/
├── frontend/
│   ├── src/
│   │   ├── components/             # React components
│   │   ├── utils/                  # Solana client wrapper
│   │   ├── idl/                    # Program IDL
│   │   ├── App.jsx                 # Main application
│   │   └── main.jsx                # Entry point
├── Anchor.toml
├── Cargo.toml
└── README.md
```

## Program ID

```
2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj
```

## Dependencies

- anchor-lang >= 1.1.2
- anchor-spl >= 1.1.2
- solana-program >= 1.16.0
- solana-sdk >= 1.16.0

## License

MIT

## Version

0.2.0 - Enhanced Anti-Sniper Protection

## Deployment Guide

For detailed deployment instructions, see `DEPLOYMENT.md`.

## User Guide

For comprehensive user documentation, see `USER_GUIDE.md`.

## Security Audit

For security audit details, see `SECURITY_AUDIT_REPORT.md`.