use anchor_lang::prelude::*;

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

#[event]
pub struct TradingEnabled {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TradingDisabled {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct LaunchPaused {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct LaunchResumed {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AuthorityTransferred {
    pub launch: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensDeposited {
    pub launch: Pubkey,
    pub vault: Pubkey,
    pub from: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensWithdrawn {
    pub launch: Pubkey,
    pub vault: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TradeExecuted {
    pub launch: Pubkey,
    pub trader: Pubkey,
    pub amount: u64,
    pub total_traded: u64,
    pub timestamp: i64,
}

#[event]
pub struct LaunchStarted {
    pub launch: Pubkey,
    pub authority: Pubkey,
    pub delay_seconds: i64,
    pub timestamp: i64,
}