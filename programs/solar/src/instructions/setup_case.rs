use anchor_lang::prelude::*;
use crate::state::case::*;
use crate::state::constants::*;
 pub fn setup_case(ctx: Context<SetupCase>, judge: Pubkey, name: String) -> Result<()> {
        // TODO: Setup admin assertion as access_control macro

        // Creating embedded Party structs within Case
        let prosecutor = Party{
            size: 0,
            type_of_party: PartyType::Prosecutor,
            members: [Pubkey::default(); MAX_MEMBERS_FOR_EACH_PARTY]
        };
        let defendant = Party{
            size: 0,
            type_of_party: PartyType::Defendant,
            members: [Pubkey::default(); MAX_MEMBERS_FOR_EACH_PARTY]
        };

        ctx.accounts
         .case
         .setup_case(judge, name, prosecutor, defendant)
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
