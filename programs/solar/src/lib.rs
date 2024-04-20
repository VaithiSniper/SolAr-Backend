use crate::state::UserType;
use anchor_lang::prelude::*;
use instructions::*;
use state::case::CaseState;
use state::case::PartyType;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("DL79DkQFevrJrjVFhi8W2sSgaxizW84vwxviTggQThvY");

#[program]
pub mod solar {

    use super::*;

    pub fn verify_user(ctx: Context<VerifyUser>) -> Result<()> {
        instructions::verify_user(ctx)
    }

    pub fn setup_user(
        ctx: Context<SetupUser>,
        username: String,
        user_type: UserType,
    ) -> Result<()> {
        instructions::setup_user(ctx, username, user_type)
    }

    pub fn setup_user_profile(
        ctx: Context<SetupUserProfile>,
        email: String,
        first_name: String,
        last_name: String,
        phone: String,
    ) -> Result<()> {
        instructions::setup_user_profile(ctx, email, first_name, last_name, phone)
    }

    pub fn setup_case(ctx: Context<SetupCase>, bump: u8, name: String) -> Result<()> {
        instructions::setup_case(ctx, bump, name)
    }

    pub fn add_member_to_party(
        ctx: Context<AddMembersToParty>,
        member: Pubkey,
        party_type: PartyType,
    ) -> Result<()> {
        instructions::add_members_to_party(ctx, member, party_type)
    }

    pub fn add_document_to_case_and_party(
        ctx: Context<AddDocumentToCaseAndParty>,
        party_type: PartyType,
        doc_id: String,
    ) -> Result<()> {
        instructions::add_document_to_case_and_party(ctx, party_type, doc_id)
    }

    pub fn get_documents_list_for_case_and_party(
        ctx: Context<FetchDocuments>,
        party_type: PartyType,
        doc_id: String,
    ) -> Result<Vec<String>> {
        instructions::get_documents_list_for_case_and_party(ctx, party_type)
    }

    pub fn declare_winner(ctx: Context<DeclareWinner>, party: bool) -> Result<()> {
        instructions::declare_winner(ctx, party)
    }

    pub fn set_case_state(ctx: Context<SetCaseState>, case_state: CaseState) -> Result<()> {
        instructions::set_case_state(ctx, case_state)
    }
}
