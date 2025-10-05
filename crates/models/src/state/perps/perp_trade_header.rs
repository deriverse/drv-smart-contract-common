use bytemuck::{Pod, Zeroable};
use drv_macros::pod_wrapper;

use std::mem::size_of;

use crate::{new_types::instrument::InstrId, state::types::Discriminator};

#[pod_wrapper]
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct PerpTradeAccountHeader<const TAG: u32> {
    pub discriminator: Discriminator,
    pub id: InstrId,
    pub slot: u32,
    pub asset_token_id: u32,
    pub crncy_token_id: u32,
}

pub const PERP_TRADE_ACCOUNT_HEADER_SIZE: usize = size_of::<PerpTradeAccountHeader<0>>();
