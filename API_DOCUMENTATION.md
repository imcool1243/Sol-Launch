# Sol-Launch API Documentation

## Overview

This document provides comprehensive API documentation for the Sol-Launch smart contract, including instruction details, account structures, events, and error codes.

## Program Information

- **Program ID**: `2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj`
- **Network**: Solana Devnet (configurable for Mainnet)
- **Framework**: Anchor 1.1.2
- **Language**: Rust

---

## Instructions

### 1. Initialize Launch

Initializes a new token launch with specified parameters.

#### Accounts
- `launch`: The launch state account (signer, writable)
- `authority`: The authority wallet (signer, writable)
- `tokenMint`: The token mint account (readonly)
- `vault`: The token vault account (signer, writable)
- `tokenProgram`: SPL Token program (readonly)
- `systemProgram`: System program (readonly)

#### Parameters
- `max_buy` (u64): Maximum tokens per transaction
- `max_wallet` (u64): Maximum tokens per wallet
- `cooldown_seconds` (i64): Cooldown period between trades
- `total_supply` (u64): Total token supply
- `sniper_protection_enabled` (bool): Enable sniper protection
- `min_trading_duration` (i64): Minimum trading duration

#### Returns
- Event: `LaunchInitialized`

#### Example
```typescript
const tx = await program.methods
  .initializeLaunch(
    new BN(1000),   // max_buy
    new BN(5000),   // max_wallet
    new BN(60),     // cooldown_seconds
    new BN(1000000), // total_supply
    true,           // sniper_protection_enabled
    new BN(300)     // min_trading_duration
  )
  .accounts({
    launch: launchKeypair.publicKey,
    authority: provider.wallet.publicKey,
    tokenMint: tokenMintPubkey,
    vault: vaultKeypair.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SYSTEM_PROGRAM_ID,
  })
  .signers([launchKeypair, vaultKeypair])
  .rpc();
```

---

### 2. Deposit Tokens

Deposits tokens into the vault from the authority's wallet.

#### Accounts
- `authority`: The authority wallet (signer, writable)
- `launch`: The launch state account (writable)
- `from`: Source token account (writable)
- `vault`: The vault token account (writable)
- `tokenProgram`: SPL Token program (readonly)

#### Parameters
- `amount` (u64): Number of tokens to deposit

#### Returns
- Event: `TokensDeposited`

#### Example
```typescript
const tx = await program.methods
  .depositTokens(new BN(amount))
  .accounts({
    authority: provider.wallet.publicKey,
    launch: launchPubkey,
    from: fromTokenAccount,
    vault: vaultPubkey,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

---

### 3. Withdraw Tokens

Withdraws tokens from the vault to the authority's wallet.

#### Accounts
- `authority`: The authority wallet (signer, writable)
- `launch`: The launch state account (writable)
- `vault`: The vault token account (writable)
- `to`: Destination token account (writable)
- `tokenProgram`: SPL Token program (readonly)

#### Parameters
- `amount` (u64): Number of tokens to withdraw

#### Returns
- Event: `TokensWithdrawn`

#### Example
```typescript
const tx = await program.methods
  .withdrawTokens(new BN(amount))
  .accounts({
    authority: provider.wallet.publicKey,
    launch: launchPubkey,
    vault: vaultPubkey,
    to: toTokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

---

### 4. Enable Trading

Enables trading for the launch.

#### Accounts
- `authority`: The authority wallet (signer, writable)
- `launch`: The launch state account (writable)

#### Parameters
- None

#### Returns
- Event: `TradingEnabled`

#### Example
```typescript
const tx = await program.methods
  .enableTrading()
  .accounts({
    authority: provider.wallet.publicKey,
    launch: launchPubkey,
  })
  .rpc();
```

---

### 5. Disable Trading

Disables trading for the launch.

#### Accounts
- `authority`: The authority wallet (signer, writable)
- `launch`: The launch state account (writable)

#### Parameters
- None

#### Returns
- Event: `TradingDisabled`

#### Example
```typescript
const tx = await program.methods
  .disableTrading()
  .accounts({
    authority: provider.wallet.publicKey,
    launch: launchPubkey,
  })
  .rpc();
```

---

### 6. Execute Trade

Executes a token trade from the vault to the user's wallet.

#### Accounts
- `launch`: The launch state account (writable)
- `tradeState`: The user's trade state account (signer, writable)
- `vault`: The vault token account (writable)
- `userToken`: The user's token account (writable)
- `authority`: The trader's wallet (signer, writable)
- `tokenProgram`: SPL Token program (readonly)
- `systemProgram`: System program (readonly)

#### Parameters
- `amount` (u64): Number of tokens to trade

#### Returns
- Event: `TradeExecuted`

#### Example
```typescript
const [tradeStatePubkey] = await PublicKey.findProgramAddress(
  [
    Buffer.from('trade'),
    provider.wallet.publicKey.toBuffer(),
    launchPubkey.toBuffer(),
  ],
  PROGRAM_ID
);

const tx = await program.methods
  .executeTrade(new BN(amount))
  .accounts({
    launch: launchPubkey,
    tradeState: tradeStatePubkey,
    vault: vaultPubkey,
    userToken: userTokenAccount,
    authority: provider.wallet.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SYSTEM_PROGRAM_ID,
  })
  .rpc();
```

---

## State Structures

### LaunchState

The main account representing a token launch.

#### Fields
- `authority` (Pubkey): Launch authority wallet
- `trading_enabled` (bool): Whether trading is enabled
- `phase` (u8): Current launch phase (1=Ready, 2=Active, 3=Paused)
- `max_buy` (u64): Maximum tokens per transaction
- `max_wallet` (u64): Maximum tokens per wallet
- `cooldown_seconds` (i64): Cooldown period between trades
- `token_mint` (Pubkey): Associated token mint
- `vault` (Pubkey): Vault token account
- `vault_bump` (u8): PDA bump seed for vault
- `launch_bump` (u8): PDA bump seed for launch
- `start_timestamp` (i64): Trading start timestamp
- `total_supply` (u64): Total token supply
- `total_traded` (u64): Total tokens traded
- `sniper_protection_enabled` (bool): Sniper protection status
- `min_trading_duration` (i64): Minimum trading duration

#### Space
- 157 bytes (including discriminator)

#### PDA Seeds
- `["launch", authority]`

### TradeState

The account representing a user's trading state.

#### Fields
- `last_trade_timestamp` (i64): Last trade timestamp
- `total_traded` (u64): Total tokens purchased by user
- `trade_count` (u64): Number of trades executed
- `bump` (u8): PDA bump seed

#### Space
- 33 bytes (including discriminator)

#### PDA Seeds
- `["trade", authority, launch]`

---

## Events

### LaunchInitialized
Emitted when a new launch is created.

#### Fields
- `launch` (Pubkey): Launch public key
- `authority` (Pubkey): Authority public key
- `token_mint` (Pubkey): Token mint
- `vault` (Pubkey): Vault public key
- `max_buy` (u64): Maximum buy amount
- `max_wallet` (u64): Maximum wallet amount
- `cooldown_seconds` (i64): Cooldown period
- `timestamp` (i64): Event timestamp

### TokensDeposited
Emitted when tokens are deposited to vault.

#### Fields
- `launch` (Pubkey): Launch public key
- `vault` (Pubkey): Vault public key
- `from` (Pubkey): Source account
- `amount` (u64): Deposit amount
- `timestamp` (i64): Event timestamp

### TokensWithdrawn
Emitted when tokens are withdrawn from vault.

#### Fields
- `launch` (Pubkey): Launch public key
- `vault` (Pubkey): Vault public key
- `to` (Pubkey): Destination account
- `amount` (u64): Withdrawal amount
- `timestamp` (i64): Event timestamp

### TradingEnabled
Emitted when trading is enabled.

#### Fields
- `launch` (Pubkey): Launch public key
- `authority` (Pubkey): Authority public key
- `timestamp` (i64): Event timestamp

### TradingDisabled
Emitted when trading is disabled.

#### Fields
- `launch` (Pubkey): Launch public key
- `authority` (Pubkey): Authority public key
- `timestamp` (i64): Event timestamp

### TradeExecuted
Emitted when a trade is executed.

#### Fields
- `launch` (Pubkey): Launch public key
- `trader` (Pubkey): Trader's public key
- `amount` (u64): Trade amount
- `total_traded` (u64): User's total traded
- `timestamp` (i64): Event timestamp

---

## Error Codes

### Unauthorized
**Code**: 0x1770  
**Description**: Operation not authorized  
**Trigger**: Non-authority attempts sensitive operation

### InvalidConfig
**Code**: 0x1771  
**Description**: Invalid configuration parameter  
**Trigger**: Invalid max_buy, max_wallet, or cooldown

### TradingNotEnabled
**Code**: 0x1772  
**Description**: Trading is not enabled  
**Trigger**: Trade attempt when trading disabled

### InvalidTradeAmount
**Code**: 0x1773  
**Description**: Invalid trade amount  
**Trigger**: Zero or negative trade amount

### MaxBuyExceeded
**Code**: 0x1774  
**Description**: Trade amount exceeds max buy  
**Trigger**: Trade amount > max_buy

### MaxWalletExceeded
**Code**: 0x1775  
**Description**: Trade would exceed max wallet  
**Trigger**: User total + amount > max_wallet

### CooldownNotElapsed
**Code**: 0x1776  
**Description**: Cooldown period not elapsed  
**Trigger**: Trade attempt before cooldown period

### InsufficientVaultBalance
**Code**: 0x1777  
**Description**: Vault has insufficient balance  
**Trigger**: Trade/withdraw amount > vault balance

### InvalidMint
**Code**: 0x1778  
**Description**: Invalid token mint  
**Trigger**: Token mint mismatch

### VaultMismatch
**Code**: 0x1779  
**Description**: Vault account mismatch  
**Trigger**: Wrong vault account provided

### InvalidVaultAuthority
**Code**: 0x177a  
**Description**: Invalid vault authority  
**Trigger**: Vault authority validation failed

### InvalidUserTokenAccount
**Code**: 0x177b  
**Description**: Invalid user token account  
**Trigger**: User token account validation failed

### InvalidPhaseTransition
**Code**: 0x177c  
**Description**: Invalid phase transition  
**Trigger**: Attempted invalid phase change

### NotReady
**Code**: 0x177d  
**Description**: Launch not ready for operation  
**Trigger**: Operation attempted too early

### AlreadyEnabled
**Code**: 0x177e  
**Description**: Feature already enabled  
**Trigger**: Enable already-enabled feature

### Overflow
**Code**: 0x177f  
**Description**: Arithmetic overflow  
**Trigger**: Number overflow in calculation

---

## Constants

### Phase Constants
- `LAUNCH_PHASE_READY`: 1
- `LAUNCH_PHASE_ACTIVE`: 2
- `LAUNCH_PHASE_PAUSED`: 3

### Seed Constants
- `LAUNCH_SEED`: "launch"
- `VAULT_SEED`: "vault"
- `TRADE_SEED`: "trade"

### Program IDs
- `TOKEN_PROGRAM_ID`: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
- `SYSTEM_PROGRAM_ID`: 11111111111111111111111111111111111112

---

## Integration Examples

### JavaScript/TypeScript

```typescript
import { Program, AnchorProvider, web3, BN } from '@project-serum/anchor';
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection('https://api.devnet.solana.com');
const provider = new AnchorProvider(connection, wallet, { commitment: 'confirmed' });
const program = new Program(idl, programId, provider);

// Initialize launch
const launchKeypair = web3.Keypair.generate();
const vaultKeypair = web3.Keypair.generate();

await program.methods
  .initializeLaunch(
    new BN(1000),
    new BN(5000),
    new BN(60),
    new BN(1000000),
    true,
    new BN(300)
  )
  .accounts({
    launch: launchKeypair.publicKey,
    authority: provider.wallet.publicKey,
    tokenMint: new PublicKey('...'),
    vault: vaultKeypair.publicKey,
    tokenProgram: new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
    systemProgram: new PublicKey('11111111111111111111111111111111111112'),
  })
  .signers([launchKeypair, vaultKeypair])
  .rpc();
```

### Python

```python
from solana.rpc.api import Client
from solana.publickey import PublicKey
from anchorpy import AnchorProgram, Context, accounts

client = Client("https://api.devnet.solana.com")
program = AnchorProgram(client, program_id, idl)

# Initialize launch
launch_keypair = Keypair()
vault_keypair = Keypair()

program.rpc["initialize_launch"](
    Context(
        accounts={
            "launch": launch_keypair.public_key,
            "authority": provider.wallet.public_key,
            "token_mint": token_mint_pubkey,
            "vault": vault_keypair.public_key,
            "token_program": TOKEN_PROGRAM_ID,
            "system_program": SYSTEM_PROGRAM_ID,
        },
        signers=[launch_keypair, vault_keypair],
    ),
    1000,  # max_buy
    5000,  # max_wallet
    60,   # cooldown_seconds
    1000000,  # total_supply
    True,  # sniper_protection_enabled
    300,   # min_trading_duration
)
```

---

## Testing

### Unit Tests
Run unit tests with:
```bash
anchor test
```

### Integration Tests
Run integration tests on devnet:
```bash
anchor test --skip-local-validator
```

### Event Monitoring
Monitor program events:
```bash
solana logs 2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj
```

---

## Rate Limits

### Solana Rate Limits
- **TPS**: ~4,000-7,000 transactions per second
- **CPUs**: 200,000 compute units per transaction
- **Account Size**: Account data size affects fees

### Program-Specific Limits
- **Max Buy**: Configurable per launch
- **Max Wallet**: Configurable per launch
- **Cooldown**: Configurable per launch
- **Anti-Bot**: 1-second minimum between trades

---

## Security Considerations

### PDA Security
- All vault operations use PDA signing
- Vault owned by launch PDA, not authority wallet
- Prevents unauthorized vault access

### Access Control
- Authority-only operations
- Phase-based access control
- Comprehensive account validation

### Overflow Protection
- All arithmetic uses checked operations
- Prevents integer overflow attacks
- Safe financial operations

---

## Support

For API-related issues:
- Review this documentation
- Check program logs: `solana logs <PROGRAM_ID>`
- Refer to USER_GUIDE.md for usage instructions
- Check SECURITY_AUDIT_REPORT.md for security details