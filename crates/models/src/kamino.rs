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
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ObligationPrefix {
    pub discriminator: [u8; 8],
    pub tag: u64,
    pub last_update: [u8; 16],
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [ObligationCollateral; 8],
    pub lowest_reserve_deposit_liquidation_ltv: u64,
    pub deposited_value_sf: [u8; 16],
    pub borrows: [ObligationLiquidity; 5],
}
