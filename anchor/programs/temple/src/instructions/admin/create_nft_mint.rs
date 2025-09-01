use crate::incense_nft::IncenseNFT;
use crate::temple_config::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

pub fn create_nft_mint(ctx: Context<CreateNftMint>, incense_id: u8, config_id: u16) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8, config_id: u16)]
pub struct CreateNftMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// nft mint
    #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            &[incense_id]
        ],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = authority.key(), // 寺庙拥有冻结权限
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            TempleConfig::SEED_PREFIX.as_bytes(),
            &config_id.to_be_bytes()
        ],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
