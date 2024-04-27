use crate::state::case::*;
use anchor_lang::prelude::*;

pub fn reset_document_list_for_case(ctx: Context<ResetDocumentListForCase>) -> Result<()> {
    ctx.accounts.case.reset_document_list_for_case()
}

#[derive(Accounts)]
pub struct ResetDocumentListForCase<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}
