use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::LaunchState;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub launch: Account<'info, LaunchState>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = authority,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_vault(
    ctx: Context<InitializeVault>,
) -> Result<()> {
    let launch = &mut ctx.accounts.launch;

    require_keys_eq!(
        launch.authority,
        ctx.accounts.authority.key()
    );

    launch.token_mint = ctx.accounts.mint.key();
    launch.vault = ctx.accounts.vault.key();

    Ok(())
}