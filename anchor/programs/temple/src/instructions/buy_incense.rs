use crate::error::ErrorCode;
use crate::state::temple_config::*;
use crate::state::user_state::*;

use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

pub fn buy_incense(
    ctx: Context<BuyIncense>,
    incense_id: u8,
    config_id: u16,
    amount: u64,
) -> Result<()> {
    // 1. 校验amount>0
    if amount == 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    // 2. 获取该香型的单价（从temple_config中读取）
    let incense_type = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .ok_or(ErrorCode::InvalidIncenseId)?;
    let fee_per_incense = ctx.accounts.temple_config.get_fee_per_incense(incense_id);
    let total_fee = fee_per_incense
        .checked_mul(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // 3. 校验用户SOL余额
    if ctx.accounts.authority.lamports() < total_fee {
        return err!(ErrorCode::InsufficientSolBalance);
    }

    // 4. 转账SOL到寺庙国库
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

    // 5. 给用户增加香余额
    ctx.accounts
        .user_state
        .add_incense_balance(incense_id, amount);

    msg!(
        "User bought {} of incense type {} (total fee: {} SOL)",
        amount,
        incense_id,
        total_fee as f64 / 1e9
    ); // SOL=1e9 lamports
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8, config_id: u16)]
pub struct BuyIncense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This account is validated through the constraint that ensures it matches the treasury in temple_config
    #[account(mut, constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury)]
    pub temple_treasury: AccountInfo<'info>, // 寺庙国库

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes(), &config_id.to_string().as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        init_if_needed,
        payer = authority,
        space = UserState::INIT_SPACE, // 需定义UserState的空间大小（如256）
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
