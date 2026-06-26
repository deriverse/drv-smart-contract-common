use crate::state::{
    client_community::ClientCommunityAccountHeader,
    client_primary_account_header::ClientPrimaryAccountHeader,
    community_account_header::CommunityAccountHeader,
    holder::HolderAccountHeader,
    instrument::InstrAccountHeader,
    perps::perp_trade_header::PerpTradeAccountHeader,
    private_mode::PrivateClientHeader,
    root::RootState,
    spots::spot_account_header::SpotTradeAccountHeader,
    types::{account_type, ClientVmAccountHeader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seed {
    ConstAscii(&'static str),
    ConstU32(u32),
    Param(&'static str),
    ParamU32(&'static str),
}

/// Canonical PDA derivation recipe for an account type.
#[derive(Debug, Clone, Copy)]
pub struct PdaRef {
    pub seeds: &'static [Seed],
}

impl PdaRef {
    pub const fn new(seeds: &'static [Seed]) -> Self {
        Self { seeds }
    }
}

/// Associates a Deriverse account type with its on-chain tag and canonical PDA derivation.
pub trait DrvAccount {
    const TAG: u32;
    const PDA: PdaRef;
}

// ── Non-instrument accounts ──────────────────────────────────────────────────

impl DrvAccount for HolderAccountHeader {
    const TAG: u32 = account_type::HOLDER;
    const PDA: PdaRef = PdaRef::new(&[Seed::ConstAscii("drvs001"), Seed::Param("holderAdmin")]);
}

impl DrvAccount for RootState {
    const TAG: u32 = account_type::ROOT;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::ROOT),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for CommunityAccountHeader {
    const TAG: u32 = account_type::COMMUNITY;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::COMMUNITY),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PrivateClientHeader {
    const TAG: u32 = account_type::PRIVATE_CLIENTS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PRIVATE_CLIENTS),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for ClientPrimaryAccountHeader {
    const TAG: u32 = account_type::CLIENT_PRIMARY;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::CLIENT_PRIMARY),
        Seed::Param("wallet"),
    ]);
}

impl DrvAccount for ClientCommunityAccountHeader {
    const TAG: u32 = account_type::CLIENT_COMMUNITY;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::CLIENT_COMMUNITY),
        Seed::Param("wallet"),
    ]);
}

impl DrvAccount for ClientVmAccountHeader {
    const TAG: u32 = account_type::VM_CLIENT;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::VM_CLIENT),
        Seed::Param("wallet"),
    ]);
}

// ── Instrument account ───────────────────────────────────────────────────────

impl DrvAccount for InstrAccountHeader {
    const TAG: u32 = account_type::INSTR;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::INSTR),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

// ── Spot accounts ────────────────────────────────────────────────────────────

impl DrvAccount for SpotTradeAccountHeader<{ account_type::SPOT_CLIENT_INFOS }> {
    const TAG: u32 = account_type::SPOT_CLIENT_INFOS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::SPOT_CLIENT_INFOS),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for SpotTradeAccountHeader<{ account_type::SPOT_BIDS_TREE }> {
    const TAG: u32 = account_type::SPOT_BIDS_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::SPOT_BIDS_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for SpotTradeAccountHeader<{ account_type::SPOT_ASKS_TREE }> {
    const TAG: u32 = account_type::SPOT_ASKS_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::SPOT_ASKS_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for SpotTradeAccountHeader<{ account_type::SPOT_BID_ORDERS }> {
    const TAG: u32 = account_type::SPOT_BID_ORDERS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::SPOT_BID_ORDERS),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for SpotTradeAccountHeader<{ account_type::SPOT_ASK_ORDERS }> {
    const TAG: u32 = account_type::SPOT_ASK_ORDERS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::SPOT_ASK_ORDERS),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for SpotTradeAccountHeader<{ account_type::SPOT_LINES }> {
    const TAG: u32 = account_type::SPOT_LINES;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::SPOT_LINES),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

// ── Perp accounts ────────────────────────────────────────────────────────────

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_ASK_ORDERS }> {
    const TAG: u32 = account_type::PERP_ASK_ORDERS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_ASK_ORDERS),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_ASKS_TREE }> {
    const TAG: u32 = account_type::PERP_ASKS_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_ASKS_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_BID_ORDERS }> {
    const TAG: u32 = account_type::PERP_BID_ORDERS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_BID_ORDERS),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_BIDS_TREE }> {
    const TAG: u32 = account_type::PERP_BIDS_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_BIDS_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_CLIENT_INFOS }> {
    const TAG: u32 = account_type::PERP_CLIENT_INFOS;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_CLIENT_INFOS),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_CLIENT_INFOS2 }> {
    const TAG: u32 = account_type::PERP_CLIENT_INFOS2;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_CLIENT_INFOS2),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_CLIENT_INFOS3 }> {
    const TAG: u32 = account_type::PERP_CLIENT_INFOS3;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_CLIENT_INFOS3),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_CLIENT_INFOS4 }> {
    const TAG: u32 = account_type::PERP_CLIENT_INFOS4;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_CLIENT_INFOS4),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_CLIENT_INFOS5 }> {
    const TAG: u32 = account_type::PERP_CLIENT_INFOS5;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_CLIENT_INFOS5),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_LINES }> {
    const TAG: u32 = account_type::PERP_LINES;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_LINES),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_LONG_PX_TREE }> {
    const TAG: u32 = account_type::PERP_LONG_PX_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_LONG_PX_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_SHORT_PX_TREE }> {
    const TAG: u32 = account_type::PERP_SHORT_PX_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_SHORT_PX_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}

impl DrvAccount for PerpTradeAccountHeader<{ account_type::PERP_REBALANCE_TIME_TREE }> {
    const TAG: u32 = account_type::PERP_REBALANCE_TIME_TREE;
    const PDA: PdaRef = PdaRef::new(&[
        Seed::ParamU32("version"),
        Seed::ConstU32(account_type::PERP_REBALANCE_TIME_TREE),
        Seed::ParamU32("assetTokenId"),
        Seed::ParamU32("crncyTokenId"),
        Seed::Param("deriverseAuthority"),
    ]);
}
