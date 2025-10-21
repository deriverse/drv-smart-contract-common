use bytemuck::{Pod, Zeroable};

use super::types::Discriminator;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct CommunityAccountHeader {
    pub discriminator: Discriminator,
    pub drvs_tokens: i64,
    pub min_amount: i64,
    pub voting_supply: i64,
    pub prev_voting_supply: i64,
    pub voting_decr: i64,
    pub prev_voting_decr: i64,
    pub voting_unchange: i64,
    pub prev_voting_unchange: i64,
    pub voting_incr: i64,
    pub prev_voting_incr: i64,
    pub voting_counter: u32,
    pub voting_start_slot: u32,
    pub voting_end_time: u32,
    pub spot_fee_rate: u32,
    pub perp_fee_rate: u32,
    pub spot_pool_ratio: u32,
    pub margin_call_penalty_rate: u32,
    pub fees_prepayment_for_max_discount: u32,
    pub max_discount: u32,
    pub reserved_value1: u32,
    pub reserved_value2: u32,
    pub reserved_value3: u32,
    pub reserved_value4: u32,
    pub reserved_value5: u32,
    pub reserved_value6: u32,
    pub reserved_value7: u32,
    pub reserved_value8: u32,
    pub count: u32,
}

pub const COMMUNITY_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<CommunityAccountHeader>();

impl ::std::ops::Deref for CommunityAccountHeader {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.discriminator
    }
}
