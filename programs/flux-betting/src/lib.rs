use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;
mod utils;
mod processor;

use state::*;

declare_id!("6HaQcudkjjPCn3wP7iSV9HKwhLSN63kLinqVLPBVPoVb");

#[program]
pub mod flux_betting {
    use super::*;

    pub fn initialize_platform(
        ctx: Context<InitializePlatform>,
        fee_percentage: u16,
    ) -> Result<()> {
        instructions::platform::initialize_platform(ctx, fee_percentage)
    }

    pub fn create_group(
        ctx: Context<CreateGroup>,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::group::create_group(ctx, name, description)
    }

    pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
        instructions::group::join_group(ctx)
    }

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
        instructions::bet::create_bet(
            ctx,
            bet_id,
            coin,
            description,
            options,
            odds,
            end_time,
            min_bet_amount,
        )
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        amount: u64,
        option_index: u8,
    ) -> Result<()> {
        instructions::bet::place_bet(ctx, amount, option_index)
    }

    pub fn resolve_bet(
        ctx: Context<ResolveBet>,
        winning_option: u8,
        actual_price: u64,
    ) -> Result<()> {
        instructions::bet::resolve_bet(ctx, winning_option, actual_price)
    }

    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        instructions::bet::claim_winnings(ctx)
    }
}
