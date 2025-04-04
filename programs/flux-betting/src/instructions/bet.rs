use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use crate::errors::FluxError;
use crate::state::*;
use crate::utils::{calculate_winnings, validate_options_and_odds};

pub fn create_bet(
    ctx: Context<CreateBet>,
    bet_id: String,
    coin: String,
    description: String,
    options: Vec<String>,
    odds: Vec<u16>,
    end_time: i64,
    min_bet_amount: u64,
) -> Result<()> {

    validate_options_and_odds(&options, &odds)?;
    
    let current_time = Clock::get()?.unix_timestamp;    
    require!(end_time > current_time, FluxError::BetPeriodEnded);
    
    let bet = &mut ctx.accounts.bet;
    let group = &mut ctx.accounts.group;
    let creator = &ctx.accounts.creator;
    let platform = &mut ctx.accounts.platform;
    let user_profile = &mut ctx.accounts.user_profile;
    
    let mut bets_per_option = Vec::new();
    for _ in 0..options.len() {
        bets_per_option.push(0);
    }
    
    bet.id = bet_id.clone();
    bet.group = group.key();
    bet.creator = creator.key();
    bet.coin = coin.clone();
    bet.description = description;
    bet.options = options;
    bet.odds = odds;
    bet.min_bet_amount = min_bet_amount;
    bet.total_pool = 0;
    bet.bets_per_option = bets_per_option;
    bet.created_at = current_time;
    bet.end_time = end_time;
    bet.resolved = false;
    bet.winning_option = None;
    bet.actual_price = None;
    bet.bump = ctx.bumps.bet;
    
    group.active_bets.push(bet.key());
    
    user_profile.active_bets.push(bet.key());
    
    platform.total_bets = platform.total_bets.checked_add(1).unwrap();
    
    msg!("Bet '{}' created for coin {} by {}", bet_id, coin, creator.key());
    
    Ok(())
}

pub fn place_bet(
    ctx: Context<PlaceBet>,
    amount: u64,
    option_index: u8,
) -> Result<()> {
    let bet = &mut ctx.accounts.bet;
    let user = &ctx.accounts.user;
    let user_bet = &mut ctx.accounts.user_bet;
    let user_profile = &mut ctx.accounts.user_profile;
    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: user.to_account_info(),
            },
        ),
        amount,
    )?;
    
    bet.total_pool = bet.total_pool.checked_add(amount).unwrap();
    bet.bets_per_option[option_index as usize] = bet
        .bets_per_option[option_index as usize]
        .checked_add(amount)
        .unwrap();
    
    user_bet.user = user.key();
    user_bet.bet = bet.key();
    user_bet.amount = amount;
    user_bet.option_index = option_index;
    user_bet.claimed = false;
    user_bet.winnings = None;
    user_bet.bump = ctx.bumps.user_bet;
    
    if !user_profile.active_bets.contains(&bet.key()) {
        user_profile.active_bets.push(bet.key());
    }
    
    msg!("User {} placed bet of {} on option {} for bet '{}'", 
         user.key(), amount, option_index, bet.id);
    
    Ok(())
}

pub fn resolve_bet(
    ctx: Context<ResolveBet>,
    winning_option: u8,
    actual_price: u64,
) -> Result<()> {
    let bet = &mut ctx.accounts.bet;
    let group = &mut ctx.accounts.group;
    
    bet.resolved = true;
    bet.winning_option = Some(winning_option);
    bet.actual_price = Some(actual_price);
    
    let bet_key = bet.key();
    if let Some(index) = group.active_bets.iter().position(|&x| x == bet_key) {
        group.active_bets.remove(index);
        group.past_bets.push(bet_key);
    }
    
    msg!("Bet '{}' resolved with winning option {} and actual price {}", 
         bet.id, winning_option, actual_price);
    
    Ok(())
}

pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
    let bet = &ctx.accounts.bet;
    let user_bet = &mut ctx.accounts.user_bet;
    let user = &ctx.accounts.user;
    let user_profile = &mut ctx.accounts.user_profile;
    let platform = &ctx.accounts.platform;
    
    let winning_option_index = bet.winning_option.unwrap();
    let odds = bet.odds[winning_option_index as usize];
    
    let winnings = calculate_winnings(
        user_bet.amount,
        odds,
        bet.total_pool,
        platform.fee_percentage,
    )?;
    
    let platform_seeds = &[b"platform".as_ref(), &[platform.bump]];
    let seeds = &[&platform_seeds[..]];
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.treasury_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        },
        seeds,
    );
    
    token::transfer(transfer_ctx, winnings)?;
    
    user_bet.claimed = true;
    user_bet.winnings = Some(winnings);
    
    user_profile.total_winnings = user_profile
        .total_winnings
        .checked_add(winnings)
        .unwrap();
    
    let bet_key = bet.key();
    if let Some(index) = user_profile.active_bets.iter().position(|&x| x == bet_key) {
        user_profile.active_bets.remove(index);
        if !user_profile.past_bets.contains(&bet_key) {
            user_profile.past_bets.push(bet_key);
        }
    }
    
    msg!("User {} claimed {} winnings for bet '{}'", 
         user.key(), winnings, bet.id);
    
    Ok(())
} 