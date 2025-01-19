use anchor_lang::prelude::*;

use crate::constants::{
    ANCHOR_DISCRIMINATOR,
    STRING_BORSH,
    MAX_COMMUNITY_NAME_LENGTH,
    U64_SIZE
};
use crate::state::ActivityCondition;

#[account]
#[derive(InitSpace)]
pub struct Community {
    pub id: u32,
    #[max_len(MAX_COMMUNITY_NAME_LENGTH)]
    pub handle: String,
    pub activity_condition: Option<ActivityCondition>
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, InitSpace, AnchorDeserialize, AnchorSerialize, Debug)]
pub enum PermissionType {
    CommunityNullAccess = 0,
    CommunityModeratorAccess = 1,
    CommunityAdminAccess = 2,
}

#[account]
#[derive(InitSpace)]
pub struct JoinTicket {
    pub level: u16,
    pub access_type: PermissionType,
    pub is_black: bool,
    pub community_key: Pubkey,
}