use anchor_lang::prelude::*;

use crate::{
    state::LaunchState,
    events::TradingEnabled,
};

#[derive(Accounts)]
pub struct EnableTrading<'info> {
    #[account(mut)]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

pub fn enable_trading(
    ctx: Context<EnableTrading>,
) -> Result<()> {

    let launch = &mut ctx.accounts.launch;

    require_keys_eq!(
        launch.authority,
        ctx.accounts.authority.key(),
        crate::errors::LaunchError::Unauthorized
    );

   launch.trading_enabled = true;

let clock = Clock::get()?;

emit!(TradingEnabled {
    timestamp: clock.unix_timestamp,
});

Ok(()) 

Ok(())

    Ok(())
}