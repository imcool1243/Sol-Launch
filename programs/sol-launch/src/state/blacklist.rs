use anchor_lang::prelude::*;
use crate::error::LaunchError;

#[account]
pub struct BlacklistState {
    pub launch: Pubkey,
    pub blacklisted_wallets: Vec<Pubkey>,
    pub blacklist_authority: Pubkey,
    pub last_updated: i64,
    pub bump: u8,
}

impl BlacklistState {
    pub const SPACE: usize = 8 + // discriminator
        32 + // launch
        4 + 32 * 50 + // blacklisted_wallets (up to 50 wallets)
        32 + // blacklist_authority
        8 +  // last_updated
        1;   // bump
    
    pub fn is_wallet_blacklisted(&self, wallet: &Pubkey) -> bool {
        self.blacklisted_wallets.contains(wallet)
    }
    
    pub fn add_wallet(&mut self, wallet: Pubkey) -> Result<()> {
        require!(!self.is_wallet_blacklisted(&wallet), LaunchError::WalletBlacklisted);
        require!(self.blacklisted_wallets.len() < 50, LaunchError::Overflow);
        
        self.blacklisted_wallets.push(wallet);
        self.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    pub fn remove_wallet(&mut self, wallet: Pubkey) -> Result<()> {
        if let Some(pos) = self.blacklisted_wallets.iter().position(|&w| w == wallet) {
            self.blacklisted_wallets.remove(pos);
            self.last_updated = Clock::get()?.unix_timestamp;
            Ok(())
        } else {
            Err(LaunchError::WalletBlacklisted.into())
        }
    }
}