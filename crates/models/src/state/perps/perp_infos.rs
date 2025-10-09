use crate::{constants::NULL_ORDER, new_types::client::ClientId, state::types::OrderSide};
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

impl PerpClientInfo {
    pub fn funds(&self) -> i64 {
        self.funds
    }
    pub fn add_funds(&mut self, amount: i64) {
        self.funds += amount;
    }
    pub fn sub_funds(&mut self, amount: i64) {
        self.funds -= amount;
    }
    pub fn add_perps(&mut self, amount: i64) {
        self.perps += amount;
    }
    pub fn sub_perps(&mut self, amount: i64) {
        self.perps -= amount;
    }
    pub fn total_perps(&self) -> i64 {
        self.perps + self.in_orders_perps
    }
    pub fn total_funds(&self) -> i64 {
        self.funds + self.in_orders_funds
    }
}

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

impl PerpClientInfo3 {
    pub fn orders_entry(&self, side: OrderSide) -> u32 {
        match side {
            OrderSide::Bid => self.bids_entry & 0xFFFF,
            OrderSide::Ask => self.asks_entry & 0xFFFF,
        }
    }
    pub fn orders_count(&self, side: OrderSide) -> u32 {
        match side {
            OrderSide::Bid => self.bids_entry >> 16,
            OrderSide::Ask => self.asks_entry >> 16,
        }
    }
    pub fn orders_reset(&mut self, side: OrderSide) {
        match side {
            OrderSide::Bid => self.bids_entry = NULL_ORDER,
            OrderSide::Ask => self.asks_entry = NULL_ORDER,
        };
    }
    pub fn is_active(&self) -> bool {
        self.bids_entry != NULL_ORDER || self.asks_entry != NULL_ORDER
    }
    pub fn set_orders_entry_with_count_incr(&mut self, new_orders_entry: u32, side: OrderSide) {
        match side {
            OrderSide::Bid => {
                self.bids_entry = (self.bids_entry & 0xFFFF0000) + 0x10000 + new_orders_entry
            }
            OrderSide::Ask => {
                self.asks_entry = (self.asks_entry & 0xFFFF0000) + 0x10000 + new_orders_entry
            }
        };
    }
    pub fn set_orders_entry_with_count_decr(&mut self, new_orders_entry: u32, side: OrderSide) {
        match side {
            OrderSide::Bid => {
                self.bids_entry = (self.bids_entry & 0xFFFF0000) - 0x10000 + new_orders_entry
            }
            OrderSide::Ask => {
                self.asks_entry = (self.asks_entry & 0xFFFF0000) - 0x10000 + new_orders_entry
            }
        };
    }
    pub fn orders_count_incr(&mut self, side: OrderSide) {
        match side {
            OrderSide::Bid => self.bids_entry += 0x10000,
            OrderSide::Ask => self.asks_entry += 0x10000,
        };
    }
    pub fn orders_count_decr(&mut self, side: OrderSide) {
        match side {
            OrderSide::Bid => self.bids_entry -= 0x10000,
            OrderSide::Ask => self.asks_entry -= 0x10000,
        };
    }
}
