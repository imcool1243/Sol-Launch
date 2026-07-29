use anchor_lang::prelude::*;
use crate::error::LaunchError;
use crate::state::LaunchState;

pub fn validate_launch_config(max_buy: u64, max_wallet: u64, cooldown_seconds: i64) -> Result<()> {
    require!(max_buy > 0, LaunchError::InvalidConfig);
    require!(max_wallet > 0, LaunchError::InvalidConfig);
    require!(max_wallet >= max_buy, LaunchError::InvalidConfig);
    require!(cooldown_seconds > 0, LaunchError::InvalidConfig);
    
    Ok(())
}

pub fn validate_authority_change(current_authority: Pubkey, new_authority: Pubkey) -> Result<()> {
    require_keys_neq!(current_authority, new_authority, LaunchError::InvalidAuthorityChange);
    Ok(())
}

pub fn validate_phase_transition(current_phase: u8, new_phase: u8) -> Result<()> {
    // Define valid phase transitions
    let valid_transitions = [
        (1, 2), // READY -> ACTIVE
        (2, 3), // ACTIVE -> PAUSED
        (3, 2), // PAUSED -> ACTIVE
    ];
    
    if current_phase == new_phase {
        return Ok(());
    }
    
    let is_valid = valid_transitions.iter().any(|&(from, to)| from == current_phase && to == new_phase);
    require!(is_valid, LaunchError::InvalidPhaseTransition);
    
    Ok(())
}

pub fn validate_trade_request(
    launch: &LaunchState,
    trade_state: &crate::state::TradeState,
    amount: u64,
    current_timestamp: i64,
) -> Result<()> {
    validate_trade_request_with_limits(
        launch,
        trade_state,
        amount,
        current_timestamp,
        launch.max_buy,
        launch.max_wallet,
    )
}

pub fn validate_trade_request_with_limits(
    launch: &LaunchState,
    trade_state: &crate::state::TradeState,
    amount: u64,
    current_timestamp: i64,
    current_max_buy: u64,
    current_max_wallet: u64,
) -> Result<()> {
    require!(launch.trading_enabled, LaunchError::TradingNotEnabled);
    require!(launch.phase == 2, LaunchError::TradingNotEnabled); // ACTIVE phase
    require!(amount > 0, LaunchError::InvalidTradeAmount);
    require!(amount <= current_max_buy, LaunchError::MaxBuyExceeded);
    
    // Check for overflow
    let new_total = trade_state.total_traded.checked_add(amount)
        .ok_or(LaunchError::Overflow)?;
    require!(new_total <= current_max_wallet, LaunchError::MaxWalletExceeded);
    
    // Check cooldown (only if there was a previous trade)
    if trade_state.trade_count > 0 {
        let cooldown_end = trade_state.last_trade_timestamp.checked_add(launch.cooldown_seconds)
            .ok_or(LaunchError::Overflow)?;
        require!(current_timestamp >= cooldown_end, LaunchError::CooldownNotElapsed);
        
        // Anti-bot: check for excessive trade frequency
        let time_since_last_trade = current_timestamp.saturating_sub(trade_state.last_trade_timestamp);
        // Require at least 1 second between trades to prevent high-frequency botting
        require!(time_since_last_trade >= 1, LaunchError::CooldownNotElapsed);
    }
    
    Ok(())
}

pub fn validate_token_mint(expected_mint: Pubkey, actual_mint: Pubkey) -> Result<()> {
    require_keys_eq!(expected_mint, actual_mint, LaunchError::InvalidMint);
    Ok(())
}

pub fn validate_vault(expected_vault: Pubkey, actual_vault: Pubkey) -> Result<()> {
    require_keys_eq!(expected_vault, actual_vault, LaunchError::VaultMismatch);
    Ok(())
}