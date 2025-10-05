use crate::new_types::instrument::InstrId;
use bytemuck::{Pod, Zeroable};
use drv_macros::pod_wrapper;

use crate::state::types::Discriminator;

#[pod_wrapper]
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct SpotTradeAccountHeader<const TAG: u32> {
    pub discriminator: Discriminator,
    pub instr_id: InstrId,
    pub slot: u32,
    pub asset_token_id: u32,
    pub crncy_token_id: u32,
}

pub const SPOT_TRADE_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<SpotTradeAccountHeader<0>>();
