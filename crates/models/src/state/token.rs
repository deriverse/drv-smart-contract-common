use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

use super::types::Discriminator;

pub enum TokenType {
    Crncy,
    Asset,
    None,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default)]
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
