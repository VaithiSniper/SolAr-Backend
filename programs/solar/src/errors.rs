use anchor_lang::error_code;

#[error_code]
pub enum CaseError {
    AlreadyStartedCase,
    PubKeyNotFound,
    NotAuthorized,
    AlreadyReachedCaseState,
    AlreadyDeclaredWinner,
}
