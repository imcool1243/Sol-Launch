use anchor_lang::prelude::*;

use crate::{
    state::LaunchState,
    events::LaunchInitialized,
};

#[derive(Accounts)]
pub struct InitializeLaunch<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + LaunchState::LEN
    )]
    pub launch: Account<'info, LaunchState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_launch(
    ctx: Context<InitializeLaunch>,
    max_buy: u64,
    max_wallet: u64,
    cooldown_seconds: i64,
    protection_window: i64,
) -> Result<()> {

    let launch = &mut ctx.accounts.launch;

    launch.authority = ctx.accounts.authority.key();
    launch.trading_enabled = false;
    launch.paused = false;
    launch.max_buy = max_buy;
    launch.max_wallet = max_wallet;
    launch.cooldown_seconds = cooldown_seconds;
    launch.launch_timestamp = 0;
    launch.protection_window = protection_window;

    emit!(LaunchInitialized {
    authority: launch.authority,
});

    Ok(())
}