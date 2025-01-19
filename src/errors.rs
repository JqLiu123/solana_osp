use anchor_lang::error_code;
#[error_code]
pub enum OSPError {
    #[msg("Not profile owner")]
    NotProfileOwner,
    #[msg("Handle too long")]
    InvalidHandleLength,
    #[msg("Handle Invalid")]
    InvalidHandle,
    #[msg("Follow conditions not met")]
    FollowConditionsNotMet,
    #[msg("Comment conditions not met")]
    CommentConditionsNotMet,
    #[msg("Cannot follow self")]
    CannotFollowSelf,
    #[msg("Discriminator not valid")]
    DiscriminatorNotValid,
    #[msg("Already following")]
    AlreadyFollowing,
    #[msg("Already joined community")]
    AlreadyJoinedCommunity,
    #[msg("Did not join community")]
    DidNotJoinCommunity,
    #[msg("Invalid community ID")]
    InvalidCommunityId,
    #[msg("Too many tags")]
    TooManyTags,
    #[msg("Inavlid Content URI")]
    InvalidContentURI,
    #[msg("Megaphone not expired")]
    MegaphoneNotExpired,
    #[msg("Not joined community")]
    NotJoinedCommunity,
    #[msg("Activity condition not met")]
    ActivityConditionNotMet,
    #[msg("No Permission")]
    NoPermission,
}