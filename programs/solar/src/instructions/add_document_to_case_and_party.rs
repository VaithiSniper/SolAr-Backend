use crate::state::case::*;
use anchor_lang::prelude::*;

pub fn add_document_to_case_and_party(
    ctx: Context<AddDocumentToCaseAndParty>,
    party_type: PartyType,
    doc_id: String,
) -> Result<()> {
    ctx.accounts
        .case
        .add_document_to_case_and_party(party_type, doc_id)
}

#[derive(Accounts)]
pub struct AddDocumentToCaseAndParty<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}
