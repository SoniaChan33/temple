use crate::error::ErrorCode;
use crate::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u16, treasury: Pubkey, incense_types: Vec<IncenseType>)]
pub struct CreateTempleConfig<'info> {
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [
            TempleConfig::SEED_PREFIX.as_bytes(),
            &index.to_string().as_bytes()
        ],
        bump,
        payer = owner,
        space = TempleConfig::INIT_SPACE
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn create_temple_config(
    ctx: Context<CreateTempleConfig>,
    index: u16,
    treasury: Pubkey,
    incense_types: Vec<IncenseType>,
) -> Result<()> {
    let temple_config: &mut Account<'_, TempleConfig> = &mut ctx.accounts.temple_config;
    temple_config.owner = ctx.accounts.owner.key();
    temple_config.index = index;
    temple_config.treasury = treasury;
    temple_config.incense_types = incense_types;
    Ok(())
}
