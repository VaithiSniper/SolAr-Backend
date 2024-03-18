use anchor_lang::prelude::*;
use crate::state::constants::*;
use crate::state::user::*;

pub fn setup_user(ctx: Context<SetupUser>, username: String) -> Result<()> {
    ctx.accounts
        .user
        .initialize_user(ctx.accounts.authority.key(), username)
}

#[derive(Accounts)]
pub struct SetupUser<'info> {
    #[account(
        init, 
        seeds = [USER_TAG, authority.key().as_ref()], 
        bump, 
        payer = authority, 
        space = UserProfile::MAXIMUM_SIZE_FOR_RENT
        )]
    pub user: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
