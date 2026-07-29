use anchor_lang::prelude::*;

use crate::{
    state::LaunchState,
    events::TradingResumed,
};

#[derive(Accounts)]
pub struct ResumeTrading<'info> {
    #[account(mut)]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

pub fn resume_trading(
    ctx: Context<ResumeTrading>,
) -> Result<()> {

    let launch = &mut ctx.accounts.launch;

    require_keys_eq!(
        launch.authority,
        ctx.accounts.authority.key(),
        crate::errors::LaunchError::Unauthorized
    );

    launch.paused = false;

let clock = Clock::get()?;

emit!(TradingResumed {
    timestamp: clock.unix_timestamp,
});

    Ok(())
}