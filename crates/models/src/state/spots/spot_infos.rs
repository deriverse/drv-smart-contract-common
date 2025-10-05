use bytemuck::Zeroable;

use crate::new_types::client::ClientId;

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
pub struct SpotClientInfo {
    pub client: ClientId,
    pub reserved: u32,
    pub bids_entry: u32,
    pub asks_entry: u32,
    pub avail_asset_tokens: i64,
    pub avail_crncy_tokens: i64,
}

pub const SPOT_CLIENT_INFO_SIZE: usize = std::mem::size_of::<SpotClientInfo>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
pub struct SpotClientInfo2 {
    pub in_orders_asset_tokens: i64,
    pub in_orders_crncy_tokens: i64,
    pub bid_slot: u32,
    pub ask_slot: u32,
    pub reserved: i64,
}

pub const SPOT_CLIENT_INFO2_SIZE: usize = std::mem::size_of::<SpotClientInfo2>();
