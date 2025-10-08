use bytemuck::Zeroable;

use crate::{constants::nulls::NULL_ORDER, new_types::client::ClientId, state::types::OrderSide};

use super::spot_account_header::SPOT_TRADE_ACCOUNT_HEADER_SIZE;

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

pub fn get_spot_info<T>(data: &[u8], id: ClientId) -> *mut T {
    data[SPOT_TRADE_ACCOUNT_HEADER_SIZE + size_of::<T>() * *id as usize..].as_ptr() as *mut T
}

impl SpotClientInfo {
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

impl SpotClientInfo2 {
    pub fn set_slot(&mut self, new_slot: u32, side: OrderSide) {
        match side {
            OrderSide::Bid => self.bid_slot = new_slot,
            OrderSide::Ask => self.ask_slot = new_slot,
        };
    }
}
