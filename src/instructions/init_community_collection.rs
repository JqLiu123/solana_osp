use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
        Mint,
        MintTo,
        mint_to,
        Token,
        TokenAccount
    },
    metadata::{
        Metadata,
        mpl_token_metadata::{
            instructions::{
                CreateMasterEditionV3Cpi,
                CreateMasterEditionV3CpiAccounts,
                CreateMasterEditionV3InstructionArgs,
                CreateMetadataAccountV3Cpi,
                CreateMetadataAccountV3CpiAccounts,
                CreateMetadataAccountV3InstructionArgs
            },
            types::{
                CollectionDetails,
                Creator,
                DataV2
            }
        }
    },
    associated_token::AssociatedToken
};
use crate::state::Authority;

#[derive(Accounts)]
pub struct InitCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
    init,
    payer = payer,
    seeds = [b"collection_authority"],
    bump,
    space = 8 + Authority::INIT_SPACE
    )]
    pub authority: Account<'info, Authority>,
    #[account(
    init,
    payer = payer,
    seeds = [b"collection_mint"],
    bump,
    mint::decimals = 0,
    mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
    init,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = authority,
    )]
    pub ata: Account<'info, TokenAccount>,
    /// CHECK: no need to check
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: no need to check
    #[account(mut)]
    pub edition: UncheckedAccount<'info>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> InitCollection<'info> {

    pub fn init_collection(&mut self, name: String, symbol: String, uri: String, bumps: &InitCollectionBumps) -> Result<()> {
        self.authority.set_inner(Authority {
            receiver: self.payer.key(),
            rent: self.authority.to_account_info().lamports()
        });
        let metadata_program = &self.metadata_program.to_account_info();
        let metadata = &self.metadata.to_account_info();
        let mint = &self.mint.to_account_info();
        let edition = &self.edition.to_account_info();
        let authority = &self.authority.to_account_info();
        let payer = &self.payer.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let creators = Some(vec![Creator {
            address: authority.key(),
            verified: true,
            share: 100
        }]);
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
                    authority: self.authority.to_account_info()
                },
                &[authority_signer]
            ),
            1
        )?;



        CreateMetadataAccountV3Cpi::new(
            metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators,
                    collection: None,
                    uses: None
                },
                is_mutable: true,
                collection_details: Some(CollectionDetails::V1 {size: 0})
            }
        ).invoke_signed(&[authority_signer])?;

        CreateMasterEditionV3Cpi::new(
            metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition,
                mint,
                update_authority: authority,
                mint_authority: authority,
                payer,
                metadata,
                token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0)
            }
        ).invoke_signed(&[authority_signer])?;

        Ok(())
    }
}