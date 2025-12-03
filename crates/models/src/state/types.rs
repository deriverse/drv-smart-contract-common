use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;
#[cfg(feature = "off-chain")]
use solana_sdk::pubkey::Pubkey;

use crate::new_types::{client::ClientId, tag::Tag, version::Version};

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

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum MarketSeatOrderType {
    Buy,
    Sell,
}

impl std::fmt::Display for MarketSeatOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MarketSeatOrderType::Buy => "Buy",
                MarketSeatOrderType::Sell => "Sell",
            }
        )
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
    MarginCall = 2,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Limit => "Limit",
                Self::Market => "Market",
                Self::MarginCall => "Margin Call",
            }
        )
    }
}

pub mod root_mask {
    pub const PRIVATE_MODE: u32 = 0x1;
}

pub mod instr_mask {
    pub const DRV: u32 = 0x10000000;
    pub const PERP: u32 = 0x40000000;
    pub const ORACLE: u32 = 0x80000000;
    pub const READY_TO_PERP_UPGRADE: u32 = 0x1000000;
}

pub mod account_type {

    #[cfg(feature = "on-chain")]
    use solana_program::program_error::ProgramError;
    #[cfg(feature = "off-chain")]
    use solana_sdk::program_error::ProgramError;

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

    impl TryFrom<u32> for AccountType {
        type Error = ProgramError;
        fn try_from(value: u32) -> Result<Self, Self::Error> {
            Ok(match value {
                1 => Self::Holder,
                2 => Self::Root,
                4 => Self::Token,
                7 => Self::Instr,
                10 => Self::SpotMaps,
                11 => Self::SpotClientAccounts,
                12 => Self::SpotClientInfos,
                13 => Self::SpotClientInfos2,
                14 => Self::SpotBidsTree,
                15 => Self::SpotAsksTree,
                16 => Self::SpotBidOrders,
                17 => Self::SpotAskOrders,
                18 => Self::SpotLines,
                19 => Self::Spot1MCandles,
                20 => Self::Spot15MCandles,
                21 => Self::SpotDayCandles,
                31 => Self::ClientPrimary,
                34 => Self::Community,
                35 => Self::ClientCommunity,
                36 => Self::PerpAskOrders,
                37 => Self::PerpAsksTree,
                38 => Self::PerpBidOrders,
                39 => Self::PerpBidsTree,
                40 => Self::PerpClientAccounts,
                41 => Self::PerpClientInfos,
                42 => Self::PerpClientInfos2,
                43 => Self::PerpClientInfos3,
                44 => Self::PerpClientInfos4,
                45 => Self::PerpClientInfos5,
                46 => Self::PerpLines,
                47 => Self::PerpMaps,
                48 => Self::PerpLongPxTree,
                49 => Self::PerpShortPxTree,
                50 => Self::PerpRebalanceTimeTree,
                51 => Self::PrivateClients,

                _ => return Err(ProgramError::InvalidAccountData),
            })
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
/// Line Quotes
///
/// 1. **`px`** - price
/// 2. **`qty`** - quantity
///
/// # Notes
/// - Used for readable lines records
/// - Stored in a vec: spot_bids, spot_asks, perp_bids, perp_asks
pub struct LineQuotes {
    pub px: i64,
    pub qty: i64,
}

pub const LINE_QUOTES_SIZE: usize = std::mem::size_of::<LineQuotes>();

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
/// Base Crncy Record
///
/// 1. **`crncy_token_id`** - Token id from token state
/// 2. **`decs_count`** - tokens decimals from token state mask
/// 3. **`funds`** - Funds available for dividents distribution
/// 4. **`rate`** - Current dividends rate per 1 DRVS token
/// 5. **`denominator`** - Denominator of base crncy for fees prepayment calculations, aligned by operator admin
/// 6. **`locked_drvs_amount`** - Locked amount of base crncy in DRVS/BaseCrncy pool
/// 7. **`locked_drvs_dividends_value`** - Amount of dividents ready to claim from locked base crncy in DRVS/BaseCrncy pool
/// 8. **`mask`**
pub struct BaseCrncyRecord {
    pub crncy_token_id: u32,
    pub decs_count: u32,
    pub funds: i64,
    pub rate: f64,
    pub denominator: f64,
    pub locked_drvs_amount: i64,
    pub locked_drvs_dividends_value: i64,
    pub mask: i64,
}

// change base crncy denominator base_crncy_id + new denominator must be > 0

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Token = 0x10000000,
    SpotLp = 0x20000000,
    SpotOrders = 0x30000000,
    Perp = 0x40000000,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Debug)]
pub struct AssetRecord {
    pub asset_id: u32,
    // client
    pub temp_id: u32,
    pub value: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Order
///
/// 1. **`qty`** — The total quantity of the order.
/// 2. **`sum`** — The total value of the order, calculated as
///    `price * qty * (1 / instr.decimal_factor)`.
/// 3. **`order_id`** — A unique identifier assigned to the order.
/// 4. **`orig_client_id`** — The identifier of the client who originally created the order.
/// 5. **`client_id`** — A temporary client identifier.
/// 6. **`line`** — The price line to which the order belongs.
/// 7. **`prev`** — A reference to the previous order in the price line’s linked list.
/// 8. **`next`** — A reference to the next order in the price line’s linked list.
/// 9. **`sref`** — The order’s index within the order memory map.
/// 10. **`link`** — The node index of the order within the RB Tree memory map.
/// 11. **`cl_prev`** — A reference to the previous order in the client’s linked list.
/// 12. **`cl_next`** — A reference to the next order in the client’s linked list.
/// 13. **`time`** — The timestamp representing when the order was created.
///
/// # Notes
///
/// - Orders are stored in a RB Tree keyed by `order_id`.
/// - Each price line maintains its own linked list of orders.
/// - Each client also maintains a linked list of their orders.
/// - When an order is not present, the constant `NULL_ORDER` is used to represent a `None` value.
pub struct Order {
    pub qty: i64,
    pub sum: i64,
    pub order_id: i64,
    pub orig_client_id: ClientId,
    pub client_id: ClientId,
    pub line: u32,
    pub prev: u32,
    pub next: u32,
    pub sref: u32,
    pub link: u32,
    pub cl_prev: u32,
    pub cl_next: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// PxOrders(Lines)
///
/// Each `PxOrders` structure corresponds to a specific price level and maintains
/// references to the linked orders at that price, as well as its position within
/// the order book structure.
///
/// # Fields
///
/// 1. **`price`** — The price level of the line.
/// 2. **`qty`** — The total aggregated quantity of all orders at this price level.
/// 3. **`next`** — A reference to the next price line in the order book chain.
/// 4. **`prev`** — A reference to the previous price line in the order book chain.
/// 5. **`sref`** — The index of this price line within the lines memory map.
/// 6. **`link`** — A node index of corresponding node in RB Tree.
/// 7. **`begin`** — A reference to the first order in line.
/// 8. **`end`** — A reference to the last order in line.
///
/// # Notes
///
/// - Price lines form a doubly linked list.
/// - Prices are aligned to SpotPrams or PerpParams list.
pub struct PxOrders {
    pub price: i64,
    pub qty: i64,
    pub next: u32,
    pub prev: u32,
    pub sref: u32,
    pub link: u32,
    pub begin: u32,
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
