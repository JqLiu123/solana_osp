use anchor_lang::prelude::*;
use crate::constants::MAX_HANDLE_LENGTH;

#[derive(Clone, PartialEq, Eq, InitSpace, AnchorDeserialize, AnchorSerialize, Debug)]
pub enum FollowCondition {

    IsFollowing {
        #[max_len(MAX_HANDLE_LENGTH)]
        handle: String
    },
    MinimumFollowers(u32)
}
