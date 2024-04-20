use anchor_lang::prelude::*;
use crate::state::case::*;
use crate::state::constants::*;
use crate::errors::*;
use crate::state::UserProfile;

pub fn setup_case(ctx: Context<SetupCase>, _bump: u8, name: String) -> Result<()> {
        // TODO: Setup admin assertion as access_control macro
        require_keys_eq!(ADMIN_PUB_KEY, ctx.accounts.admin.key(), UnauthorizedError::NotAdmin);

        // Creating embedded Party structs within Case
        let mut prosecutor= Party::default();
        prosecutor.type_of_party = PartyType::Prosecutor;
        let mut defendant= Party::default();
        defendant.type_of_party = PartyType::Prosecutor;

        let judge= &mut ctx.accounts.judge;

        _ = ctx.accounts
         .case
         .setup_case(judge.authority, name, prosecutor, defendant);
        
        judge.add_case_to_user_account(ctx.accounts.case.key())
 }

#[derive(Accounts)]
#[instruction(_bump: u8, name:String)]
pub struct SetupCase<'info> {
    #[account(
        init,
        seeds = [CASE_TAG, judge.key().as_ref(), &[_bump]],
        bump,
        payer = admin, 
        space = Case::MAXIMUM_SIZE_FOR_RENT
        )]
    pub case: Account<'info, Case>,

    #[account(mut)]
    pub judge: Account<'info, UserProfile>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
