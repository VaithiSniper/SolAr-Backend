use anchor_lang::error_code;

#[error_code]
pub enum CaseError {
    AlreadyStartedCase,
    PubKeyNotFound,
    NotAuthorized,
    AlreadyReachedCaseState,
    AlreadyDeclaredWinner,
}

#[error_code]
pub enum UnauthorizedError {
    #[msg("Unauthorized: You are not an admin to perform this action")]
    NotAdmin,
    #[msg("Unauthorized: You are not a judge to perform this action")]
    NotJudge,
    #[msg("Unauthorized: You are not a lawyer to perform this action")]
    NotLawyer,
    #[msg("Unauthorized: You are not a client to perform this action")]
    NotClient,
    #[msg("Unauthorized: You do not own this account to perform this action")]
    NoOwnershipOverAccount,
}

#[error_code]
pub enum UserError {
    #[msg("User is already verified")]
    AlreadyVerified,
}