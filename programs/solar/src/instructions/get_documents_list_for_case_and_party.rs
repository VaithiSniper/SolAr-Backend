use crate::state::case::*;
use anchor_lang::prelude::*;

pub fn get_documents_list_for_case_and_party(
    ctx: Context<FetchDocuments>,
    party_type: PartyType,
) -> Result<Vec<String>> {
    ctx.accounts
        .case
        .get_documents_list_for_case_and_party(party_type)
}

#[derive(Accounts)]
pub struct FetchDocuments<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}
