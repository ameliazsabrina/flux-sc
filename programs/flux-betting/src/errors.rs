use anchor_lang::prelude::*;

#[error_code]
pub enum FluxError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("Invalid option index")]
    InvalidOptionIndex,
    
    #[msg("Bet already exists")]
    BetAlreadyExists,
    
    #[msg("Bet already resolved")]
    BetAlreadyResolved,
    
    #[msg("Bet not resolved yet")]
    BetNotResolved,
    
    #[msg("Bet already closed")]
    BetClosed,
    
    #[msg("Bet period ended")]
    BetPeriodEnded,
    
    #[msg("Only bet creator can resolve bet")]
    UnauthorizedResolver,
    
    #[msg("Fee percentage must be 10000 or less (100%)")]
    InvalidFeePercentage,
    
    #[msg("Only group admin can create bets")]
    UnauthorizedBetCreator,
    
    #[msg("User is not a member of the group")]
    NotGroupMember,
    
    #[msg("No winnings to claim")]
    NoWinningsToClaim,
    
    #[msg("Options and odds arrays must be same length")]
    OptionOddsMismatch,
    
    #[msg("Minimum of 2 options required")]
    TooFewOptions,
    
    #[msg("Maximum of 10 options allowed")]
    TooManyOptions,
    
    #[msg("Bet amount below minimum")]
    BetAmountBelowMinimum,
} 