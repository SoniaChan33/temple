#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

pub mod admin {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("DRayqG9RXYi8WHgWEmRQGrUWRWbhjYWYkCRJDd6JBBak");
    #[cfg(not(feature = "devnet"))]
    pub const ID: Pubkey = pubkey!("GThUX1Atko4tqhN2NaiTazWSeFWMuiUvfFnyJyUghFMJ");
}

#[program]
pub mod temple {

    use super::*;

    /// 创建寺庙配置
    pub fn create_temple_config(ctx: Context<CreateTempleConfig>, index: u16) -> Result<()> {
        instructions::create_temple_config(ctx, index)
    }

    /// 创建NFT mint
    pub fn create_nft_mint(
        ctx: Context<CreateNftMint>,
        incese_id: u8,
        config_id: u16,
    ) -> Result<()> {
        instructions::create_nft_mint(ctx, incese_id, config_id)
    }

    /// 烧香
    pub fn burn_incense(ctx: Context<BurnIncense>, params: BurnIncenseParams) -> Result<()> {
        instructions::burn_incense::burn_incense(ctx, params)
    }
}
