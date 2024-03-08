use crate::state::case::*;
use anchor_lang::prelude::*;

pub fn declare_winner(ctx: Context<DeclareWinner>, party: Pubkey) -> Result<()> {
    ctx.accounts.case.declare_winner(party)
}

#[derive(Accounts)]
pub struct DeclareWinner<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,
}
