use crate::error::ErrorCode;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;
#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateTempleConfig<'info> {
    /// Address to be set as protocol owner.
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    /// Initialize config state account to store protocol owner address and fee rates.
    #[account(
        init_if_needed,
        seeds = [
            TempleConfig::SEED_PREFIX.as_bytes(),
            &index.to_be_bytes()
        ],
        bump,
        payer = owner,
        space = TempleConfig::INIT_SPACE
    )]
    pub temple_config: Account<'info, TempleConfig>,

    pub system_program: Program<'info, System>,
}

pub fn create_temple_config(ctx: Context<CreateTempleConfig>, index: u16) -> Result<()> {
    Ok(())
}
