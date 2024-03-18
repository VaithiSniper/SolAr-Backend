use anchor_lang::prelude::*;
use crate::state::case::*;

pub fn set_case_state(ctx: Context<SetCaseState>, case_state: CaseState) -> Result<()> {
    ctx.accounts.case.set_case_state(case_state)
}

#[derive(Accounts)]
pub struct SetCaseState<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,
}
