use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub mod state;
pub mod error;
pub mod events;
pub mod utils;
pub mod constants;

pub use state::{LaunchState, TradeState, BlacklistState};
use error::LaunchError;
use events::*;
use utils::*;
use constants::*;

declare_id!("2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj");

pub const LAUNCH_PHASE_INACTIVE: u8 = 0;
pub const LAUNCH_PHASE_READY: u8 = 1;
pub const LAUNCH_PHASE_ACTIVE: u8 = 2;
pub const LAUNCH_PHASE_PAUSED: u8 = 3;

#[program]
pub mod sol_launch {
    use super::*;

    pub fn initialize_launch(
        ctx: Context<InitializeLaunch>,
        max_buy: u64,
        max_wallet: u64,
        cooldown_seconds: i64,
        total_supply: u64,
        sniper_protection_enabled: bool,
        min_trading_duration: i64,
        // Enhanced anti-sniper parameters
        wallet_blacklist_enabled: bool,
        progressive_limits_enabled: bool,
        initial_max_buy: u64,
        initial_max_wallet: u64,
        limit_increase_interval: i64,
        limit_increase_multiplier: u64,
        anti_scam_enabled: bool,
        max_trades_per_user: u64,
    ) -> Result<()> {
        let launch = &mut ctx.accounts.launch;

        validate_launch_config(max_buy, max_wallet, cooldown_seconds)?;
        
        // Validate progressive limits
        if progressive_limits_enabled {
            require!(initial_max_buy > 0, LaunchError::InvalidConfig);
            require!(initial_max_wallet > 0, LaunchError::InvalidConfig);
            require!(initial_max_buy <= max_buy, LaunchError::InvalidConfig);
            require!(initial_max_wallet <= max_wallet, LaunchError::InvalidConfig);
            require!(limit_increase_interval > 0, LaunchError::InvalidConfig);
            require!(limit_increase_multiplier > 0, LaunchError::InvalidConfig);
        }

        launch.authority = ctx.accounts.authority.key();
        launch.trading_enabled = false;
        launch.phase = LAUNCH_PHASE_READY;
        launch.max_buy = max_buy;
        launch.max_wallet = max_wallet;
        launch.cooldown_seconds = cooldown_seconds;
        launch.token_mint = ctx.accounts.token_mint.key();
        launch.vault = ctx.accounts.vault.key();
        launch.vault_bump = ctx.bumps.vault;
        launch.launch_bump = ctx.bumps.launch;
        launch.start_timestamp = 0;
        launch.total_supply = total_supply;
        launch.total_traded = 0;
        launch.sniper_protection_enabled = sniper_protection_enabled;
        launch.min_trading_duration = min_trading_duration;
        
        // Enhanced anti-sniper features
        launch.wallet_blacklist_enabled = wallet_blacklist_enabled;
        launch.progressive_limits_enabled = progressive_limits_enabled;
        launch.initial_max_buy = if progressive_limits_enabled { initial_max_buy } else { max_buy };
        launch.initial_max_wallet = if progressive_limits_enabled { initial_max_wallet } else { max_wallet };
        launch.limit_increase_interval = limit_increase_interval;
        launch.limit_increase_multiplier = limit_increase_multiplier;
        launch.anti_scam_enabled = anti_scam_enabled;
        launch.max_trades_per_user = max_trades_per_user;
        launch.total_traders = 0;

        emit!(LaunchInitialized {
            launch: launch.key(),
            authority: launch.authority,
            token_mint: launch.token_mint,
            vault: launch.vault,
            max_buy,
            max_wallet,
            cooldown_seconds,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn deposit_tokens(
        ctx: Context<DepositTokens>,
        amount: u64,
    ) -> Result<()> {
        let launch = &ctx.accounts.launch;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        validate_vault(launch.vault, ctx.accounts.vault.key())?;

        let launch_key = launch.key();
        let transfer_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.launch.to_account_info(),
        };

        let vault_seeds = &[VAULT_SEED, launch_key.as_ref(), &[launch.vault_bump]];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            transfer_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;

        emit!(TokensDeposited {
            launch: launch.key(),
            vault: ctx.accounts.vault.key(),
            from: ctx.accounts.from.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn withdraw_tokens(
        ctx: Context<WithdrawTokens>,
        amount: u64,
    ) -> Result<()> {
        let launch = &ctx.accounts.launch;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        validate_vault(launch.vault, ctx.accounts.vault.key())?;

        // Check vault has sufficient balance
        require!(ctx.accounts.vault.amount >= amount, LaunchError::InsufficientBalance);

        let launch_key = launch.key();
        let transfer_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.launch.to_account_info(),
        };

        let vault_seeds = &[VAULT_SEED, launch_key.as_ref(), &[launch.vault_bump]];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            transfer_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;

        emit!(TokensWithdrawn {
            launch: launch.key(),
            vault: ctx.accounts.vault.key(),
            to: ctx.accounts.to.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn enable_trading(ctx: Context<EnableTrading>) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        require!(!launch.trading_enabled, LaunchError::AlreadyEnabled);

        // Check if launch delay has passed
        if launch.start_timestamp > 0 {
            require!(current_timestamp >= launch.start_timestamp, LaunchError::NotReady);
        }

        validate_phase_transition(launch.phase, LAUNCH_PHASE_ACTIVE)?;

        launch.trading_enabled = true;
        launch.phase = LAUNCH_PHASE_ACTIVE;

        emit!(TradingEnabled {
            launch: launch.key(),
            authority: ctx.accounts.authority.key(),
            timestamp: current_timestamp,
        });

        Ok(())
    }

    pub fn disable_trading(ctx: Context<DisableTrading>) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        require!(launch.trading_enabled, LaunchError::TradingNotEnabled);

        // Sniper protection: prevent disabling trading too quickly
        if launch.sniper_protection_enabled {
            let trading_duration = current_timestamp.saturating_sub(launch.start_timestamp);
            require!(trading_duration >= launch.min_trading_duration, LaunchError::TradingNotEnabled);
        }

        validate_phase_transition(launch.phase, LAUNCH_PHASE_PAUSED)?;

        launch.trading_enabled = false;
        launch.phase = LAUNCH_PHASE_PAUSED;

        emit!(TradingDisabled {
            launch: launch.key(),
            authority: ctx.accounts.authority.key(),
            timestamp: current_timestamp,
        });

        Ok(())
    }

    pub fn start_launch(ctx: Context<StartLaunch>, launch_delay_seconds: i64) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        require!(launch.phase == LAUNCH_PHASE_READY, LaunchError::NotReady);
        require!(launch_delay_seconds > 0, LaunchError::InvalidConfig);
        require!(launch.start_timestamp == 0, LaunchError::AlreadyEnabled);

        launch.phase = LAUNCH_PHASE_READY;
        launch.cooldown_seconds = launch_delay_seconds.max(launch.cooldown_seconds);
        launch.start_timestamp = current_timestamp + launch_delay_seconds;

        emit!(LaunchStarted {
            launch: launch.key(),
            authority: ctx.accounts.authority.key(),
            delay_seconds: launch_delay_seconds,
            timestamp: current_timestamp,
        });

        Ok(())
    }

    pub fn pause_launch(ctx: Context<PauseLaunch>) -> Result<()> {
        let launch = &mut ctx.accounts.launch;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        require!(launch.phase != LAUNCH_PHASE_PAUSED, LaunchError::AlreadyPaused);

        validate_phase_transition(launch.phase, LAUNCH_PHASE_PAUSED)?;

        launch.phase = LAUNCH_PHASE_PAUSED;
        launch.trading_enabled = false;

        emit!(LaunchPaused {
            launch: launch.key(),
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn resume_launch(ctx: Context<ResumeLaunch>) -> Result<()> {
        let launch = &mut ctx.accounts.launch;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        require!(launch.phase == LAUNCH_PHASE_PAUSED, LaunchError::NotPaused);

        validate_phase_transition(launch.phase, LAUNCH_PHASE_ACTIVE)?;

        launch.phase = LAUNCH_PHASE_ACTIVE;
        launch.trading_enabled = true;

        emit!(LaunchResumed {
            launch: launch.key(),
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn transfer_authority(
        ctx: Context<TransferAuthority>,
        new_authority: Pubkey,
    ) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        let old_authority = launch.authority;

        require_keys_eq!(
            launch.authority,
            ctx.accounts.authority.key(),
            LaunchError::Unauthorized
        );

        validate_authority_change(launch.authority, new_authority)?;

        launch.authority = new_authority;

        emit!(AuthorityTransferred {
            launch: launch.key(),
            old_authority,
            new_authority,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn execute_trade(
        ctx: Context<ExecuteTrade>,
        amount: u64,
    ) -> Result<()> {
        let launch = &ctx.accounts.launch;
        let trade_state = &mut ctx.accounts.trade_state;
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // Use progressive limits for validation
        let current_max_buy = launch.get_current_max_buy(current_timestamp);
        let current_max_wallet = launch.get_current_max_wallet(current_timestamp);
        
        validate_trade_request_with_limits(
            launch, 
            trade_state, 
            amount, 
            current_timestamp,
            current_max_buy,
            current_max_wallet,
        )?;

        // Check vault has sufficient tokens for the trade
        require!(ctx.accounts.vault.amount >= amount, LaunchError::InsufficientVaultBalance);

        // Anti-scam: Check max trades per user
        if launch.anti_scam_enabled {
            require!(trade_state.trade_count < launch.max_trades_per_user, LaunchError::MaxTradesExceeded);
        }

        let launch_key = launch.key();
        let launch_bump = launch.vault_bump;
        
        // Transfer tokens from vault to user using PDA signing
        let transfer_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.launch.to_account_info(),
        };

        let vault_seeds = &[VAULT_SEED, launch_key.as_ref(), &[launch_bump]];
        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            transfer_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;

        // Update trade state efficiently (single borrow)
        let is_first_trade = trade_state.trade_count == 0;
        trade_state.last_trade_timestamp = current_timestamp;
        trade_state.total_traded = trade_state.total_traded.checked_add(amount)
            .ok_or(LaunchError::Overflow)?;
        trade_state.trade_count = trade_state.trade_count.checked_add(1)
            .ok_or(LaunchError::Overflow)?;
        trade_state.bump = ctx.bumps.trade_state;

        // Update global launch statistics
        let launch = &mut ctx.accounts.launch;
        launch.total_traded = launch.total_traded.checked_add(amount)
            .ok_or(LaunchError::Overflow)?;
        
        // Increment total traders if this is their first trade
        if is_first_trade {
            launch.total_traders = launch.total_traders.checked_add(1)
                .ok_or(LaunchError::Overflow)?;
        }

        emit!(TradeExecuted {
            launch: launch.key(),
            trader: ctx.accounts.authority.key(),
            amount,
            total_traded: trade_state.total_traded,
            timestamp: current_timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLaunch<'info> {
    #[account(
        init,
        payer = authority,
        space = LaunchState::SPACE,
        seeds = [LAUNCH_SEED, authority.key().as_ref()],
        bump
    )]
    pub launch: Account<'info, LaunchState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = launch,
        seeds = [VAULT_SEED, launch.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    #[account(
        mut,
        constraint = from.mint == launch.token_mint @ LaunchError::InvalidMint,
        constraint = from.owner == authority.key() @ LaunchError::InvalidVaultAuthority
    )]
    pub from: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault.key() == launch.vault @ LaunchError::VaultMismatch,
        constraint = vault.mint == launch.token_mint @ LaunchError::InvalidMint
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    #[account(
        mut,
        constraint = vault.key() == launch.vault @ LaunchError::VaultMismatch,
        constraint = vault.mint == launch.token_mint @ LaunchError::InvalidMint
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to.mint == launch.token_mint @ LaunchError::InvalidMint,
        constraint = to.owner == authority.key() @ LaunchError::InvalidUserTokenAccount
    )]
    pub to: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EnableTrading<'info> {
    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisableTrading<'info> {
    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct StartLaunch<'info> {
    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferAuthority<'info> {
    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct PauseLaunch<'info> {
    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResumeLaunch<'info> {
    #[account(
        mut,
        constraint = launch.authority == authority.key() @ LaunchError::Unauthorized
    )]
    pub launch: Account<'info, LaunchState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    #[account(
        mut,
        constraint = launch.trading_enabled @ LaunchError::TradingNotEnabled,
        constraint = launch.phase == LAUNCH_PHASE_ACTIVE @ LaunchError::TradingNotEnabled
    )]
    pub launch: Account<'info, LaunchState>,

    #[account(
        init,
        payer = authority,
        space = TradeState::SPACE,
        seeds = [TRADE_SEED, authority.key().as_ref(), launch.key().as_ref()],
        bump
    )]
    pub trade_state: Account<'info, TradeState>,

    #[account(
        mut,
        constraint = vault.key() == launch.vault @ LaunchError::VaultMismatch,
        constraint = vault.mint == launch.token_mint @ LaunchError::InvalidMint
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token.mint == launch.token_mint @ LaunchError::InvalidMint,
        constraint = user_token.owner == authority.key() @ LaunchError::InvalidUserTokenAccount
    )]
    pub user_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_launch_config_values() {
        assert!(validate_launch_config(100, 500, 60).is_ok());
        assert!(validate_launch_config(0, 500, 60).is_err());
        assert!(validate_launch_config(100, 50, 30).is_err());
        assert!(validate_launch_config(100, 5000, 0).is_err());
    }

    #[test]
    fn validates_trade_request_rules() {
        let launch = LaunchState {
            authority: Pubkey::default(),
            trading_enabled: true,
            phase: LAUNCH_PHASE_ACTIVE,
            max_buy: 100,
            max_wallet: 500,
            cooldown_seconds: 60,
            token_mint: Pubkey::default(),
            vault: Pubkey::default(),
            vault_bump: 0,
            launch_bump: 0,
            start_timestamp: 0,
            total_supply: 1000,
            total_traded: 0,
            sniper_protection_enabled: false,
            min_trading_duration: 0,
            wallet_blacklist_enabled: false,
            progressive_limits_enabled: false,
            initial_max_buy: 100,
            initial_max_wallet: 500,
            limit_increase_interval: 0,
            limit_increase_multiplier: 0,
            anti_scam_enabled: false,
            max_trades_per_user: 0,
            total_traders: 0,
        };
        
        // Test with no previous trade (trade_count = 0) - should pass cooldown check
        let tracker_no_history = TradeState {
            last_trade_timestamp: 100,
            total_traded: 200,
            trade_count: 0,
            bump: 0,
        };

        // Should pass since trade_count is 0 (no cooldown check)
        assert!(validate_trade_request(&launch, &tracker_no_history, 50, 150).is_ok());
        assert!(validate_trade_request(&launch, &tracker_no_history, 50, 200).is_ok());
        assert!(validate_trade_request(&launch, &tracker_no_history, 400, 200).is_err()); // Exceeds max_wallet
        
        // Test with previous trade (trade_count > 0) - should enforce cooldown
        let tracker_with_history = TradeState {
            last_trade_timestamp: 100,
            total_traded: 200,
            trade_count: 1,
            bump: 0,
        };

        // Should fail due to cooldown (150 - 100 = 50 < 60)
        assert!(validate_trade_request(&launch, &tracker_with_history, 50, 150).is_err());
        // Should pass (200 - 100 = 100 >= 60)
        assert!(validate_trade_request(&launch, &tracker_with_history, 50, 200).is_ok());
        // Should fail due to max_wallet
        assert!(validate_trade_request(&launch, &tracker_with_history, 400, 200).is_err());
    }

    #[test]
    fn test_phase_transitions() {
        assert!(validate_phase_transition(1, 2).is_ok()); // READY -> ACTIVE
        assert!(validate_phase_transition(2, 3).is_ok()); // ACTIVE -> PAUSED
        assert!(validate_phase_transition(3, 2).is_ok()); // PAUSED -> ACTIVE
        assert!(validate_phase_transition(1, 3).is_err()); // READY -> PAUSED (invalid)
        assert!(validate_phase_transition(0, 1).is_err()); // INACTIVE -> READY (invalid)
    }

    #[test]
    fn test_anti_bot_rate_limiting() {
        let launch = LaunchState {
            authority: Pubkey::default(),
            trading_enabled: true,
            phase: LAUNCH_PHASE_ACTIVE,
            max_buy: 100,
            max_wallet: 500,
            cooldown_seconds: 60, // Standard cooldown
            token_mint: Pubkey::default(),
            vault: Pubkey::default(),
            vault_bump: 0,
            launch_bump: 0,
            start_timestamp: 0,
            total_supply: 1000,
            total_traded: 0,
            sniper_protection_enabled: false,
            min_trading_duration: 0,
            wallet_blacklist_enabled: false,
            progressive_limits_enabled: false,
            initial_max_buy: 100,
            initial_max_wallet: 500,
            limit_increase_interval: 0,
            limit_increase_multiplier: 0,
            anti_scam_enabled: false,
            max_trades_per_user: 0,
            total_traders: 0,
        };
        
        // Test with no previous trade (should pass - no cooldown check)
        let tracker_no_history = TradeState {
            last_trade_timestamp: 0,
            total_traded: 0,
            trade_count: 0,
            bump: 0,
        };
        
        // Should pass with no previous trade
        assert!(validate_trade_request(&launch, &tracker_no_history, 50, 200).is_ok());
        
        // Test with previous trade (should enforce cooldown + 1-second minimum)
        let tracker_with_history = TradeState {
            last_trade_timestamp: 200,
            total_traded: 100,
            trade_count: 1,
            bump: 0,
        };
        
        // Current timestamp is 200, should fail (less than cooldown)
        assert!(validate_trade_request(&launch, &tracker_with_history, 50, 200).is_err());
        
        // Current timestamp is 261, should pass (cooldown elapsed + 1 second minimum)
        assert!(validate_trade_request(&launch, &tracker_with_history, 50, 261).is_ok());
    }

    #[test]
    fn test_trade_count_overflow_protection() {
        let launch = LaunchState {
            authority: Pubkey::default(),
            trading_enabled: true,
            phase: LAUNCH_PHASE_ACTIVE,
            max_buy: 100,
            max_wallet: 500,
            cooldown_seconds: 60,
            token_mint: Pubkey::default(),
            vault: Pubkey::default(),
            vault_bump: 0,
            launch_bump: 0,
            start_timestamp: 0,
            total_supply: 1000,
            total_traded: 0,
            sniper_protection_enabled: false,
            min_trading_duration: 0,
            wallet_blacklist_enabled: false,
            progressive_limits_enabled: false,
            initial_max_buy: 100,
            initial_max_wallet: 500,
            limit_increase_interval: 0,
            limit_increase_multiplier: 0,
            anti_scam_enabled: false,
            max_trades_per_user: 0,
            total_traders: 0,
        };
        
        let tracker = TradeState {
            last_trade_timestamp: 100,
            total_traded: 200,
            trade_count: u64::MAX,
            bump: 0,
        };
        
        // Should still work for the validation, overflow would happen in execution
        assert!(validate_trade_request(&launch, &tracker, 50, 200).is_ok());
    }

    #[test]
    fn test_sniper_protection_settings() {
        let launch_with_protection = LaunchState {
            authority: Pubkey::default(),
            trading_enabled: true,
            phase: LAUNCH_PHASE_ACTIVE,
            max_buy: 100,
            max_wallet: 500,
            cooldown_seconds: 60,
            token_mint: Pubkey::default(),
            vault: Pubkey::default(),
            vault_bump: 0,
            launch_bump: 0,
            start_timestamp: 1000,
            total_supply: 1000,
            total_traded: 0,
            sniper_protection_enabled: true,
            min_trading_duration: 300,
            wallet_blacklist_enabled: false,
            progressive_limits_enabled: false,
            initial_max_buy: 100,
            initial_max_wallet: 500,
            limit_increase_interval: 0,
            limit_increase_multiplier: 0,
            anti_scam_enabled: false,
            max_trades_per_user: 0,
            total_traders: 0,
        };
        
        let launch_without_protection = LaunchState {
            sniper_protection_enabled: false,
            min_trading_duration: 0,
            ..launch_with_protection.clone()
        };
        
        assert!(launch_with_protection.sniper_protection_enabled);
        assert!(launch_with_protection.min_trading_duration == 300);
        assert!(!launch_without_protection.sniper_protection_enabled);
        assert!(launch_without_protection.min_trading_duration == 0);
    }

    #[test]
    fn test_authority_change_validation() {
        let auth1 = Pubkey::new_unique();
        let auth2 = Pubkey::new_unique();
        
        assert!(validate_authority_change(auth1, auth2).is_ok());
        assert!(validate_authority_change(auth1, auth1).is_err());
    }

    #[test]
    fn test_token_mint_validation() {
        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();
        
        assert!(validate_token_mint(mint1, mint1).is_ok());
        assert!(validate_token_mint(mint1, mint2).is_err());
    }

    #[test]
    fn test_vault_validation() {
        let vault1 = Pubkey::new_unique();
        let vault2 = Pubkey::new_unique();
        
        assert!(validate_vault(vault1, vault1).is_ok());
        assert!(validate_vault(vault1, vault2).is_err());
    }

    #[test]
    fn test_launch_state_structure() {
        let launch = LaunchState {
            authority: Pubkey::default(),
            trading_enabled: true,
            phase: LAUNCH_PHASE_ACTIVE,
            max_buy: 1000,
            max_wallet: 10000,
            cooldown_seconds: 120,
            token_mint: Pubkey::new_unique(),
            vault: Pubkey::new_unique(),
            vault_bump: 255,
            launch_bump: 254,
            start_timestamp: 1000000,
            total_supply: 1_000_000_000,
            total_traded: 500_000_000,
            sniper_protection_enabled: true,
            min_trading_duration: 600,
            wallet_blacklist_enabled: false,
            progressive_limits_enabled: false,
            initial_max_buy: 1000,
            initial_max_wallet: 10000,
            limit_increase_interval: 0,
            limit_increase_multiplier: 0,
            anti_scam_enabled: false,
            max_trades_per_user: 0,
            total_traders: 0,
        };
        
        assert_eq!(launch.phase, LAUNCH_PHASE_ACTIVE);
        assert!(launch.trading_enabled);
        assert!(launch.sniper_protection_enabled);
        assert_eq!(launch.max_buy, 1000);
        assert_eq!(launch.total_supply, 1_000_000_000);
    }

    #[test]
    fn test_trade_state_structure() {
        let trade_state = TradeState {
            last_trade_timestamp: 1234567890,
            total_traded: 5000,
            trade_count: 10,
            bump: 123,
        };
        
        assert_eq!(trade_state.last_trade_timestamp, 1234567890);
        assert_eq!(trade_state.total_traded, 5000);
        assert_eq!(trade_state.trade_count, 10);
        assert_eq!(trade_state.bump, 123);
    }

    #[test]
    fn test_launch_state_space_calculation() {
        // Verify the space calculation is correct with new fields
        let expected_space = 8 + // discriminator
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
        
        assert_eq!(LaunchState::SPACE, expected_space);
    }

    #[test]
    fn test_trade_state_space_calculation() {
        let expected_space = 8 + // discriminator
            8 + // last_trade_timestamp
            8 + // total_traded
            8 + // trade_count
            1;  // bump
        
        assert_eq!(TradeState::SPACE, expected_space);
    }
}