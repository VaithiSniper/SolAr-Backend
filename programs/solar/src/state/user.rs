use anchor_lang::prelude::*;

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
}

impl UserProfile {
    // Maximum size for renAKWhaDu9yheRinzAeSrbkU35p5K1PkCF9wWQzJhg7xK1t
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
}
