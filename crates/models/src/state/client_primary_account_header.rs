use bytemuck::{Pod, Zeroable};

use crate::new_types::client::ClientId;

#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;

use super::types::Discriminator;
#[cfg(feature = "off-chain")]
use solana_sdk::pubkey::Pubkey;

/// Client Primary Account Header
///
/// 1. **`lut_address`**
/// 2. **`ref_address`** - Current address of referee
/// 3. **`vm_wallet_address`** - Reserved for vault mode
/// 4. **`vm_instr[8]`** - Reserved for vault mode
/// 5. **`vm_withdraw_token_id`** - Reserved for vault mode
/// 6. **`vm_wallet_address`** - Reserved for vault mode
/// 7. **`first_ref_link_discount`** - Discount of first referer link
/// 8. **`second_ref_link_discount`** - Discount of second referer link
/// 9. **`first_ref_link_ratio`** - Ratio of first referral link
/// 10. **`second_ref_link_ratio`** - Ration of second referral link
/// 11. **`ref_program_discount`** - Discount of currently applied referral link
/// 12. **`ref_program_ratio`** - Ratio of currently applied referral link
/// 13. **`mask`**
///     - Progress trakcer which grant points for hitting trading milestones on the pltaform etc. spot_trades, lp_trades, perp_trades
///     - 0x70000000000000 - New liquidation event
///     - 0x100000000000000 - Airdrop available
/// 14. **`id`** - Original client id
/// 15. **`ref_client_id`** - Origincal client id of current referee
/// 16. **`ref_counter`** - Amount of referees
/// 17. **`first_ref_link_id`** - First link id
/// 18. **`second_ref_link_id`** - Second link id
/// 19. **`first_ref_link_expiration`** - Expiration of first ref link in unix_timestamp
/// 20. **`second_ref_link_expiration`** - Expiration of second ref link in unix_timestamp
/// 21. **`ref_program_expiration`** - Expiration of referral program in unixt_timestamp
/// 22. **`spot_trades`** - Amount of trades made on spot as a taker
/// 23. **`perp_trades`** - Amount of trades made on perp as a taker
/// 24. **`lp_trades`** - Amount of trades made in lp
/// 25. **`points`** - Amount of points for Points Program
/// 26. **`slot`** - Record last **writable** manipulation with ClientPrimaryAccountHeader
/// 27. **`assets_count`** - Length of assets record array
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ClientPrimaryAccountHeader {
    pub discriminator: Discriminator,
    pub wallet_address: Pubkey,
    pub lut_address: Pubkey,
    pub ref_address: Pubkey,
    pub vm_wallet_address: Pubkey,
    pub vm_instrs: [u32; 8],
    pub vm_withdraw_token_id: u32,
    pub vm_mask: u32,
    pub vm_withdraw_amount: i64,
    pub first_ref_link_discount: f64,
    pub second_ref_link_discount: f64,
    pub first_ref_link_ratio: f64,
    pub second_ref_link_ratio: f64,
    pub ref_program_discount: f64,
    pub ref_program_ratio: f64,
    pub reserved: i64,
    pub mask: i64,
    pub id: ClientId,
    pub ref_client_id: ClientId,
    pub ref_counter: u32,
    pub first_ref_link_id: u32,
    pub second_ref_link_id: u32,
    pub first_ref_link_expiration: u32,
    pub second_ref_link_expiration: u32,
    pub ref_program_expiration: u32,
    pub spot_trades: u32,
    pub perp_trades: u32,
    pub lp_trades: u32,
    pub points: u32,
    pub slot: u32,
    pub assets_count: u32,
    pub spot_filled_orders: u32,
    pub perp_filled_orders: u32,
    pub reserved_value1: i64,
    pub reserved_value2: i64,
    pub reserved_value3: i64,
    pub reserved_value4: i64,
    pub reserved_value5: i64,
    pub reserved_value6: i64,
    pub reserved_value7: i64,
    pub reserved_value8: i64,
}

pub const CLIENT_PRIMARY_ACCOUNT_HEADER_SIZE: usize =
    std::mem::size_of::<ClientPrimaryAccountHeader>();

impl ::std::ops::Deref for ClientPrimaryAccountHeader {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.discriminator
    }
}
