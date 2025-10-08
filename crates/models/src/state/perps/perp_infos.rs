use crate::new_types::client::ClientId;
use bytemuck::Zeroable;

use super::perp_trade_header::PERP_TRADE_ACCOUNT_HEADER_SIZE;

pub fn get_perp_info<T>(data: &[u8], id: ClientId) -> *mut T {
    data[PERP_TRADE_ACCOUNT_HEADER_SIZE + size_of::<T>() * *id as usize..].as_ptr() as *mut T
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
pub struct PerpClientInfo {
    pub funds: i64,
    pub perps: i64,
    pub in_orders_funds: i64,
    pub in_orders_perps: i64,
}

pub const PERP_CLIENT_INFO_SIZE: usize = std::mem::size_of::<PerpClientInfo>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
pub struct PerpClientInfo2 {
    pub cost: i64,
    pub result: i64,
    pub bid_slot: u32,
    pub ask_slot: u32,
    pub px_node: u32,
    pub mask: u32,
}

pub const PERP_CLIENT_INFO2_SIZE: usize = std::mem::size_of::<PerpClientInfo2>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
pub struct PerpClientInfo3 {
    pub client: ClientId,
    pub priority_node: u32,
    pub bids_entry: u32,
    pub asks_entry: u32,
    pub fees: i64,
    pub rebates: i64,
}

pub const PERP_CLIENT_INFO3_SIZE: usize = std::mem::size_of::<PerpClientInfo3>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
pub struct PerpClientInfo4 {
    pub last_soc_loss_rate: f64,
    pub last_soc_loss_perps: i64,
    pub soc_loss_funds: i64,
    pub loss_coverage: i64,
}

pub const PERP_CLIENT_INFO4_SIZE: usize = std::mem::size_of::<PerpClientInfo4>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
pub struct PerpClientInfo5 {
    pub funding_funds: i64,
    pub last_funding_rate: f64,
    pub reserved: i64,
    pub rebalance_time: u32,
    pub funding_node: u32,
}

pub const PERP_CLIENT_INFO5_SIZE: usize = std::mem::size_of::<PerpClientInfo5>();
