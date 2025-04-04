use anchor_lang::prelude::*;
use crate::errors::FluxError;

pub fn process_initialize_platform(fee_percentage: u16) -> Result<()> {
    require!(fee_percentage <= 10000, FluxError::InvalidFeePercentage);
    Ok(())
}

// create bet
pub fn process_create_bet(
    options: &Vec<String>,
    odds: &Vec<u16>,
    end_time: i64,
) -> Result<()> {
    require!(options.len() == odds.len(), FluxError::OptionOddsMismatch);
    
    require!(options.len() >= 2, FluxError::TooFewOptions);
    require!(options.len() <= 10, FluxError::TooManyOptions);
    
    let current_time = Clock::get()?.unix_timestamp;
    require!(end_time > current_time, FluxError::BetPeriodEnded);
    
    Ok(())
}

// place bet
pub fn process_place_bet(
    option_index: u8,
    amount: u64,
    min_bet_amount: u64,
    options_len: usize,
    is_resolved: bool,
    end_time: i64,
) -> Result<()> {
    require!(!is_resolved, FluxError::BetAlreadyResolved);
    
    require!(option_index < options_len as u8, FluxError::InvalidOptionIndex);
    
    require!(amount >= min_bet_amount, FluxError::BetAmountBelowMinimum);
    
    let current_time = Clock::get()?.unix_timestamp;
    require!(current_time < end_time, FluxError::BetPeriodEnded);
    
    Ok(())
} 