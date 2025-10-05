use bytemuck::{Pod, Zeroable};

use crate::new_types::client::ClientId;

#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;

// #[cfg(not(feature = "on-chain"))]
// compile_error!("TODO")

use super::types::Discriminator;
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ClientPrimaryAccountHeader {
    pub discriminator: Discriminator,
    pub wallet_address: Pubkey,
    pub drv_address: Pubkey,
    pub community_address: Pubkey,
    pub lut_address: Pubkey,
    pub ref_address: Pubkey,
    pub first_ref_link_discount: f64,
    pub second_ref_link_discount: f64,
    pub first_ref_link_ratio: f64,
    pub second_ref_link_ratio: f64,
    pub ref_program_discount: f64,
    pub ref_program_ratio: f64,
    pub reserved: i64,
    pub mask: i64,
    pub id: ClientId,
    pub ref_client_id: ClientId,
    pub ref_counter: u32,
    pub first_ref_link_id: u32,
    pub second_ref_link_id: u32,
    pub first_ref_link_expiration: u32,
    pub second_ref_link_expiration: u32,
    pub ref_program_expiration: u32,
    pub spot_trades: u32,
    pub perp_trades: u32,
    pub lp_trades: u32,
    pub points: u32,
    pub slot: u32,
    pub assets_count: u32,
    pub reserved_value1: i64,
    pub reserved_value2: i64,
    pub reserved_value3: i64,
    pub reserved_value4: i64,
    pub reserved_value5: i64,
    pub reserved_value6: i64,
    pub reserved_value7: i64,
    pub reserved_value8: i64,
}

pub const CLIENT_PRIMARY_ACCOUNT_HEADER_SIZE: usize =
    std::mem::size_of::<ClientPrimaryAccountHeader>();
