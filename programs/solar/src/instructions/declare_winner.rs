use crate::state::case::*;
use crate::state::user::*;
use anchor_lang::prelude::*;

pub fn declare_winner(ctx: Context<DeclareWinner>, party: bool) -> Result<()> {
    // require!(matches!(ctx.accounts.judge.type_of_user, UserType::Judge), UnauthorizedError::NotJudge);

    ctx.accounts.case.declare_winner(party)
}

#[derive(Accounts)]
pub struct DeclareWinner<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,

    #[account(mut)]
    pub judge: Account<'info, UserProfile>,
}
