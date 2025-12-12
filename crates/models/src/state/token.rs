use bytemuck::{Pod, Zeroable};
#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;
#[cfg(feature = "off-chain")]
use solana_sdk::pubkey::Pubkey;

use super::types::Discriminator;

pub enum TokenType {
    Crncy,
    Asset,
    None,
}

/// Token State
///
/// 1. **`id`** - Unique id for token. DRVS token always has id = 0
/// 2. **`mask`**
///     - Token decimals = 0xFF
///     - Base crncy flag = 0x40000000
/// 4. **`base_crncy_index`** - Index of BaseCrncyRecord in CommunityState
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, PartialEq, Debug)]
pub struct TokenState {
    pub discriminator: Discriminator,
    pub address: Pubkey,
    pub program_address: Pubkey,
    pub id: u32,
    pub mask: u32,
    pub reserved: u32,
    pub base_crncy_index: u32,
}

pub const TOKEN_ACCOUNT_SIZE: usize = std::mem::size_of::<TokenState>();

impl ::std::ops::Deref for TokenState {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.discriminator
    }
}
