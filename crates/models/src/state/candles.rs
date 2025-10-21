use super::types::Discriminator;
use crate::new_types::instrument::InstrId;
use bytemuck::{Pod, Zeroable};
use drv_macros::pod_wrapper;

use std::mem::size_of;

/// Represents a single price candle in a circular buffer of candles (`CandleBuffer`).
///
/// A candle aggregates trading data over a fixed duration and contains:
///
/// 1. **`open`** - The opening price at the start of the candle. Must be within allowed price limits.
/// 2. **`close`** - The closing price at the end of the candle. Must be within allowed price limits.
/// 3. **`max`** - The maximum price reached during the candle's duration. Must respect price limits.
/// 4. **`min`** - The minimum price reached during the candle's duration. Must respect price limits.
/// 5. **`asset_tokens`** - The total volume of asset tokens traded during this candle.
/// 6. **`crncy_tokens`** - The total volume of currency tokens traded during this candle.
/// 7. **`time`** - The start time of the candle (aligned to the candle duration, i.e., `time % CANDLE.duration == 0`).
/// 8. **`counter`** - Trade counter.
///
/// # Notes
/// - Candles are stored in a circular buffer (`CandleBuffer`).
/// - Prices are represented as integers (`i64`).
/// - The `time` field is always aligned to the candle duration.
/// - Allowed price limits `MIN_PRICE..MAX_PRICE`
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
///
/// 4. **`count`** - amount of allocated candles in the buffer. count < CANDLE.capacity
/// 5. **`last`** - index of last written candle. last < CANDLE.capacity
pub struct CandlesAccountHeader<const TAG: u32> {
    pub discriminator: Discriminator,
    pub id: InstrId,
    pub slot: u32,
    pub count: u32,
    pub last: u32,
}

pub const CANDLES_ACCOUNT_HEADER_SIZE: usize = size_of::<CandlesAccountHeader<0>>();
