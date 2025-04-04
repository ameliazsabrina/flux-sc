use anchor_lang::prelude::*;
use crate::errors::FluxError;
use crate::state::*;

pub fn initialize_platform(
    ctx: Context<InitializePlatform>,
    fee_percentage: u16,
) -> Result<()> {
    // fee percentage >= to 10000 (100%)
    require!(fee_percentage <= 10000, FluxError::InvalidFeePercentage);
    
    let platform = &mut ctx.accounts.platform;
    let admin = &ctx.accounts.admin;
    let treasury = &ctx.accounts.treasury;
    
    platform.admin = admin.key();
    platform.fee_percentage = fee_percentage;
    platform.treasury = treasury.key();
    platform.total_bets = 0;
    platform.total_users = 0;
    platform.total_groups = 0;
    platform.bump = ctx.bumps.platform;
    
    msg!("Platform initialized with fee percentage of {}%", fee_percentage as f64 / 100.0);
    
    Ok(())
} 