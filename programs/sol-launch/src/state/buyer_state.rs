use anchor_lang::prelude::*;

#[account]
pub struct TradeState {
    pub last_trade_timestamp: i64,
    pub total_traded: u64,
    pub trade_count: u64,
    pub bump: u8,
}

impl TradeState {
    pub const SPACE: usize = 8 + // discriminator
        8 + // last_trade_timestamp
        8 + // total_traded
        8 + // trade_count
        1;  // bump
}