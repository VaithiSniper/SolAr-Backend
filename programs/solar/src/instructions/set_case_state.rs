use crate::errors::*;
use crate::state::case::*;
use crate::state::user::*;
use anchor_lang::prelude::*;

pub fn set_case_state(ctx: Context<SetCaseState>, case_state: CaseState) -> Result<()> {
    require!(
        matches!(ctx.accounts.judge.type_of_user, UserType::Judge),
        UnauthorizedError::NotJudge
    );

    ctx.accounts.case.set_case_state(case_state)
}

#[derive(Accounts)]
pub struct SetCaseState<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,

    #[account()]
    pub judge: Account<'info, UserProfile>,
}
