use anchor_lang::prelude::*;

use crate::constants::{
    MAX_ACTIVITY_URL_LENGTH
};

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub profile_id: u32,
    pub community_id: u32,
    #[max_len(MAX_ACTIVITY_URL_LENGTH)]
    pub content_uri: String,
    pub reference_profile_id: u32,
    pub reference_content_id: u32
}
