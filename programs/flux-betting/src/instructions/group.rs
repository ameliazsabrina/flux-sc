use anchor_lang::prelude::*;
use crate::state::*;

pub fn create_group(
    ctx: Context<CreateGroup>,
    name: String,
    description: String,
) -> Result<()> {
    let group = &mut ctx.accounts.group;
    let admin = &ctx.accounts.admin;
    let platform = &mut ctx.accounts.platform;
    let user_profile = &mut ctx.accounts.user_profile;
    
    group.name = name.clone();
    group.description = description;
    group.admin = admin.key();
    group.members = vec![admin.key()];
    group.active_bets = Vec::new();
    group.past_bets = Vec::new();
    group.created_at = Clock::get()?.unix_timestamp;
    group.bump = ctx.bumps.group;
    
    // new profile
    if user_profile.user == Pubkey::default() {
        user_profile.user = admin.key();
        user_profile.groups = Vec::new();
        user_profile.active_bets = Vec::new();
        user_profile.past_bets = Vec::new();
        user_profile.total_winnings = 0;
        user_profile.total_losses = 0;
        user_profile.bump = ctx.bumps.user_profile;
        
        platform.total_users = platform.total_users.checked_add(1).unwrap();
    }
    
    user_profile.groups.push(group.key());
    
    platform.total_groups = platform.total_groups.checked_add(1).unwrap();
    
    msg!("Group '{}' created by {}", name, admin.key());
    
    Ok(())
}

pub fn join_group(ctx: Context<JoinGroup>) -> Result<()> {
    let group = &mut ctx.accounts.group;
    let user = &ctx.accounts.user;
    let user_profile = &mut ctx.accounts.user_profile;
    
    if group.members.contains(&user.key()) {
        msg!("User is already a member of this group");
        return Ok(());
    }
    
    group.members.push(user.key());
    
    // new profile
    if user_profile.user == Pubkey::default() {
        user_profile.user = user.key();
        user_profile.groups = Vec::new();
        user_profile.active_bets = Vec::new();
        user_profile.past_bets = Vec::new();
        user_profile.total_winnings = 0;
        user_profile.total_losses = 0;
        user_profile.bump = ctx.bumps.user_profile;
    }
    
    user_profile.groups.push(group.key());
    
    msg!("User {} joined group '{}'", user.key(), group.name);
    
    Ok(())
} 