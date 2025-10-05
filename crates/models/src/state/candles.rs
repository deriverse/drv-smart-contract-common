use super::types::Discriminator;
use crate::new_types::instrument::InstrId;
use bytemuck::{Pod, Zeroable};
use drv_macros::pod_wrapper;

use std::mem::size_of;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Candle {
    pub open: i64,
    pub close: i64,
    pub max: i64,
    pub min: i64,
    pub asset_tokens: i64,
    pub crncy_tokens: i64,
    pub time: u32,
    pub counter: u32,
}

pub const CANDLE_SIZE: usize = size_of::<Candle>();

#[pod_wrapper]
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
/// Describe the state of candles buffer based on candles tag and instrument.
pub struct CandlesAccountHeader<const TAG: u32> {
    pub discriminator: Discriminator,
    pub id: InstrId,
    pub slot: u32,
    pub count: u32,
    pub last: u32,
}

pub const CANDLES_ACCOUNT_HEADER_SIZE: usize = size_of::<CandlesAccountHeader<0>>();
