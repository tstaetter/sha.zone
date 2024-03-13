/// All supported token types
pub enum TokenType {
    /// Used when a registered user requests a file from a 3rd party
    /// being sent to his account
    FileRequestToken,
    /// Used for refreshing UserTokens
    RefreshToken,
    /// Used while uploading files
    TransitionToken,
    /// Common JWT token for user identification
    UserToken,
}
