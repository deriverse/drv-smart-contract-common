use bytemuck::{Pod, Zeroable};

use super::types::Discriminator;

/// Community Account Header
///
/// 1. **`drvs_tokens`** - Total supply of drvs token via all clients
/// 2. **`min_amount`** - Minimal amount of drvs tokens in clients account
/// 3. **`voting_sypply`** - Total supply for current voting (amount of drvs_tokens during voting finalization)
/// 4. **`prev_supply`** - Total supply for previous voting
/// 5. **`voting_decr`** - Current amount of votes for DECREMENT(choice = 0)
/// 6. **`prev_voting_decr`** - Preveous amount of votes for decrement
/// 7. **`voting_unchage`** - Current amount of votes for UNCHANGE(choice = 1)
/// 8. **`prev_voting_unchage`** - Preveous amount of votes for UNCHANGE
/// 9. **`voting_incr`** - Current amount of votes for INCREMENT(choice = 2)
/// 10. **`prev_incr_unchage`** - Preveous amount of votes for INCREMENT
/// 11. **`voting_counter`** - Current voting_counter. voting_counter mod 6 result in current voting topic
/// 12. **`voting_start_slot`** - Slot of start of current voting
/// 13. **`voting_end_time`** - Ending time of current voting, calculated using `voting_end`
/// 14. **`spot_fee_rate`** - Raw record of spot fee rate. Used in case of zero fees_prepayment and during rabates_fees calculation. FEE_RATE_STEP * spot_fee_rate result it actual value
/// 15. **`perp_fee_rate`** - Raw record of perp fee rate. Used in case of zero fees_prepayment and during rabates_fees calculation. FEE_RATE_STEP * spot_fee_rate result it actual value
/// 16. **`spot_pool_ratio`** - Raw record of spot pool ratio. POOL_RATIO_STEP * spot_pool_ratio result it actual value
/// 17. **`margin_call_penalty_rate`** - Raw record. Represent a fee of trade taken from **Taker** during margin call. MARGIN_CALL_PENALTY_RATE_STEP * margin_call_penalty_rate result in actual value
/// 18. **`fees_prepayment_for_max_discount`** - Raw record. Represent the amount of prepaied fees needed for max discount.
/// 19. **`max_discount`** - Raw record. Represent a maximum discount in fees prepayment program.
/// 20. **`count`** - Length of `BaseCrncyRecord` array

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
