use anchor_lang::prelude::*;

use crate::state::LaunchState;

events::TradingPaused,

#[derive(Accounts)]
pub struct PauseTrading<'info> {
    #[account(mut)]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

pub fn pause_trading(
    ctx: Context<PauseTrading>,
) -> Result<()> {

    let launch = &mut ctx.accounts.launch;

    require_keys_eq!(
        launch.authority,
        ctx.accounts.authority.key(),
        crate::errors::LaunchError::Unauthorized
    );

    launch.paused = true;

    Ok(())
}