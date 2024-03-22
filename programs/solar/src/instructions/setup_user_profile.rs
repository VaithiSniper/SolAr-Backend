use anchor_lang::prelude::*;
use crate::state::user::*;

pub fn setup_user_profile(ctx: Context<SetupUserProfile>, email: String, first_name: String, last_name: String, phone: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user;
    assert_eq!(user_account.authority, ctx.accounts.authority.key());
    user_account.initialize_user_profile(email, first_name, last_name, phone)
}

#[derive(Accounts)]
pub struct SetupUserProfile<'info> {
    #[account(mut)]
    pub user: Account<'info, UserProfile>,

    #[account()]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
