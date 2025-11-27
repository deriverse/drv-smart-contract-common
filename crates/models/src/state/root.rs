use bytemuck::{Pod, Zeroable};
#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;
#[cfg(feature = "off-chain")]
use solana_sdk::pubkey::Pubkey;

use std::ops::Deref;

use super::types::Discriminator;

/// Root State
///
/// 1. **`operator_address`**
/// 2. **`holder_address`**
/// 3. **`drvs_mint_address`** - LUT address with root related accounts
/// 4. **`airdrop_authority`** - systems airdrop authority
/// 5. **`private_mode_authority_address`** - systems private mode authority
/// 6. **`ref_program_duration`** - Duration of refereal program
/// 7. **`ref_link_duration`** - Duration of each ref link
/// 8. **`ref_discount`** - Discount for ref program
/// 9. **`ref_ratio`** - Ratio for ref program
/// 10. **`clients_count`** - Total clients amount registered on the platform
/// 11. **`tokens_count`** - Total tokens amount created on the platform
/// 12. **`instr_count`** - Total amount of instrument created on the platform
/// 13. **`ref_counter`** - Amount of new ref links created
/// 14. **`mask`**
///     - *PRIVATE_MODE* = 0x1 - Private mode flag
/// 15. **`points_program_expiration`** - Points program expiration time
/// 16. **`purchasing_perp_seat_fee`** - Fee for purchasing an new market seat, 0 by default
///
/// # Notes
/// - Ref stats can be adjust with change_ref_program instruction
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, Debug)]
pub struct RootState {
    pub discriminator: Discriminator,
    pub operator_address: Pubkey,
    pub holder_address: Pubkey,
    pub drvs_mint_address: Pubkey,
    pub lut_address: Pubkey,
    pub airdrop_authority_address: Pubkey,
    pub private_mode_authority_address: Pubkey,
    pub ref_program_duration: u32,
    pub ref_link_duration: u32,
    pub ref_discount: f64,
    pub ref_ratio: f64,
    pub clients_count: u32,
    pub tokens_count: u32,
    pub instr_count: u32,
    pub ref_counter: u32,
    pub mask: u32,
    pub points_program_expiration: u32,
    pub purchasing_perp_seat_fee: f64,
}

impl Deref for RootState {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.discriminator
    }
}

pub const ROOT_ACCOUNT_SIZE: usize = std::mem::size_of::<RootState>();
