///! # Spot client infos module
///! - Store information about clients state on spot
///! - Each info record is stored in an array in different accounts at temp_client_id index and managed by memory map from maps_acc
use bytemuck::Zeroable;
use std::mem::size_of;

use crate::new_types::client::ClientId;

use super::spot_account_header::SPOT_TRADE_ACCOUNT_HEADER_SIZE;

pub fn get_spot_info<T>(data: &[u8], id: ClientId) -> *mut T {
    data[SPOT_TRADE_ACCOUNT_HEADER_SIZE + size_of::<T>() * *id as usize..].as_ptr() as *mut T
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// Spot Client Info
///
/// 1. **`client`** - Original client id
/// 2. **`filled_orders`** - Fully filled orders count
/// 2. **`bids_entry`** - Stores clients bids orders linked list head in last 4 bits and linked list length in first 4 bits
/// 3. **`ask_entry`** - Stores clients asks orders linked list head in last 4 bits and linked list length in first 4 bits
/// 4. **`avail_asset_tokens`** - Total avail asset tokens to withdraw
/// 5. **`avail_crncy_tokens`** - Total avail crncy tokens to withdraw
pub struct SpotClientInfo {
    pub client: ClientId,
    pub filled_orders: u32,
    pub bids_entry: u32,
    pub asks_entry: u32,
    pub avail_asset_tokens: i64,
    pub avail_crncy_tokens: i64,
}

pub const SPOT_CLIENT_INFO_SIZE: usize = size_of::<SpotClientInfo>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]

/// Spo Client Info 2
///
/// 1. **`in_orders_asset_tokens`** - Total amount of assets tokens locked in orders
/// 2. **`in_orders_crncy_tokens`** - Total amount of crncy tokens locked in orders
/// 3. **`bid_slot`** - Slot of last update on bid side
/// 4. **`ask_slot`** - Slot of last update on asl=k side
pub struct SpotClientInfo2 {
    pub in_orders_asset_tokens: i64,
    pub in_orders_crncy_tokens: i64,
    pub bid_slot: u32,
    pub ask_slot: u32,
    pub reserved: i64,
}

pub const SPOT_CLIENT_INFO2_SIZE: usize = size_of::<SpotClientInfo2>();
