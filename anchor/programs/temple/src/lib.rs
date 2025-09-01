#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use crate::temple_config::IncenseType;
use instructions::*;
use state::*;

declare_id!("5iZVCAG6GAq3wdVL31Hy2eTybnUEYkgvnamqdQETAPUK");

// todo 这个设置的用途是？
pub mod admin {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("DRayqG9RXYi8WHgWEmRQGrUWRWbhjYWYkCRJDd6JBBak");
    #[cfg(feature = "localnet")]
    pub const ID: Pubkey = pubkey!("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU");
    #[cfg(not(any(feature = "devnet", feature = "localnet")))]
    pub const ID: Pubkey = pubkey!("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU");
}

#[program]
pub mod temple {

    use super::*;

    /// 创建寺庙配置
    pub fn create_temple_config(
        ctx: Context<CreateTempleConfig>,
        index: u16,
        treasury: Pubkey,
        incense_types: Vec<IncenseType>,
    ) -> Result<()> {
        instructions::create_temple_config(ctx, index, treasury, incense_types)
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
    pub fn burn_incense(
        ctx: Context<BurnIncense>,
        config_id: u16,
        params: BurnIncenseParams,
    ) -> Result<()> {
        instructions::burn_incense::burn_incense(ctx, config_id, params)
    }
}
