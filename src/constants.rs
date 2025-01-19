use anchor_lang::prelude::Pubkey;

pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const PUBKEY_SIZE: usize = 32;
pub const U64_SIZE: usize = 8;
pub const U32_SIZE: usize = 4;
pub const STRING_BORSH: usize = 4;
pub const USERNAME_SIZE: usize = 12;

pub const MAX_TAGS: usize = 10;
pub const MAX_HANDLE_LENGTH: usize = 15;
pub const MIN_HANDLE_LENGTH: usize = 4;
pub const MAX_COMMUNITY_NAME_LENGTH: usize = 63;
pub const MAX_ACTIVITY_URL_LENGTH: usize = 100;

pub const FOLLOW_NFT_NAME_SUFFIX: &str = "'s Follower";
pub const FOLLOW_NFT_SYMBOL_SUFFIX: &str = "-FOL";
pub const JOIN_NFT_NAME_SUFFIX: &str = "'s Member";
pub const JOIN_NFT_SYMBOL_SUFFIX: &str = "-MBR";

pub const USDC: Pubkey = Pubkey::new_from_array([
    198, 250, 122, 243, 190, 219, 173,  58, 61, 101, 243, 106, 171, 201, 116,  49,
    177, 187, 228, 194, 210, 246, 224, 228, 124, 166,   2,   3,  69,  47,  93,  97
]);