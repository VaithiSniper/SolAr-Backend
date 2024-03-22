use anchor_lang::prelude::*;
use crate::state::UserType;
use crate::state::UserType;
use instructions::*;
use state::case::CaseState;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("WmiUzNXC9NadW5NiEaUUCHj8MSbcRyusELEBxemrryf");

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

    pub fn setup_case(ctx: Context<SetupCase>, parties: [Pubkey; 2]) -> Result<()> {
        instructions::setup_case(ctx, parties)
    }

    pub fn declare_winner(ctx: Context<DeclareWinner>, party: Pubkey) -> Result<()> {
        instructions::declare_winner(ctx, party)
    }

    pub fn set_case_state(ctx: Context<SetCaseState>, case_state: CaseState) -> Result<()> {
        instructions::set_case_state(ctx, case_state)
    }
}
