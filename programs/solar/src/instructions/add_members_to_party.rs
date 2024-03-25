use anchor_lang::prelude::*;
use crate::state::case::*;
use crate::state::UserProfile;
use crate::state::constants::*;

pub fn add_members_to_party(ctx: Context<AddMembersToParty>, member: Pubkey, party_type: PartyType) -> Result<()> {
    // assert_eq!(ctx.accounts.judge.key().as_ref(), ADMIN_PUB_KEY);

    // TODO: Have assertion with new access_role for judge
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

    #[account(mut)]
    pub judge: Signer<'info>,

    pub system_program: Program<'info, System>,
}
