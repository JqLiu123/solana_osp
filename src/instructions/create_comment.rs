use anchor_lang::prelude::*;
use crate::{
    constants::ANCHOR_DISCRIMINATOR, errors::OSPError, CommentCreated, Community, Content,
    FollowTicket, JoinTicket, Profile, ReferenceCondition,
};

#[derive(Accounts)]
#[instruction(uri: String)]
pub struct CreateComment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"profile", user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,
    #[account(mut)]
    pub reference_content: Account<'info, Content>,
    #[account(
        init,
        payer = user,
        seeds = [b"content", user.key().as_ref(), (profile.content_counter + 1).to_le_bytes().as_ref()],
        space = ANCHOR_DISCRIMINATOR + Content::INIT_SPACE,
        bump,
    )]
    pub content: Account<'info, Content>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateComment<'info> {
    pub fn create_comment(
        &mut self,
        uri: String,
        bumps: &CreateCommentBumps,
        reference_condition: Option<ReferenceCondition>,
    ) -> Result<()> {
        require!(uri != "", OSPError::InvalidContentURI);
        self.profile.content_counter += 1;

        self.content.set_inner(Content {
            content_id: self.profile.content_counter,
            profile_id: self.profile.id,
            community_id: self.reference_content.community_id,
            uri: uri.clone(),
            reference_content_id: self.reference_content.content_id,
            reference_profile_id: self.reference_content.profile_id,
            reference_condition,
        });

        Ok(())
    }

    pub fn check_comment_conditions(&mut self, remaining_accounts: &[AccountInfo]) -> Result<()> {
        if let Some(condition) = &self.reference_content.reference_condition {
            match condition {
                ReferenceCondition::OnlyFollowers => {
                    if remaining_accounts.len() != 3 {
                        return Err(OSPError::FollowConditionsNotMet.into());
                    }

                    let reference_profile = &remaining_accounts[0];
                    let reference_user = &remaining_accounts[1];
                    let follow_ticket = &remaining_accounts[2];

                    let mut data = reference_profile.try_borrow_mut_data()?;
                    let is_following_profile = Profile::try_deserialize(&mut data.as_ref())
                        .expect("Error Deserializing Data");
                    // check profile id
                    require_eq!(
                        is_following_profile.id,
                        self.reference_content.profile_id,
                        OSPError::FollowConditionsNotMet
                    );

                    let profile_address = Pubkey::find_program_address(
                        &[b"profile", reference_user.key().as_ref()],
                        &crate::ID,
                    )
                    .0;
                    // check profile user
                    require_keys_eq!(
                        reference_profile.key(),
                        profile_address,
                        OSPError::FollowConditionsNotMet
                    );

                    let follow_ticket_address = Pubkey::find_program_address(
                        &[
                            b"follow",
                            reference_user.key().as_ref(),
                            &self.user.key().as_ref(),
                        ],
                        &crate::ID,
                    )
                    .0;
                    data = follow_ticket.try_borrow_mut_data()?;
                    FollowTicket::try_deserialize(&mut data.as_ref())
                        .expect("Error Deserializing Data");
                    // check follow ticket
                    require_keys_eq!(
                        follow_ticket.key(),
                        follow_ticket_address,
                        OSPError::FollowConditionsNotMet
                    );
                }
                ReferenceCondition::SameCommunity => {
                    if self.reference_content.community_id != 0 {
                        if remaining_accounts.len() != 2 {
                            return Err(OSPError::FollowConditionsNotMet.into());
                        }

                        let reference_community = &remaining_accounts[0];
                        let reference_join_ticket = &remaining_accounts[1];

                        let mut data = reference_community.try_borrow_mut_data()?;
                        let community = Community::try_deserialize(&mut data.as_ref())
                            .expect("Error Deserializing Data");
                        // check community id
                        require_eq!(
                            community.id,
                            self.reference_content.community_id,
                            OSPError::FollowConditionsNotMet
                        );

                        let join_ticket_address = Pubkey::find_program_address(
                            &[
                                b"join",
                                reference_community.key().as_ref(),
                                &self.user.key().as_ref(),
                            ],
                            &crate::ID,
                        )
                        .0;
                        data = reference_join_ticket.try_borrow_mut_data()?;
                        JoinTicket::try_deserialize(&mut data.as_ref())
                            .expect("Error Deserializing Data");
                        // check join ticket
                        require_keys_eq!(
                            reference_join_ticket.key(),
                            join_ticket_address,
                            OSPError::FollowConditionsNotMet
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub fn emit_comment_created(&self, ctx: Vec<u8>) -> Result<()> {
        emit!(CommentCreated {
            profile_id: self.profile.id,
            content_id: self.content.content_id,
            community_id: self.content.community_id,
            content_uri: self.content.uri.clone(),
            reference_content_id: self.content.reference_content_id,
            reference_profile_id: self.content.reference_profile_id,
            timestamp: Clock::get()?.unix_timestamp,
            ctx,
        });

        Ok(())
    }
}
