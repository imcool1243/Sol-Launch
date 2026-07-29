use anchor_lang::prelude::*;

/// Event emitted when a new launch is successfully initialized.
/// 
/// This event contains comprehensive information about the launch configuration
/// and serves as an audit trail for launch initialization.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `authority` - The public key of the launch authority who can control the launch
/// * `token_mint` - The public key of the token mint being launched
/// * `vault` - The public key of the vault holding the launch tokens
/// * `max_buy` - Maximum tokens allowed per transaction
/// * `max_wallet` - Maximum tokens allowed per wallet
/// * `cooldown_seconds` - Minimum seconds between trades for a user
/// * `timestamp` - Unix timestamp when the launch was initialized
#[event]
pub struct LaunchInitialized {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub max_buy: u64,
    pub max_wallet: u64,
    pub cooldown_seconds: i64,
    pub timestamp: i64,
}

/// Event emitted when trading is enabled for a launch.
/// 
/// This event marks the transition from the ready phase to the active trading phase,
/// allowing users to start trading tokens through the protected trading system.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `authority` - The public key of the authority who enabled trading
/// * `timestamp` - Unix timestamp when trading was enabled
#[event]
pub struct TradingEnabled {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Event emitted when trading is disabled for a launch.
/// 
/// This event marks the transition from active trading to paused state,
/// which can be used for emergency stops or controlled trading halts.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `authority` - The public key of the authority who disabled trading
/// * `timestamp` - Unix timestamp when trading was disabled
#[event]
pub struct TradingDisabled {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Event emitted when a launch is paused.
/// 
/// This event indicates that the launch has been temporarily paused,
/// preventing any trading activity while the launch is in the paused state.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `authority` - The public key of the authority who paused the launch
/// * `timestamp` - Unix timestamp when the launch was paused
#[event]
pub struct LaunchPaused {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Event emitted when a paused launch is resumed.
/// 
/// This event indicates that a previously paused launch has been resumed,
/// allowing trading activity to continue from the paused state.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `authority` - The public key of the authority who resumed the launch
/// * `timestamp` - Unix timestamp when the launch was resumed
#[event]
pub struct LaunchResumed {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Event emitted when launch authority is transferred to a new wallet.
/// 
/// This event provides an audit trail for authority changes, which is important
/// for security and governance purposes.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `old_authority` - The public key of the previous authority
/// * `new_authority` - The public key of the new authority
/// * `timestamp` - Unix timestamp when the authority was transferred
#[event]
pub struct AuthorityTransferred {
    pub launch: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

/// Event emitted when tokens are deposited to the launch vault.
/// 
/// This event tracks token deposits to the vault, providing transparency
/// about the token supply available for trading.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `vault` - The public key of the vault receiving the tokens
/// * `from` - The public key of the wallet depositing the tokens
/// * `amount` - The number of tokens deposited
/// * `timestamp` - Unix timestamp when the deposit occurred
#[event]
pub struct TokensDeposited {
    pub launch: Pubkey,
    pub vault: Pubkey,
    pub from: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

/// Event emitted when tokens are withdrawn from the launch vault.
/// 
/// This event tracks token withdrawals from the vault, providing transparency
/// about token movements from the launch system.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `vault` - The public key of the vault the tokens were withdrawn from
/// * `to` - The public key of the wallet receiving the tokens
/// * `amount` - The number of tokens withdrawn
/// * `timestamp` - Unix timestamp when the withdrawal occurred
#[event]
pub struct TokensWithdrawn {
    pub launch: Pubkey,
    pub vault: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

/// Event emitted when a trade is successfully executed.
/// 
/// This event tracks individual trades, providing transparency about trading
/// activity and helping to monitor the launch's progress.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `trader` - The public key of the wallet executing the trade
/// * `amount` - The number of tokens traded in this transaction
/// * `total_traded` - The cumulative total tokens traded by this trader
/// * `timestamp` - Unix timestamp when the trade was executed
#[event]
pub struct TradeExecuted {
    pub launch: Pubkey,
    pub trader: Pubkey,
    pub amount: u64,
    pub total_traded: u64,
    pub timestamp: i64,
}

/// Event emitted when a launch is started with a delay.
/// 
/// This event indicates that a launch has been initiated with a specified delay
/// before trading becomes available, allowing for controlled launch timing.
/// 
/// # Fields
/// * `launch` - The public key of the launch account
/// * `authority` - The public key of the authority who started the launch
/// * `delay_seconds` - The number of seconds until trading becomes available
/// * `timestamp` - Unix timestamp when the launch was started
#[event]
pub struct LaunchStarted {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub delay_seconds: i64,
    pub timestamp: i64,
}