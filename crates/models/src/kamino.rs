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
