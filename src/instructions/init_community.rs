use anchor_lang::{
    prelude::*,
    system_program::{
        transfer,
        Transfer
    }
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::{
            instructions::{
                CreateMasterEditionV3Cpi,
                CreateMasterEditionV3CpiAccounts,
                CreateMasterEditionV3InstructionArgs,
                CreateMetadataAccountV3Cpi,
                CreateMetadataAccountV3CpiAccounts,
                CreateMetadataAccountV3InstructionArgs,
            },
            types::DataV2
        }, Metadata
    },
    token::{
        mint_to,
        Mint,
        MintTo,
        Token,
        TokenAccount
    }
};
use anchor_spl::metadata::mpl_token_metadata::instructions::{VerifyCollectionV1Cpi, VerifyCollectionV1CpiAccounts};
use anchor_spl::metadata::mpl_token_metadata::types::{Collection, Creator};
use crate::{
    constants::{
        MAX_COMMUNITY_NAME_LENGTH,
        ANCHOR_DISCRIMINATOR
    },
    errors::OSPError,
    state::{
        Authority,
        Community,
        Storage,
        Profile,
        ActivityCondition,
        JoinTicket,
        PermissionType::CommunityNullAccess
    },
    CommunityCreated
};

#[derive(Accounts)]
#[instruction(handle: String)]
pub struct InitCommunity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"storage"],
        bump,
    )]
    pub storage: Box<Account<'info, Storage>>,
    #[account(
    seeds = [b"collection_authority"],
    bump
    )]
    pub authority: Account<'info, Authority>,
    #[account(
        seeds = [b"profile", user.key.as_ref()],
        bump,
    )]
    pub profile: Box<Account<'info, Profile>>,
    #[account(
        init,
        payer = user,
        seeds = [b"community", handle.as_str().as_bytes()],
        bump,
        space = ANCHOR_DISCRIMINATOR + Community::INIT_SPACE,
    )]
    pub community: Box<Account<'info, Community>>,
    #[account(
        init,
        payer = user,
        seeds = [b"community_mint", community.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = community,
    )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    #[account(
    seeds = [b"collection_mint"],
    bump
    )]
    pub collection_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub ata: Box<Account<'info, TokenAccount>>,
    /// CHECK: no need to check this as the metaplex program will do it for us
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: no need to check this as the metaplex program will do it for us
    #[account(mut)]
    pub edition: UncheckedAccount<'info>,
    /// CHECK: no need to check
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,
    /// CHECK: no need to check
    #[account(mut)]
    pub collection_edition: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
    /// CHECK: no need to check
    pub sysvar_instructions: UncheckedAccount<'info>,
}

impl<'info> InitCommunity<'info> {
    pub fn init_community(&mut self, handle: String, activity_condition: Option<ActivityCondition>) -> Result<()> {
        self.storage.community_counter += 1;
        self.community.set_inner(Community {
            id: self.storage.community_counter,
            handle,
            activity_condition
        });

        Ok(())
    }

    pub fn community_purchase(&mut self) -> Result<()> {
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.user.to_account_info(),
                    to: self.authority.to_account_info()
                }
            ),
            self.storage.community_price
        )?;
        Ok(())
    }

    pub fn mint_community_nft(&mut self, uri: String, bumps: &InitCommunityBumps) -> Result<()> {
        let metadata_program = &self.metadata_program.to_account_info();
        let metadata = &self.metadata.to_account_info();
        let mint = &self.mint.to_account_info();
        let edition = &self.edition.to_account_info();
        let authority = &self.authority.to_account_info();
        let user = &self.user.to_account_info();
        let community = &self.community.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let creators = Some(vec![Creator {
            address: authority.key(),
            verified: true,
            share: 100
        }]);
        let community_signer = &[
            &b"community"[..],
            &self.storage.community_counter.to_le_bytes(),
            &[bumps.community],
        ];
        let authority_signer = &[
            &b"collection_authority"[..],
            &[bumps.authority]
        ];

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.ata.to_account_info(),
                    authority: self.community.to_account_info(),
                },
                &[community_signer]
            ),
            1
        )?;

        CreateMetadataAccountV3Cpi::new(
            metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: community,
                update_authority: (community, true),
                payer: user,
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "OpenSocial Protocol Community".to_owned(),
                    symbol: "OSPC".to_owned(),
                    uri,
                    seller_fee_basis_points: self.storage.seller_fee_basis_points,
                    creators: Some(vec![Creator {
                        address: authority.key(),
                        verified: true,
                        share: 100
                    }]),
                    collection: Some(Collection {
                        verified: false,
                        key: self.collection_metadata.key(),
                    }),
                    uses: None
                },
                is_mutable: true,
                collection_details: None,
            }
        ).invoke_signed(&[community_signer, authority_signer])?;

        CreateMasterEditionV3Cpi::new(
            metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition,
                mint,
                update_authority: community,
                mint_authority: community,
                payer: user,
                metadata,
                token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            }
        ).invoke_signed(&[community_signer])?;

        VerifyCollectionV1Cpi::new(
            metadata_program,
            VerifyCollectionV1CpiAccounts {
                authority,
                delegate_record: None,
                metadata,
                collection_mint: &self.collection_mint.to_account_info(),
                collection_metadata: Some(&self.collection_metadata.to_account_info()),
                collection_master_edition: Some(&self.collection_edition.to_account_info()),
                system_program,
                sysvar_instructions: &self.sysvar_instructions.to_account_info(),
            }
        ).invoke_signed(&[authority_signer])?;

        Ok(())
    }


    pub fn validate_handle(&mut self, handle: &str) -> Result<()> {
        require!(
            0 <= handle.len() && handle.len() <= MAX_COMMUNITY_NAME_LENGTH,
            OSPError::InvalidHandle
        );
        for element in handle.bytes() {
            require!(
                element.is_ascii_digit() || element.is_ascii_lowercase() || element.eq(&('_' as u8)),
                OSPError::InvalidHandle
            );
        }
        Ok(())
    }

    pub fn emit_init_community_event(&self, tags: Vec<String>, handle: String, ctx: Vec<u8>) -> Result<()> {
        emit!(CommunityCreated {
            community_id: self.community.id,
            address: self.user.key(),
            handle,
            tags: tags.clone(),
            timestamp: Clock::get()?.unix_timestamp,
            ctx
        });

        Ok(())
    }
}