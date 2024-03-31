use anchor_lang::prelude::*;
use anchor_lang::solana_program;

#[constant]
pub const USER_TAG: &[u8] = b"USER_STATE";

#[constant]
pub const CASE_TAG: &[u8] = b"CASE_STATE";

#[constant]
pub const CASE_ID_TAG: &[u8] = b"CASE_ID_STATE";

#[constant]
pub const PROSECUTOR_TAG: &[u8] = b"PROSECUTOR_STATE";

#[constant]
pub const DEFENDANT_TAG: &[u8] = b"DEFENDANT_STATE";

#[constant]
pub const ADMIN_PUB_KEY: Pubkey = solana_program::pubkey!("6gHNdTY6JhB5x5SnArg6XkoTNH7one7aUByWyYWM2AJj");