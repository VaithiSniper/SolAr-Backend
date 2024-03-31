use anchor_lang::prelude::*;
use crate::state::user::*;
use crate::errors::*;

pub fn verify_user(ctx: Context<VerifyUser>) -> Result<()> {
    // Get the user account
    let user_account = &mut ctx.accounts.user;
    // Can't be verified already
    require!(!user_account.verified, UserError::AlreadyVerified);
    // Get the admin account
    let admin_account_type = ctx.accounts.admin.type_of_user;
    match user_account.type_of_user {
        UserType::Judge => require!(matches!(admin_account_type, UserType::Admin), UnauthorizedError::NotAdmin),
        UserType::Client | UserType::Lawyer =>  require!(matches!(admin_account_type, UserType::Judge),  UnauthorizedError::NotJudge),
        _ => {}
    }
    user_account.verify_user()
}

#[derive(Accounts)]
pub struct VerifyUser<'info> {
    #[account(mut)]
    pub user: Account<'info, UserProfile>,

    #[account()]
    pub admin: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}