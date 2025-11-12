use crate::state::types::Discriminator;
use bytemuck::{Pod, Zeroable};

#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, Debug)]
pub struct PrivateClientHeader {
    pub discriminator: Discriminator,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, Debug)]
pub struct PrivateClient {
    pub creation_time: u32,
    pub expiration_time: u32,
    pub wallet: Pubkey,
}

impl PrivateClient {
    pub fn is_vacant(&self, current_time: u32) -> bool {
        self.creation_time == 0 || current_time > self.expiration_time
    }
}
