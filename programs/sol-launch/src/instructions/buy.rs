use anchor_lang::prelude::*;

use crate::{
    errors::LaunchError,
    state::{BuyerState, LaunchState},
    events::BuyExecuted,
};

#[derive(Accounts)]
pub struct Buy<'info> {

    #[account(mut)]
    pub launch: Account<'info, LaunchState>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + BuyerState::LEN,
        seeds = [b"buyer", buyer.key().as_ref()],
        bump
    )]
    pub buyer_state: Account<'info, BuyerState>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub system_program: Program<'info, System>,
}


pub fn buy(
    ctx: Context<Buy>,
    amount: u64,
) -> Result<()> {

    let launch = &ctx.accounts.launch;
    let buyer_state = &mut ctx.accounts.buyer_state;

    require!(
        launch.trading_enabled,
        LaunchError::TradingDisabled
    );

    require!(
        !launch.paused,
        LaunchError::TradingPaused
    );

    require!(
        amount <= launch.max_buy,
        LaunchError::MaxBuyExceeded
    );


    let clock = Clock::get()?;

    if buyer_state.last_buy_timestamp != 0 {
        let elapsed =
            clock.unix_timestamp - buyer_state.last_buy_timestamp;

        require!(
            elapsed >= launch.cooldown_seconds,
            LaunchError::CooldownActive
        );
    }


    buyer_state.owner = ctx.accounts.buyer.key();
    buyer_state.last_buy_timestamp = clock.unix_timestamp;
    buyer_state.amount_bought += amount;

emit!(BuyExecuted {
    buyer: ctx.accounts.buyer.key(),
    amount,
    timestamp: clock.unix_timestamp,
});

    require!(
        buyer_state.amount_bought <= launch.max_wallet,
        LaunchError::MaxWalletExceeded
    );


    Ok(())
}