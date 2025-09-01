use crate::error::ErrorCode;
use crate::incense_nft::IncenseNFT;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_master_edition_v3;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMasterEditionV3;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
pub fn burn_incense(ctx: Context<BurnIncense>, params: BurnIncenseParams) -> Result<()> {
    // 解析香型ID
    let incense_id = params
        .incense_id
        .parse::<u8>()
        .map_err(|_| ErrorCode::InvalidIncenseId)?;
    // let incense_name: &'static str = match incense_id {
    //     0 => "Fresh Incense",
    //     1 => "Sandalwood Incense",
    //     2 => "Ambergris Incense",
    //     3 => "Supreme Spiritual Fragrance Incense",
    //     _ => return err!(ErrorCode::InvalidIncenseId),
    // };

    // 获取香的配置
    let incense_type: &IncenseType = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .unwrap();

    // 计算SOL总费用
    let fee_per_incense: u64 = ctx.accounts.temple_config.get_fee_per_incense(incense_id);
    let total_fee: u64 = fee_per_incense
        .checked_mul(params.amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // 验证用户SOL余额
    if ctx.accounts.authority.lamports() < total_fee {
        return err!(ErrorCode::InsufficientSolBalance);
    }

    // 转账
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.temple_treasury.to_account_info(),
            },
        ),
        total_fee,
    )?;

    // 生成NFT名称和序号（保持原逻辑）
    let number = ctx.accounts.nft_mint_account.supply;
    let nft_name = format!("{} #{}", incense_type.name, number + 1);

    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        &params.incense_id.as_bytes(),
        &[ctx.bumps.nft_mint_account],
    ]];

    // 创建元数据
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                update_authority: ctx.accounts.temple_authority.to_account_info(), // 寺庙
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: nft_name,
            symbol: IncenseNFT::TOKEN_SYMBOL.to_string(),
            uri: IncenseNFT::TOKEN_URL.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    )?;

    // Mint NFT给用户
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        params.amount, // 按数量mint
    )?;

    // 创建主版本
    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                update_authority: ctx.accounts.temple_authority.to_account_info(), // 寺庙拥有更新权限
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        None, // 需要限制发行量吗 不需要
    )?;

    // TODO 修改香火值

    // 修改寺庙功德和香火值

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: BurnIncenseParams)]
pub struct BurnIncense<'info> {
    /// 用户账号（付款方，签名者）
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 寺庙管理员账号
    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes(), temple_config.key().as_ref()],
        bump,
        seeds::program = crate::ID,
    )]
    pub temple_authority: AccountInfo<'info>,

    /// 寺庙
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

    /// 寺庙配置
    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
        seeds::program = crate::ID,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    /// nft mint
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes() , &params.incense_id.as_bytes()],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(), // 寺庙拥有冻结权限
    )]
    pub nft_mint_account: Account<'info, Mint>,

    /// 用户的NFT关联账户
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: this is the metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub meta_account: UncheckedAccount<'info>,

    /// CHECK: this is the metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub master_edition_account: UncheckedAccount<'info>,

    // 程序账号
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BurnIncenseParams {
    pub incense_id: String, // 香型ID
    pub amount: u64,        // 燃烧数量
}
