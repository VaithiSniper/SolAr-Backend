use anchor_lang::prelude::*;
use crate::state::user::*;

pub fn verify_user(ctx: Context<VerifyUser>) -> Result<()> {
    // Get the user account
    let user_account = &mut ctx.accounts.user;
    // Can't be verified already
    assert_eq!(user_account.verified, false);
    // Get the admin account
    // let admin_account_type = ctx.accounts.admin.type_of_user;
    // match user_account.type_of_user {
    //     UserType::Client | UserType::Lawyer => assert_eq!(admin_account_type, UserType::Judge),
    //     UserType::Judge => assert_eq!(admin_account_type, UserType::Admin),
    //     _ => {
    //         // Do nothing
    //     }
    // }
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