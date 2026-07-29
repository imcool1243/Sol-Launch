use anchor_lang::prelude::*;

#[account]
pub struct LaunchState {
    pub authority: Pubkey,
    pub trading_enabled: bool,
    pub phase: u8,
    pub max_buy: u64,
    pub max_wallet: u64,
    pub cooldown_seconds: i64,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub launch_bump: u8,
    pub start_timestamp: i64,
    pub total_supply: u64,
    pub total_traded: u64,
    pub sniper_protection_enabled: bool,
    pub min_trading_duration: i64,
    // Enhanced anti-sniper features
    pub wallet_blacklist_enabled: bool,
    pub progressive_limits_enabled: bool,
    pub initial_max_buy: u64,
    pub initial_max_wallet: u64,
    pub limit_increase_interval: i64,
    pub limit_increase_multiplier: u64,
    pub anti_scam_enabled: bool,
    pub max_trades_per_user: u64,
    pub total_traders: u64,
}

impl LaunchState {
    pub const SPACE: usize = 8 + // discriminator
        32 + // authority
        1 +  // trading_enabled
        1 +  // phase
        8 +  // max_buy
        8 +  // max_wallet
        8 +  // cooldown_seconds
        32 + // token_mint
        32 + // vault
        1 +  // vault_bump
        1 +  // launch_bump
        8 +  // start_timestamp
        8 +  // total_supply
        8 +  // total_traded
        1 +  // sniper_protection_enabled
        8 +  // min_trading_duration
        1 +  // wallet_blacklist_enabled
        1 +  // progressive_limits_enabled
        8 +  // initial_max_buy
        8 +  // initial_max_wallet
        8 +  // limit_increase_interval
        8 +  // limit_increase_multiplier
        1 +  // anti_scam_enabled
        8 +  // max_trades_per_user
        8;   // total_traders
    
    pub fn get_current_max_buy(&self, current_timestamp: i64) -> u64 {
        if !self.progressive_limits_enabled {
            return self.max_buy;
        }
        
        let elapsed = current_timestamp.saturating_sub(self.start_timestamp);
        if elapsed <= 0 {
            return self.initial_max_buy;
        }
        
        let intervals_elapsed = elapsed / self.limit_increase_interval;
        let multiplier = (intervals_elapsed as u64).saturating_mul(self.limit_increase_multiplier);
        
        // Progressive increase: initial * (1 + multiplier)
        let base = self.initial_max_buy;
        let increase = base.saturating_mul(multiplier);
        let current = base.saturating_add(increase);
        
        // Cap at final max_buy
        current.min(self.max_buy)
    }
    
    pub fn get_current_max_wallet(&self, current_timestamp: i64) -> u64 {
        if !self.progressive_limits_enabled {
            return self.max_wallet;
        }
        
        let elapsed = current_timestamp.saturating_sub(self.start_timestamp);
        if elapsed <= 0 {
            return self.initial_max_wallet;
        }
        
        let intervals_elapsed = elapsed / self.limit_increase_interval;
        let multiplier = (intervals_elapsed as u64).saturating_mul(self.limit_increase_multiplier);
        
        // Progressive increase: initial * (1 + multiplier)
        let base = self.initial_max_wallet;
        let increase = base.saturating_mul(multiplier);
        let current = base.saturating_add(increase);
        
        // Cap at final max_wallet
        current.min(self.max_wallet)
    }
}