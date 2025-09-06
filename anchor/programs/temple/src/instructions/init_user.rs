use crate::state::user_state::UserState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserState::INIT_SPACE, // 8字节判别器 + 数据空间
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    let user = &ctx.accounts.user;
    let clock = Clock::get()?;

    // 初始化用户状态
    user_state.user = user.key();
    user_state.incense_points = 0;
    user_state.merit = 0;
    user_state.incense_number = 0;
    user_state.update_time = clock.unix_timestamp;

    // 初始化空的香型余额和每日计数数组
    user_state.incense_balance = Vec::new();
    user_state.daily_incense_count = Vec::new();

    // 设置bump
    user_state.bump = ctx.bumps.user_state;

    msg!("User state initialized for: {}", user.key());
    Ok(())
}
