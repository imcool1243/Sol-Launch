use anchor_lang::prelude::*;

use crate::{
    state::LaunchState,
    events::ConfigUpdated,
};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

pub fn update_config(
    ctx: Context<UpdateConfig>,
    max_buy: u64,
    max_wallet: u64,
    cooldown_seconds: i64,
    protection_window: i64,
) -> Result<()> {

    let launch = &mut ctx.accounts.launch;

    require_keys_eq!(
        launch.authority,
        ctx.accounts.authority.key(),
        crate::errors::LaunchError::Unauthorized
    );

    require!(
        !launch.trading_enabled,
        crate::errors::LaunchError::AlreadyTrading
    );

    launch.max_buy = max_buy;
    launch.max_wallet = max_wallet;
    launch.cooldown_seconds = cooldown_seconds;
    launch.protection_window = protection_window;

    emit!(ConfigUpdated {
        max_buy,
        max_wallet,
        cooldown_seconds,
        protection_window,
    });

    Ok(())
}