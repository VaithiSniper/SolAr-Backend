use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub cases_count: u8,
    pub latest_case: u8,
}

impl UserProfile {
    // Maximum size for rent
    pub const MAXIMUM_SIZE_FOR_RENT: usize = 8 + std::mem::size_of::<UserProfile>();

    pub fn initialize_user(&mut self, authority: Pubkey, username: String) -> Result<()> {
        // Initialize user profile with default data
        self.authority = authority;
        self.username = username;
        self.cases_count = 0;
        self.latest_case = 0;

        Ok(())
    }

    pub fn initialize_user_profile(&mut self, email: String, first_name: String, last_name: String, phone: String) -> Result<()> {
        // Update the profile data values
        self.email = email;
        self.first_name = first_name;
        self.last_name = last_name;
        self.phone = phone;

        Ok(())
    }

}
