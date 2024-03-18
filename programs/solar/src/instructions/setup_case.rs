use anchor_lang::prelude::*;
use crate::state::case::*;
use crate::state::constants::*;

pub fn setup_case(ctx: Context<SetupCase>, parties: [Pubkey; 2]) -> Result<()> {
    ctx.accounts.case.initialize_case(parties)
}

#[derive(Accounts)]
pub struct SetupCase<'info> {
    #[account(
        init, 
        seeds = [CASE_TAG, admin.key().as_ref()], 
        bump, 
        payer = admin, 
        space = Case::MAXIMUM_SIZE_FOR_RENT
        )]
    pub case: Account<'info, Case>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
