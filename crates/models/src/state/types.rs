use std::ops::Deref;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::new_types::{client::ClientId, instrument::InstrId, tag::Tag, version::Version};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum OrderSide {
    Bid,
    Ask,
}

impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrderSide::Bid => "Bid",
                OrderSide::Ask => "Ask",
            }
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
    MarginCall = 2,
    FrocedClose = 3,
}

pub mod root_mask {
    pub const PRIVATE_MODE: u32 = 0x1;
}

pub mod instr_mask {
    pub const DRV: u32 = 0x10000000;
    pub const READY_TO_DRV_UPGRADE: u32 = 0x20000000;
    pub const PERP: u32 = 0x40000000;
    pub const ORACLE: u32 = 0x80000000;
    pub const READY_TO_PERP_UPGRADE: u32 = 0x1000000;
}

pub mod account_type {

    use super::*;

    pub const CLIENT_COMMUNITY: u32 = 35;
    pub const CLIENT_DRV: u32 = 32;
    pub const CLIENT_PRIMARY: u32 = 31;
    pub const COMMUNITY: u32 = 34;

    pub const HOLDER: u32 = 1;
    pub const ROOT: u32 = 2;
    pub const INSTR: u32 = 7;

    pub const SPOT_15M_CANDLES: u32 = 20;
    pub const SPOT_1M_CANDLES: u32 = 19;
    pub const SPOT_ASK_ORDERS: u32 = 17;
    pub const SPOT_ASKS_TREE: u32 = 15;
    pub const SPOT_BID_ORDERS: u32 = 16;
    pub const SPOT_BIDS_TREE: u32 = 14;
    pub const SPOT_CLIENT_INFOS: u32 = 12;
    pub const SPOT_CLIENT_INFOS2: u32 = 13;
    pub const SPOT_DAY_CANDLES: u32 = 21;
    pub const SPOT_LINES: u32 = 18;
    pub const SPOT_MAPS: u32 = 10;
    pub const TOKEN: u32 = 4;
    pub const PERP_ASK_ORDERS: u32 = 36;
    pub const PERP_ASKS_TREE: u32 = 37;
    pub const PERP_BID_ORDERS: u32 = 38;
    pub const PERP_BIDS_TREE: u32 = 39;
    pub const PERP_CLIENT_INFOS: u32 = 41;
    pub const PERP_CLIENT_INFOS2: u32 = 42;
    pub const PERP_CLIENT_INFOS3: u32 = 43;
    pub const PERP_CLIENT_INFOS4: u32 = 44;
    pub const PERP_CLIENT_INFOS5: u32 = 45;
    pub const PERP_LINES: u32 = 46;
    pub const PERP_MAPS: u32 = 47;
    pub const PERP_LONG_PX_TREE: u32 = 48;
    pub const PERP_SHORT_PX_TREE: u32 = 49;
    pub const PERP_REBALANCE_TIME_TREE: u32 = 50;
    pub const PRIVATE_CLIENTS: u32 = 51;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[repr(u32)]
    pub enum AccountType {
        Holder = 1,
        Root = 2,
        Token = 4,
        Instr = 7,
        SpotMaps = 10,
        SpotClientAccounts = 11,
        SpotClientInfos = 12,
        SpotClientInfos2 = 13,
        SpotBidsTree = 14,
        SpotAsksTree = 15,
        SpotBidOrders = 16,
        SpotAskOrders = 17,
        SpotLines = 18,
        Spot1MCandles = 19,
        Spot15MCandles = 20,
        SpotDayCandles = 21,

        ClientPrimary = 31,
        Community = 34,
        ClientCommunity = 35,
        PerpAskOrders = 36,
        PerpAsksTree = 37,
        PerpBidOrders = 38,
        PerpBidsTree = 39,
        PerpClientAccounts = 40,
        PerpClientInfos = 41,
        PerpClientInfos2 = 42,
        PerpClientInfos3 = 43,
        PerpClientInfos4 = 44,
        PerpClientInfos5 = 45,
        PerpLines = 46,
        PerpMaps = 47,
        PerpLongPxTree = 48,
        PerpShortPxTree = 49,
        PerpRebalanceTimeTree = 50,
        PrivateClients = 51,

        ProgramTokenAccount,
        DrvsAuthority,
    }

    impl From<u32> for AccountType {
        fn from(value: u32) -> Self {
            unsafe { std::mem::transmute(value) }
        }
    }

    impl std::fmt::Display for AccountType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}({})", self, (*self) as u32)
        }
    }

    #[test]
    fn some_test() {
        let account_type = AccountType::Community;
        assert_eq!(format!("{}", account_type), "Community(34)".to_string());
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Zeroable, Pod)]
/// Discriminator is a unique identifier of every account in the system.
/// Should be stored in the first 8 bytes of accounts data.
pub struct Discriminator {
    pub tag: Tag,
    pub version: Version,
}

impl Discriminator {
    pub fn new(tag: Tag, version: Version) -> Self {
        Self { tag, version }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Operator {
    pub operator_address: Pubkey,
    pub version: Version,
    pub reserved: u32,
}

pub const OPERATOR_SIZE: usize = std::mem::size_of::<Operator>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
pub struct LineQuotes {
    pub px: i64,
    pub qty: i64,
}

pub const LINE_QUOTES_SIZE: usize = std::mem::size_of::<LineQuotes>();

#[repr(C)]
#[derive(Zeroable)]
pub struct TraceAccountHeader {
    pub header: Discriminator,
    pub id: InstrId,
    pub count: u32,
}

impl Deref for TraceAccountHeader {
    type Target = Discriminator;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

pub const TRACE_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<TraceAccountHeader>();

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BaseCrncyRecord {
    pub crncy_token_id: u32,
    pub decs_count: u32,
    pub funds: i64,
    pub rate: f64,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Token = 0x1000000,
    SpotLp = 0x2000000,
    SpotOrders = 0x3000000,
    Perp = 0x4000000,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Zeroable)]
pub struct ClientSpot {
    pub asset_id: u32,
    pub temp_client_id: ClientId,
    pub slot: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Zeroable)]
pub struct ClientPerp {
    pub asset_id: u32,
    pub temp_client_id: ClientId,
    pub slot: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
pub struct AssetRecord {
    pub asset_id: u32,
    // client
    pub temp_id: u32,
    pub value: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Order {
    pub qty: i64,
    pub sum: i64,
    pub order_id: i64,
    pub orig_client_id: ClientId,
    pub client_id: ClientId,
    pub line: u32,
    // price line order chain
    pub prev: u32,
    pub next: u32,
    // self ref
    pub sref: u32,
    // ref to node in RBTree account
    pub link: u32,
    // client orders chain
    pub cl_prev: u32,
    pub cl_next: u32,
    // creation time
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PxOrders {
    pub price: i64,
    pub qty: i64,
    // order book lines chain on each side
    pub next: u32,
    pub prev: u32,
    pub sref: u32,
    // ref to node in RBTree on according line
    pub link: u32,
    // ref to the first order in the line
    pub begin: u32,
    // ref to the last order in the line
    pub end: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Enum for token version verification
pub enum TokenProgram {
    Original,
    Token2022,
}

impl std::fmt::Display for TokenProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenProgram::Original => write!(f, "Original"),
            TokenProgram::Token2022 => write!(f, "Token2022"),
        }
    }
}
