///! # Perp client infos module
///! - Store information about clients state on perp
///! - Each info record is stored in an array in different accounts at temp_client_id index and managed by memory map from maps_acc
use crate::{new_types::client::ClientId, state::types::OrderSide};
use bytemuck::Zeroable;

use std::mem::size_of;

use super::perp_trade_header::PERP_TRADE_ACCOUNT_HEADER_SIZE;

pub fn get_perp_info<T>(data: &[u8], id: ClientId) -> *mut T {
    data[PERP_TRADE_ACCOUNT_HEADER_SIZE + size_of::<T>() * *id as usize..].as_ptr() as *mut T
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
/// Perp Client Info
///
/// 1. **`funds`** - Client available funds in base crncy
/// 2. **`perps`** - The number of futues in clients position
/// 3. **`in_order_funds`** - Clients funds locked in buy orders
/// 4. **`in_orders_perps`** - Clients funds locked in sell orders
pub struct PerpClientInfo {
    pub funds: i64,
    pub perps: i64,
    pub in_orders_funds: i64,
    pub in_orders_perps: i64,
}

pub const PERP_CLIENT_INFO_SIZE: usize = size_of::<PerpClientInfo>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
/// Perp Client Info 2
///
/// 1. **`cost`** - Amount of funds in base crncy spent or received for the open position
/// 2. **`result`** - Profit or loss already made from closed trades
/// 3. **`bid_slot`** - Slot of last update on bid side
/// 4. **`ask_slot`** - Slot of last update on ask side
/// 5. **`px_node`** - Link on edge price node in long/short tree
/// 6. **`mask`**
///     - MSB (most significant bit) - Position side, long(0) or short (1)
///     - 0xFF - Current clients leverage
///     - 0x40000000
pub struct PerpClientInfo2 {
    pub cost: i64,
    pub result: i64,
    pub bid_slot: u32,
    pub ask_slot: u32,
    pub px_node: u32,
    pub mask: u32,
}

pub const PERP_CLIENT_INFO2_SIZE: usize = size_of::<PerpClientInfo2>();

impl PerpClientInfo2 {
    pub fn set_slot(&mut self, new_slot: u32, side: OrderSide) {
        match side {
            OrderSide::Bid => self.bid_slot = new_slot,
            OrderSide::Ask => self.ask_slot = new_slot,
        };
    }
    pub fn leverage(&self) -> i64 {
        (self.mask & 0xFF) as i64
    }
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
/// Perp Client Info 3
///
/// 1. **`client`** - Original client id
/// 2. **`filled_orders`** - Fully filled orders count
/// 3. **`bids_entry`** - Stores clients bids orders linked list head in last 4 bits and linked list length in first 4 bits
/// 4. **`ask_entry`** - Stores clients asks orders linked list head in last 4 bits and linked list length in first 4 bits
/// 5. **`fee`** - Statistic of all amount of fees paid to the protocol
/// 6. **`rebates`** - Statisit of rebates received from protocol
pub struct PerpClientInfo3 {
    pub client: ClientId,
    pub filled_orders: u32,
    pub bids_entry: u32,
    pub asks_entry: u32,
    pub fees: i64,
    pub rebates: i64,
}

pub const PERP_CLIENT_INFO3_SIZE: usize = size_of::<PerpClientInfo3>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
/// Perp Client Info 4
///
/// 1. **`last_soc_loss_rate`** - Last social loss rate according to clients position
/// 2. **`last_soc_loss_perps`** - Total amount of perps during last soc lost update
/// 3. **`soc_loss_funds`** - Amount of funds compensated by soc loss compensation procedure
/// 4. **`loss_coverage`** - Amount of funds compensated in total
pub struct PerpClientInfo4 {
    pub last_soc_loss_rate: f64,
    pub last_soc_loss_perps: i64,
    pub soc_loss_funds: i64,
    pub loss_coverage: i64,
}

pub const PERP_CLIENT_INFO4_SIZE: usize = size_of::<PerpClientInfo4>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Debug)]
/// Perp Client Info 5
///
/// 1. **`funding_funds`** - Statistic over received funding funds from protocol
/// 2. **`last_funding_rate`** - Funding rate during last clients funding rate update
/// 3. **`rebalance_time`** - Last rebalance time record. rebalance_time and temp_client_id form key in rebalance RBTree
/// 4. **`funding_node`** - Node in rebalance_time RBTree
pub struct PerpClientInfo5 {
    pub funding_funds: i64,
    pub last_funding_rate: f64,
    pub reserved: i64,
    pub rebalance_time: u32,
    pub funding_node: u32,
}

pub const PERP_CLIENT_INFO5_SIZE: usize = size_of::<PerpClientInfo5>();
