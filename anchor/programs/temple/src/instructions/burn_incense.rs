use crate::error::ErrorCode;
use crate::incense_nft::IncenseNFT;
use crate::state::temple_config::*;
use crate::state::user_state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_master_edition_v3;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMasterEditionV3;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::burn;
use anchor_spl::token::mint_to;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
pub fn burn_incense(
    ctx: Context<BurnIncense>,
    incense_id: u8,
    config_id: u16,
    amount: u64,
) -> Result<()> {
    let incense_type = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .ok_or(ErrorCode::InvalidIncenseId)?;

    let incense_points = incense_type.incense_points as u64;
    let merit = incense_type.merit as u64;

    // 检查用户烧香次数是否超过每日限制
    ctx.accounts.user_state.check_daily_incense_limit(incense_id, amount as u8)?;

    // 检查并扣减香余额（替代SOL支付）
    let current_balance = ctx.accounts.user_state.get_incense_balance(incense_id);
    if current_balance < amount {
        return err!(ErrorCode::InsufficientIncenseBalance);
    }
    ctx.accounts.user_state.subtract_incense_balance(incense_id, amount)?;
    msg!("Consumed {} incense of type {} from user balance", amount, incense_id);

    // 生成NFT名称和序号
    let number = ctx.accounts.nft_mint_account.supply;
    let nft_name = format!("{} #{}", incense_type.name, number + 1);

    // seeds = [IncenseNFT::SEED_PREFIX.as_bytes(), temple_config.key().as_ref(), params.incense_id.as_bytes()],
    let temple_config_key: Pubkey = ctx.accounts.temple_config.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        temple_config_key.as_ref(),
        &[incense_id],
        &[ctx.bumps.nft_mint_account],
    ]];

    // 创建元数据
    if ctx.accounts.nft_mint_account.supply == 0 {
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
                update_authority: ctx.accounts.temple_authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        None, // 需要限制发行量吗 不需要
    )?;
}

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
        amount,
    )?;
    msg!("NFT minted successfully");

    // 立即销毁NFT（消耗品模式）
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    msg!("NFT burned successfully - consumable incense used");

    // 更新每日烧香次数
    ctx.accounts.user_state.update_daily_count(incense_id, amount as u8);

    // 更新用户的香火值和功德值
    ctx.accounts
        .user_state
        .add_incense_value_and_merit(incense_points * amount, merit * amount);

    // 修改寺庙功德和香火值
    ctx.accounts
        .temple_config
        .add_incense_value_and_merit(incense_points * amount, merit * amount);

    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8, config_id: u16)]
pub struct BurnIncense<'info> {
    /// 用户账号（付款方，签名者）
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: 寺庙管理员账号
    #[account(mut,
        constraint = temple_authority.key() == temple_config.owner @ ErrorCode::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// CHECK: 寺庙存储sol的帐号
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,


    #[account(
        mut,
        seeds = [
            TempleConfig::SEED_PREFIX.as_bytes(),
            &config_id.to_string().as_bytes(),
        ],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    /// 用户账号
    #[account(
        init_if_needed,
        payer = authority, // TODO 用户的账号需要谁付钱？
        space = 100, 
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// nft mint
    #[account(
        mut,
        seeds = [IncenseNFT::SEED_PREFIX.as_bytes(), temple_config.key().as_ref(), &[incense_id]],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(), // 寺庙拥有冻结权限
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

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
    pub config_id: u16,
}
