use anchor_lang::prelude::*;
use instructions::*;
use state::case::CaseState;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("D25GoHFNzhWrFciSxFxiTt6qEV93LYsv8vRZgqESQ9ni");

#[program]
pub mod solar {
    use super::*;

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
