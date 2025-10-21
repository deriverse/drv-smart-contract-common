use std::mem::size_of;

use bytemuck::{Pod, Zeroable};

use super::types::Discriminator;
use crate::new_types::client::ClientId;

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// Clients community information records for a specific currency.
///
/// 1. **`dividends_rate`** - Dividents rate per 1 DRVS token
/// 2. **`dividends_value`** - Amount of dividents ready to claim
/// 3. **`fees_prepayment`** - Fees prepayment for a discount. fees_prepayment >= 0
/// 4. **`fees_ratio`**
/// 5. **`ref_rewards`** - Total rewards received from refereal program. ref_rewards >= 0
/// 6. **`ref_payments`** - Total rewards paid to provided refereal. ref_payments <= 0
/// 7. **`last_fees_prepayment_time`**
/// 8. **`crncy_token_id`**
///
/// # Notes
/// - `ClientCommunityRecord` is stored in a Vec. For each different **`crncy_token_id`**

pub struct ClientCommunityRecord {
    pub dividends_rate: f64,
    pub dividends_value: i64,
    pub fees_prepayment: i64,
    pub fees_ratio: f64,
    pub ref_rewards: i64,
    pub ref_payments: i64,
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
