use anchor_lang::prelude::*;
use crate::errors::FluxError;

#[account]
pub struct Platform {
    pub admin: Pubkey,
    pub fee_percentage: u16, // Basis points (e.g., 100 = 1%)
    pub treasury: Pubkey,
    pub total_bets: u64,
    pub total_users: u64,
    pub total_groups: u64,
    pub bump: u8,
}

#[account]
pub struct Group {
    pub name: String,
    pub description: String,
    pub admin: Pubkey,
    pub members: Vec<Pubkey>,
    pub active_bets: Vec<Pubkey>,
    pub past_bets: Vec<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
pub struct UserProfile {
    pub user: Pubkey,
    pub groups: Vec<Pubkey>,
    pub active_bets: Vec<Pubkey>,
    pub past_bets: Vec<Pubkey>,
    pub total_winnings: u64,
    pub total_losses: u64,
    pub bump: u8,
}

#[account]
pub struct Bet {
    pub id: String,
    pub group: Pubkey,
    pub creator: Pubkey,
    pub coin: String,
    pub description: String,
    pub options: Vec<String>,
    pub odds: Vec<u16>, // Odds in basis points (100 = 1.0x, 200 = 2.0x)
    pub min_bet_amount: u64,
    pub total_pool: u64,
    pub bets_per_option: Vec<u64>,
    pub created_at: i64,
    pub end_time: i64,
    pub resolved: bool,
    pub winning_option: Option<u8>,
    pub actual_price: Option<u64>,
    pub bump: u8,
}

#[account]
pub struct UserBet {
    pub user: Pubkey,
    pub bet: Pubkey,
    pub amount: u64,
    pub option_index: u8,
    pub claimed: bool,
    pub winnings: Option<u64>,
    pub bump: u8,
}


#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32 + 2 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"platform"],
        bump
    )]
    pub platform: Account<'info, Platform>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub treasury: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 4 + name.len() + 4 + description.len() + 32 + 4 + (32 * 10) + 4 + (32 * 50) + 4 + (32 * 50) + 8 + 1,
        seeds = [b"group", admin.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub group: Account<'info, Group>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,
    
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + 32 + 4 + (32 * 10) + 4 + (32 * 50) + 4 + (32 * 50) + 8 + 8 + 1,
        seeds = [b"user_profile", admin.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGroup<'info> {
    #[account(mut)]
    pub group: Account<'info, Group>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 4 + (32 * 10) + 4 + (32 * 50) + 4 + (32 * 50) + 8 + 8 + 1,
        seeds = [b"user_profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bet_id: String, coin: String, description: String, options: Vec<String>, odds: Vec<u16>, end_time: i64, min_bet_amount: u64)]
pub struct CreateBet<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + // discriminator
            4 + bet_id.len() + // id (String)
            32 + // group (Pubkey)
            32 + // creator (Pubkey)
            4 + coin.len() + // coin (String)
            4 + description.len() + // description (String)
            4 + options.iter().map(|s| 4 + s.len()).sum::<usize>() + // options (Vec<String>)
            4 + (odds.len() * 2) + // odds (Vec<u16>)
            8 + // min_bet_amount (u64)
            8 + // total_pool (u64)
            4 + (options.len() * 8) + // bets_per_option (Vec<u64>)
            8 + // created_at (i64)
            8 + // end_time (i64)
            1 + // resolved (bool)
            2 + // Option<u8> for winning_option (1 for is_some + 1 for u8)
            9 + // Option<u64> for actual_price (1 for is_some + 8 for u64)
            1, // bump (u8)
        seeds = [b"bet", group.key().as_ref(), bet_id.as_bytes()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    
    #[account(
        mut,
        constraint = group.admin == creator.key() @ FluxError::UnauthorizedBetCreator
    )]
    pub group: Account<'info, Group>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,
    
    #[account(
        mut,
        seeds = [b"user_profile", creator.key().as_ref()],
        bump = user_profile.bump,
        constraint = user_profile.user == creator.key()
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64, option_index: u8)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        constraint = !bet.resolved @ FluxError::BetAlreadyResolved,
        constraint = Clock::get().unwrap().unix_timestamp < bet.end_time @ FluxError::BetPeriodEnded,
        constraint = option_index < bet.options.len() as u8 @ FluxError::InvalidOptionIndex,
        constraint = amount >= bet.min_bet_amount @ FluxError::BetAmountBelowMinimum
    )]
    pub bet: Account<'info, Bet>,
    
    #[account(
        mut,
        constraint = group.members.contains(&user.key()) @ FluxError::NotGroupMember
    )]
    pub group: Account<'info, Group>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8 + 1 + 1 + 8 + 1,
        seeds = [b"user_bet", bet.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_bet: Account<'info, UserBet>,
    
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump,
        constraint = user_profile.user == user.key()
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(winning_option: u8, actual_price: u64)]
pub struct ResolveBet<'info> {
    #[account(
        mut,
        constraint = !bet.resolved @ FluxError::BetAlreadyResolved,
        constraint = winning_option < bet.options.len() as u8 @ FluxError::InvalidOptionIndex,
        constraint = bet.creator == creator.key() @ FluxError::UnauthorizedResolver
    )]
    pub bet: Account<'info, Bet>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub group: Account<'info, Group>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(
        mut,
        constraint = bet.resolved @ FluxError::BetNotResolved,
        seeds = [b"bet", bet.group.as_ref(), bet.id.as_bytes()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    
    #[account(
        mut,
        seeds = [b"user_bet", bet.key().as_ref(), user.key().as_ref()],
        bump = user_bet.bump,
        constraint = user_bet.user == user.key(),
        constraint = !user_bet.claimed @ FluxError::NoWinningsToClaim,
        constraint = Some(user_bet.option_index) == bet.winning_option @ FluxError::NoWinningsToClaim
    )]
    pub user_bet: Account<'info, UserBet>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump = user_profile.bump,
        constraint = user_profile.user == user.key()
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
} 