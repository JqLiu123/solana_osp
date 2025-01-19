use anchor_lang::prelude::*;
use crate::state::Currency;

#[event]
pub struct ProfileCreated {
    pub id: u32,
    pub handle: String,
    pub user: Pubkey,
    pub timestamp: i64,
    pub ctx: Vec<u8>
}

#[event]
pub struct Followed {
    pub follower: Pubkey,
    pub follower_profile_id: u32,
    pub profile_id: u32,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct Unfollowed {
    pub follower: Pubkey,
    pub follower_profile_id: u32,
    pub profile_id: u32,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct CommunityCreated {
    pub community_id: u32,
    pub address: Pubkey,
    pub handle: String,
    pub tags: Vec<String>,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct Joined {
    pub joiner: Pubkey,
    pub joiner_profile_id: u32,
    pub community_id: u32,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct UnJoined {
    pub user: Pubkey,
    pub user_profile_id: u32,
    pub community_id: u32,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct ActivityCreated {
    pub profile_id: u32,
    pub content_id: u32,
    pub community_id: u32,
    pub content_uri: String,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct CommentCreated {
    pub profile_id: u32,
    pub content_id: u32,
    pub community_id: u32,
    pub content_uri: String,
    pub reference_profile_id: u32,
    pub reference_content_id: u32,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct CommentDeleted {
    pub profile_id: u64,
    pub content_id: u64,
    pub community_id: u64,
    pub content_uri: String,
    pub reference_profile_id: u64,
    pub reference_content_id: u64,
    pub comment_id: u64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct OpenReactionCreated {
    pub profile_id: u32,
    pub community_id: u32,
    pub reference_profile_id: u32,
    pub reference_content_id: u32,
    pub reaction_value: u64,
    pub timestamp: i64,
    pub ctx: Vec<u8>,
}

#[event]
pub struct MegaphoneCreated {
    pub profile_id: u32,
    pub reference_profile_id: u32,
    pub reference_content_id: u32,
    pub tags: Vec<String>,
    pub start_time: i64,
    pub duration: u64,
    pub currency: Currency,
    pub amount: u64,
    pub ctx: Vec<u8>,
}