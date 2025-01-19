use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum OpenReaction {
    Like = 0,
    VoteUp = 1,
    VoteDown = 2,
    VoteCancel = 3,
}