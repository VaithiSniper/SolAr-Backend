use crate::state::case::*;
use anchor_lang::prelude::*;

pub fn setup_case(ctx: Context<SetupCase>, parties: [Pubkey; 2]) -> Result<()> {
    ctx.accounts.case.start(parties)
}

#[derive(Accounts)]
pub struct SetupCase<'info> {
    #[account(init, payer = admin, space = Case::MAXIMUM_SIZE_FOR_RENT + 8)]
    pub case: Account<'info, Case>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}