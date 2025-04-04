use anchor_lang::prelude::*;
use crate::errors::FluxError;

// calculate winnings
pub fn calculate_winnings(
    bet_amount: u64,
    odds: u16,
    total_pool: u64,
    fee_percentage: u16,
) -> Result<u64> {
    let raw_winnings = bet_amount
        .checked_mul(odds as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(100)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    // apply platform fee
    let fee_amount = raw_winnings
        .checked_mul(fee_percentage as u64)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    let final_winnings = raw_winnings
        .checked_sub(fee_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    // Ensure we don't pay out more than the total pool
    if final_winnings > total_pool {
        return Err(FluxError::InsufficientFunds.into());
    }
    
    Ok(final_winnings)
}

// validate options and odds
pub fn validate_options_and_odds(
    options: &Vec<String>,
    odds: &Vec<u16>
) -> Result<()> {
    if options.len() != odds.len() {
        return Err(FluxError::OptionOddsMismatch.into());
    }
    
    if options.len() < 2 {
        return Err(FluxError::TooFewOptions.into());
    }
    
    if options.len() > 10 {
        return Err(FluxError::TooManyOptions.into());
    }
    
    Ok(())
}

// transfer tokens
pub fn transfer_tokens_from_treasury<'a>(
    token_program: &Program<'a, anchor_spl::token::Token>,
    from: &Account<'a, anchor_spl::token::TokenAccount>,
    to: &Account<'a, anchor_spl::token::TokenAccount>,
    authority: &AccountInfo<'a>,
    seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let transfer_instruction = anchor_spl::token::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    
    let ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        transfer_instruction,
        seeds,
    );
    
    anchor_spl::token::transfer(ctx, amount)
} 