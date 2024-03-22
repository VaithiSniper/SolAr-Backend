use anchor_lang::prelude::*;
use num_derive::*;
// use crate::state::CaseState::ToStart;

// Structs we'll need for Case struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Winner {
    Prosecutor,
    Defendant,
}

impl Winner {
    pub fn from_usize(index: usize) -> Option<Winner> {
        match index {
            0 => Some(Winner::Prosecutor),
            1 => Some(Winner::Defendant),
            _ => None,
        }
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum CaseState {
    ToStart,
    WaitingForParticipants,
    Active,
    AwaitingRuling,
    Disposed,
    Completed,
}

#[account]
pub struct Case {
    parties: [Pubkey; 2],        // (32*2)
    case_winner: Option<Winner>, // (2)
    case_state: CaseState,       // (1)
}

impl Case {
    // Maximum size for rent
    pub const MAXIMUM_SIZE_FOR_RENT: usize = 8 + std::mem::size_of::<Case>();

    // To create a new case
    pub fn initialize_case(&mut self, parties: [Pubkey; 2]) -> Result<()> {
        // require_eq!(self.case_state, CaseState::ToStart, CaseError::AlreadyStartedCase);
        self.case_state = CaseState::WaitingForParticipants;
        self.parties = parties;

        Ok(())
    }

    pub fn declare_winner(&mut self, party: Pubkey) -> Result<()> {
        let index = self.parties.iter().position(|&x| x == party);
        // require_neq!(index, None, CaseError::PubKeyNotFound);
        // require_neq!(&self.case_winner, None, CaseError::AlreadyDeclaredWinner);
        let index_val = index.unwrap_or_else(|| 0);
        let winner = Some(Winner::from_usize(index_val));
        let winner_val = winner.unwrap();
        self.case_winner = winner_val;

        Ok(())
    }

    pub fn set_case_state(&mut self, new_case_status: CaseState) -> Result<()> {
        // require_neq!(new_case_status, &self.case_state, CaseError::AlreadyReachedCaseState);
        self.case_state = new_case_status;

        Ok(())
    }
}
