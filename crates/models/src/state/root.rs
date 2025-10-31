use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

use std::ops::Deref;

use super::types::Discriminator;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, Debug)]
pub struct RootState {
    pub discriminator: Discriminator,
    pub operator_address: Pubkey,
    pub holder_address: Pubkey,
    pub drvs_mint_address: Pubkey,
    pub lut_address: Pubkey,
    pub airdrop_authority_address: Pubkey,
    pub ref_program_duration: u32,
    pub ref_link_duration: u32,
    pub ref_discount: f64,
    pub ref_ratio: f64,
    pub clients_count: u32,
    pub tokens_count: u32,
    pub instr_count: u32,
    pub ref_counter: u32,
    pub mask: u32,
    pub points_program_expiration: u32,
}

impl Deref for RootState {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.discriminator
    }
}

pub const ROOT_ACCOUNT_SIZE: usize = std::mem::size_of::<RootState>();
