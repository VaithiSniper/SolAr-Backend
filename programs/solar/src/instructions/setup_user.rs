use anchor_lang::prelude::*;
use crate::state::constants::*;
use crate::state::user::*;

pub fn setup_user(ctx: Context<SetupUser>, username: String, user_type: UserType) -> Result<()> {
    
    // Check for user type and if Judge/Admin, make sure that the Admin is adding 
    // match user_type {
    //     UserType::Admin => assert_eq!(ADMIN_PUB_KEY, ctx.accounts.authority.key().as_ref()),
    //     UserType::Judge => assert_eq!(ADMIN_PUB_KEY, ctx.accounts.authority.key().as_ref()),
    //     _ => {}
    // }
    
    ctx.accounts
        .user
        .initialize_user(ctx.accounts.authority.key(), username, user_type)
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
