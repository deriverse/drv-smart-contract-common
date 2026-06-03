use bytemuck::{Pod, Zeroable};
use solana_pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitUserMetadataArgs {
    pub disc: [u8; 8],
    pub user_lookup_table: Pubkey,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitObligationArgs {
    pub disc: [u8; 8],
    pub tag: u8,
    pub id: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RefreshArgs {
    pub disc: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct U64ArgsIx {
    pub disc: [u8; 8],
    pub liquidity_amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitObligationFarmsForReserveArgs {
    pub disc: [u8; 8],
    pub mode: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ObligationCollateral {
    pub deposit_reserve: Pubkey,
    pub deposited_amount: u64,
    pub market_value_sf: [u8; 16],
    pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
    pub padding: [u64; 9],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FixedTermBorrowRolloverConfig {
    pub auto_rollover_enabled: u8,
    pub open_term_allowed: u8,
    pub migration_to_fixed_enabled: u8,
    pub alignment_padding: [u8; 1],
    pub max_borrow_rate_bps: u32,
    pub min_debt_term_seconds: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ObligationLiquidity {
    pub borrow_reserve: Pubkey,
    pub cumulative_borrow_rate_bsf: [u64; 6],
    pub last_borrowed_at_timestamp: u64,
    pub borrowed_amount_sf: [u8; 16],
    pub market_value_sf: [u8; 16],
    pub borrow_factor_adjusted_market_value_sf: [u8; 16],
    pub borrowed_amount_outside_elevation_groups: u64,
    pub fixed_term_borrow_rollover_config: FixedTermBorrowRolloverConfig,
    pub borrowed_amount_at_expiration: u64,
    pub padding2: [u64; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct ReserveConfig {
    pub status: u8,

    pub padding_deprecated_asset_tier: u8,

    pub host_fixed_interest_rate_bps: u16,

    pub min_deleveraging_bonus_bps: u16,

    pub block_ctoken_usage: u8,

    pub early_repay_remaining_interest_pct: u8,

    pub emergency_mode: u8,

    pub reserved_1: [u8; 4],

    pub protocol_order_execution_fee_pct: u8,

    pub protocol_take_rate_pct: u8,

    pub protocol_liquidation_fee_pct: u8,

    pub loan_to_value_pct: u8,

    pub liquidation_threshold_pct: u8,

    pub min_liquidation_bonus_bps: u16,

    pub max_liquidation_bonus_bps: u16,

    pub bad_debt_liquidation_bonus_bps: u16,

    pub deleveraging_margin_call_period_secs: u64,

    pub deleveraging_threshold_decrease_bps_per_day: u64,

    pub fees: [u64; 3],

    pub borrow_rate_curve: [u64; 11],

    pub borrow_factor_pct: u64,

    pub deposit_limit: u64,

    pub borrow_limit: u64,

    pub token_info: [u64; 48],

    pub deposit_withdrawal_cap: [u8; 32],

    pub debt_withdrawal_cap: [u8; 32],

    pub elevation_groups: [u8; 20],
    pub disable_usage_as_coll_outside_emode: u8,

    pub utilization_limit_block_borrowing_above_pct: u8,

    pub autodeleverage_enabled: u8,

    pub proposer_authority_locked: u8,

    pub borrow_limit_outside_elevation_group: u64,

    pub borrow_limit_against_this_collateral_in_elevation_group: [u64; 32],

    pub deleveraging_bonus_increase_bps_per_day: u64,

    pub debt_maturity_timestamp: u64,

    pub debt_term_seconds: u64,

    pub rewards_amount_per_slot: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct ObligationPrefix {
    pub discriminator: [u8; 8],
    pub tag: u64,
    pub last_update: [u8; 16],
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [ObligationCollateral; 8],
    pub lowest_reserve_deposit_liquidation_ltv: u64,
    pub deposited_value_sf: [u8; 16], // was u128
    pub borrows: [ObligationLiquidity; 5],
    pub borrow_factor_adjusted_debt_value_sf: [u8; 16], // was u128
    pub borrowed_assets_market_value_sf: [u8; 16],      // was u128
    pub allowed_borrow_value_sf: [u8; 16],              // was u128
    pub unhealthy_borrow_value_sf: [u8; 16],            // was u128
    pub padding_deprecated_asset_tiers: [u8; 13],
    pub elevation_group: u8,
    pub num_of_obsolete_deposit_reserves: u8,
    pub has_debt: u8,
    pub referrer: Pubkey,
    pub borrowing_disabled: u8,
    pub autodeleverage_target_ltv_pct: u8,
    pub lowest_reserve_deposit_max_ltv_pct: u8,
    pub num_of_obsolete_borrow_reserves: u8,
    pub ownership_transfer_state: u8,
    pub reserved: [u8; 3],
    pub highest_borrow_factor_pct: u64,
    pub autodeleverage_margin_call_started_timestamp: u64,
}

impl ObligationPrefix {
    #[inline]
    pub fn deposited_value_sf(&self) -> u128 {
        u128::from_le_bytes(self.deposited_value_sf)
    }

    #[inline]
    pub fn borrow_factor_adjusted_debt_value_sf(&self) -> u128 {
        u128::from_le_bytes(self.borrow_factor_adjusted_debt_value_sf)
    }

    #[inline]
    pub fn borrowed_assets_market_value_sf(&self) -> u128 {
        u128::from_le_bytes(self.borrowed_assets_market_value_sf)
    }

    #[inline]
    pub fn allowed_borrow_value_sf(&self) -> u128 {
        u128::from_le_bytes(self.allowed_borrow_value_sf)
    }

    #[inline]
    pub fn unhealthy_borrow_value_sf(&self) -> u128 {
        u128::from_le_bytes(self.unhealthy_borrow_value_sf)
    }
}
