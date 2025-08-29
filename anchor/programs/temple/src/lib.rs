#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod temple {
    use super::*;

    pub fn close(_ctx: Context<CloseTemple>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.temple.count = ctx.accounts.temple.count.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.temple.count = ctx.accounts.temple.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn initialize(_ctx: Context<InitializeTemple>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.temple.count = value.clone();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTemple<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  init,
  space = 8 + Temple::INIT_SPACE,
  payer = payer
    )]
    pub temple: Account<'info, Temple>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseTemple<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  mut,
  close = payer, // close account and return lamports to payer
    )]
    pub temple: Account<'info, Temple>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub temple: Account<'info, Temple>,
}

#[account]
#[derive(InitSpace)]
pub struct Temple {
    count: u8,
}
