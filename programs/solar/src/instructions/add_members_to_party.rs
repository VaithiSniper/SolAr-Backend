use anchor_lang::prelude::*;
use crate::state::case::*;
use crate::state::{UserProfile, UserType};
use crate::state::constants::*;
use crate::errors::*;

pub fn add_members_to_party(ctx: Context<AddMembersToParty>, member: Pubkey, party_type: PartyType) -> Result<()> {
    require!(matches!(ctx.accounts.judge.type_of_user, UserType::Judge), UnauthorizedError::NotJudge);

    let user_account = &mut ctx.accounts.user;
    _ = user_account.add_case_to_user_account(ctx.accounts.case.id);
    ctx.accounts.case.add_member_to_party(member, party_type)
}

#[derive(Accounts)]
pub struct AddMembersToParty<'info> {
    #[account(mut)]
    pub case: Account<'info, Case>,

    #[account(mut)]
    pub user: Account<'info, UserProfile>,

    #[account()]
    pub judge: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}
