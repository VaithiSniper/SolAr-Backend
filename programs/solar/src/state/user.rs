use anchor_lang::prelude::*;
use crate::state::MAX_MEMBERS_FOR_EACH_PARTY;

// Maximum number of cases that user can participate in
pub const MAX_NUMBER_OF_PARTICIPATING_CASES: usize = 5;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum UserType {
    Admin,
    Judge,
    Lawyer,
    Client,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub type_of_user: UserType,
    pub verified: bool,
    pub list_of_cases: [Pubkey; MAX_NUMBER_OF_PARTICIPATING_CASES],
    pub total_participating_cases: u8
}

impl UserProfile {
    pub const MAXIMUM_SIZE_FOR_RENT: usize = 8 + std::mem::size_of::<UserProfile>();

    pub fn initialize_user(
        &mut self,
        authority: Pubkey,
        username: String,
        user_type: UserType,
    ) -> Result<()> {
        // Initialize user profile with default data
        self.authority = authority;
        self.username = username;
        self.type_of_user = user_type;
        self.list_of_cases = [Pubkey::default(); MAX_NUMBER_OF_PARTICIPATING_CASES];
        self.total_participating_cases = 0;
        // Admin has to always be verified by default
        match self.type_of_user {
            UserType::Admin => self.verified = true,
            _ => self.verified = false,
        }

        Ok(())
    }

    pub fn initialize_user_profile(
        &mut self,
        email: String,
        first_name: String,
        last_name: String,
        phone: String,
    ) -> Result<()> {
        // Update the profile data values
        self.email = email;
        self.first_name = first_name;
        self.last_name = last_name;
        self.phone = phone;

        Ok(())
    }

    pub fn verify_user(&mut self) -> Result<()> {
        self.verified = true;

        Ok(())
    }

    pub fn add_case_to_user_account(&mut self, case_id: Pubkey) -> Result<()> {
        self.list_of_cases[self.total_participating_cases as usize] = case_id;
        self.total_participating_cases += 1;

        Ok(())
    }
}
