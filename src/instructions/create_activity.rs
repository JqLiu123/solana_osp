use std::cmp::{Ordering, PartialOrd};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::{
    Mint, 
    TokenAccount
};

use anchor_spl::metadata::MetadataAccount;

use crate::{
    errors::OSPError,
    ActivityCreated, 
    Community,
    Profile,
    constants::ANCHOR_DISCRIMINATOR
};
use crate::state::{ActivityCondition, Content, JoinTicket, ReferenceCondition};

#[derive(Accounts)]
#[instruction(uri: String)]
pub struct CreateActivity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"profile", user.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,
    pub community: Option<Account<'info, Community>>,
    #[account(
        init,
        payer = user,
        seeds = [b"content", user.key().as_ref(), (profile.content_counter + 1).to_le_bytes().as_ref()],
        space = ANCHOR_DISCRIMINATOR + Content::INIT_SPACE,
        bump,
    )]
    pub content: Account<'info, Content>,
    #[account(
        seeds = [
            b"join",
            join_ticket.community_key.as_ref(),
            user.key().as_ref()
        ],
        bump
    )]
    pub join_ticket: Option<Account<'info, JoinTicket>>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateActivity<'info> {
    pub fn create_activity(&mut self, uri: String, bumps: &CreateActivityBumps, reference_condition: Option<ReferenceCondition>, remaining_accounts: &[AccountInfo]) -> Result<()> {
        require!(uri != "", OSPError::InvalidContentURI);
        self.profile.content_counter += 1;
        self.content.set_inner(Content {
            content_id: self.profile.content_counter,
            profile_id: self.profile.id,
            community_id: match self.community.as_ref() {
                Some(community) => {
                    if self.join_ticket.is_none() {
                        return Err(OSPError::NotJoinedCommunity.into());
                    }
                    if let Some(condition) = &community.activity_condition {
                        match condition {
                            ActivityCondition::AccessActivityCondition(access) => {
                                if self.join_ticket.as_ref().unwrap().access_type < *access {
                                    return Err(OSPError::ActivityConditionNotMet.into());
                                }
                            },
                            ActivityCondition::HoldNFTActivityCondition(collection_addr) => {
                                if remaining_accounts.len() != 2 {
                                    return Err(OSPError::FollowConditionsNotMet.into());
                                }
                                let condition_nft_ata = &remaining_accounts[0];
                                let condition_nft_metadata = &remaining_accounts[1];

                                let mut data = condition_nft_metadata.try_borrow_mut_data()?;
                                let nft_metadata = MetadataAccount::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");

                                if let Some(collection) = &nft_metadata.collection {
                                    // 检查Collection地址是否匹配
                                    if collection.key != *collection_addr {
                                        return Err(OSPError::ActivityConditionNotMet.into());
                                    }

                                    // 检查Collection是否已验证
                                    if !collection.verified {
                                        return Err(OSPError::ActivityConditionNotMet.into());
                                    }
                                } else {
                                    // 如果没有collection字段，则返回错误
                                    return Err(OSPError::ActivityConditionNotMet.into())
                                };

                                let ata_address = get_associated_token_address(self.user.key, &nft_metadata.mint);
                                if ata_address != *condition_nft_ata.key {
                                    return Err(OSPError::ActivityConditionNotMet.into());
                                }

                                data = condition_nft_ata.try_borrow_mut_data()?;

                                let ata_account = TokenAccount::try_deserialize(&mut data.as_ref()).expect("Error Deserializing Data");
                                require!(ata_account.amount > 0, OSPError::ActivityConditionNotMet)
                            }
                        }
                    }
                    community.id
                },
                None => 0
            },
            uri: uri.clone(),
            reference_content_id: 0,
            reference_profile_id: 0,
            reference_condition,
        });

        Ok(())
    }

    pub fn emit_activity_created(&self, ctx: Vec<u8>) -> Result<()> {
        emit!(ActivityCreated {
            profile_id: self.profile.id,
            content_id: self.content.content_id,
            community_id: self.content.community_id,
            content_uri: self.content.uri.clone(),
            timestamp: Clock::get()?.unix_timestamp,
            ctx,
        });

        Ok(())
    }
}

