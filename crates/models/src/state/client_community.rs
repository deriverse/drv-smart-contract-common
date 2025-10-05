use std::mem::size_of;

use bytemuck::{Pod, Zeroable};

use super::types::Discriminator;
use crate::new_types::client::ClientId;

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
pub struct ClientCommunityRecord {
    pub dividends_rate: f64,
    pub dividends_value: i64,
    pub fees_prepayment: i64,
    pub fees_ratio: f64,   // 1-fees_discount
    pub ref_rewards: i64,  // received
    pub ref_payments: i64, // paid
    pub last_fees_prepayment_time: u32,
    pub crncy_token_id: u32,
}

pub const CLIENT_COMMUNITY_RECORD_SIZE: usize = size_of::<ClientCommunityRecord>();

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ClientCommunityAccountHeader {
    pub discriminator: Discriminator,
    pub id: ClientId,
    pub last_voting_time: u32,
    pub last_voting_counter: u32,
    pub current_voting_counter: u32,
    pub current_voting_tokens: i64,
    pub last_voting_tokens: i64,
    pub last_choice: u32,
    pub slot: u32,
    pub drvs_tokens: i64,
    pub count: u32,
    pub reserved: u32,
}

pub const CLIENT_COMMUNITY_ACCOUNT_HEADER_SIZE: usize = size_of::<ClientCommunityAccountHeader>();
