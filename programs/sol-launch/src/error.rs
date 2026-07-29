use anchor_lang::prelude::*;

#[error_code]
pub enum LaunchError {
    #[msg("Only the launch authority can perform this action.")]
    Unauthorized,
    #[msg("Launch parameters must be greater than zero.")]
    InvalidConfig,
    #[msg("Trading is already enabled for this launch.")]
    AlreadyEnabled,
    #[msg("Trading must be enabled before it can be paused.")]
    TradingNotEnabled,
    #[msg("Trading is already disabled for this launch.")]
    AlreadyDisabled,
    #[msg("The new authority must be different from the current authority.")]
    InvalidAuthorityChange,
    #[msg("The launch is already paused.")]
    AlreadyPaused,
    #[msg("The launch is not currently paused.")]
    NotPaused,
    #[msg("The launch is not in the ready state.")]
    NotReady,
    #[msg("Invalid launch phase transition.")]
    InvalidPhaseTransition,
    #[msg("Trade amount exceeds maximum allowed per transaction.")]
    MaxBuyExceeded,
    #[msg("Wallet balance exceeds maximum allowed.")]
    MaxWalletExceeded,
    #[msg("Cooldown period has not elapsed.")]
    CooldownNotElapsed,
    #[msg("Invalid token mint.")]
    InvalidMint,
    #[msg("Vault account mismatch.")]
    VaultMismatch,
    #[msg("Insufficient vault balance.")]
    InsufficientVaultBalance,
    #[msg("Trade amount must be greater than zero.")]
    InvalidTradeAmount,
    #[msg("Overflow in calculation.")]
    Overflow,
    #[msg("Invalid vault authority.")]
    InvalidVaultAuthority,
    #[msg("Withdrawal amount exceeds available balance.")]
    InsufficientBalance,
    #[msg("Withdrawals are currently disabled.")]
    WithdrawalsDisabled,
    #[msg("Invalid user token account.")]
    InvalidUserTokenAccount,
    #[msg("Launch is not in active phase.")]
    NotActive,
    #[msg("Max trades per user exceeded.")]
    MaxTradesExceeded,
    #[msg("Wallet is blacklisted from trading.")]
    WalletBlacklisted,
}