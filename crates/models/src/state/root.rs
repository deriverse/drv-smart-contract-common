use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

use std::ops::Deref;

use super::types::Discriminator;

/// Root State
///
/// 1. **`operator_address`**
/// 2. **`holder_address`**
/// 3. **`drvs_mint_address`** - LUT address with root related accounts
/// 4. **`ref_program_duration`** - Duration of refereal program
/// 5. **`ref_link_duration`** - Duration of each ref link
/// 6. **`ref_discount`** - Discount for ref program
/// 7. **`ref_ratio`** - Ratio for ref program
/// 8. **`clients_count`** - Total clients amount registered on the platform
/// 9. **`tokens_count`** - Total tokens amount created on the platform
/// 10. **`instr_count`** - Total amount of instrument created on the platform
/// 11. **`ref_counter`** - Amount of new ref links created
/// 12. **`mask`**
///     - *PRIVATE_MODE* = 0x1 - Private mode flag
/// 13. **`points_program_expiration`** - Points program expiration time
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
}

impl Deref for RootState {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.discriminator
    }
}

pub const ROOT_ACCOUNT_SIZE: usize = std::mem::size_of::<RootState>();
