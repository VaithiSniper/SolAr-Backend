use anchor_lang::prelude::*;
use num_derive::*;
use solana_program::entrypoint::ProgramResult;
// use crate::state::CaseState::ToStart;

// Maximum number of members that can participate in a case
pub const MAX_MEMBERS_FOR_EACH_PARTY: usize = 5;

// Structs we'll need for Case struct
#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum PartyType {
    Prosecutor,
    Defendant,
}

impl PartyType {
    pub fn from_usize(index: usize) -> Option<PartyType> {
        match index {
            0 => Some(PartyType::Prosecutor),
            1 => Some(PartyType::Defendant),
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub mime_type: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Party {
    pub type_of_party: PartyType,
    pub members: [Pubkey; MAX_MEMBERS_FOR_EACH_PARTY],
    pub size: u8,
    pub documents: Vec<String>,
}

// Implement the Default trait for the Party struct
impl Default for Party {
    fn default() -> Self {
        Party {
            type_of_party: PartyType::Prosecutor,
            members: [Pubkey::default(); MAX_MEMBERS_FOR_EACH_PARTY],
            size: 0,
            documents: Vec::new(), // Empty vector for documents
        }
    }
}

// TODO: Add documents section under each party
impl Party {
    // Method to add a new document to the list
    pub fn add_document(&mut self, id: String) {
        self.documents.push(id);
    }

    // Method to remove a document from the list
    pub fn remove_document(&mut self, id: String) {
        self.documents.retain(|doc_id| *doc_id != id);
    }

    // Method to retrieve a document from the list by ID
    pub fn get_document(&self, id: String) -> Option<&String> {
        self.documents.iter().find(|doc_id| **doc_id == id)
    }
}

#[account]
pub struct Case {
    pub name: String,
    pub judge: Pubkey,
    pub prosecutor: Party,
    pub defendant: Party,
    pub case_winner: Option<PartyType>,
    pub case_state: CaseState,
}

impl Case {
    // Maximum size for rent
    pub const MAXIMUM_SIZE_FOR_RENT: usize = 8 + std::mem::size_of::<Case>();

    // To define defaults for a new case
    pub fn setup_case(
        &mut self,
        judge: Pubkey,
        name: String,
        prosecutor: Party,
        defendant: Party,
    ) -> Result<()> {
        self.judge = judge;
        self.name = name;
        self.prosecutor = prosecutor;
        self.defendant = defendant;

        Ok(())
    }

    // To check if pubkey exists in array
    fn pubkey_exists(pubkeys: [Pubkey; 5], pubkey_to_check: &Pubkey) -> bool {
        pubkeys
            .iter()
            .any(|&existing_pubkey| existing_pubkey == *pubkey_to_check)
    }

    // To add a member to a case
    pub fn add_member_to_party(&mut self, member: Pubkey, party_type: PartyType) -> Result<()> {
        // Check if member is already present and prevent addition in such case
        match party_type {
            PartyType::Prosecutor => {
                // assert!(Self::pubkey_exists(self.prosecutor.members, &member));
                self.prosecutor.members[self.prosecutor.size as usize] = member;
                self.prosecutor.size += 1;
            }
            PartyType::Defendant => {
                // assert!(Self::pubkey_exists(self.defendant.members, &member));
                self.defendant.members[self.defendant.size as usize] = member;
                self.defendant.size += 1;
            }
        }

        Ok(())
    }

    // To minimize storage cost, we will use a binary type to determine who should be declared winner
    // 0 - false - Prosecutor
    // 1 - true - Defendant
    pub fn declare_winner(&mut self, party: bool) -> Result<()> {
        if party {
            self.case_winner = Option::from(PartyType::Prosecutor);
        } else {
            self.case_winner = Option::from(PartyType::Defendant);
        }

        Ok(())
    }

    pub fn set_case_state(&mut self, new_case_status: CaseState) -> Result<()> {
        // require_neq!(new_case_status, &self.case_state, CaseError::AlreadyReachedCaseState);
        self.case_state = new_case_status;

        Ok(())
    }

    // Method to add a new document to the list
    pub fn add_document_to_case_and_party(
        &mut self,
        party_type: PartyType,
        doc_id: String,
    ) -> Result<()> {
        match party_type {
            PartyType::Prosecutor => {
                self.prosecutor.documents.push(doc_id);
            }
            PartyType::Defendant => {
                self.defendant.documents.push(doc_id);
            }
        }

        Ok(())
    }


    // Method to fetch document list
    pub fn get_documents_list_for_case_and_party(
        &mut self,
        party_type: PartyType,
    ) -> Result<Vec<String>> {
        match party_type {
            PartyType::Prosecutor => {
                Ok(self.prosecutor.documents.clone())
            }
            PartyType::Defendant => {
                Ok(self.defendant.documents.clone())
            }
        }

    }

    // // Method to remove a document from the list
    // pub fn remove_document(&mut self, id: String) {
    //     self.documents.retain(|doc_id| *doc_id != id);
    // }
    //
    // // Method to retrieve a document from the list by ID
    // pub fn get_document(&self, id: String) -> Option<&String> {
    //     self.documents.iter().find(|doc_id| **doc_id == id)
    // }
}
