use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
#[cfg(feature = "on-chain")]
use solana_program::pubkey::Pubkey;
#[cfg(feature = "off-chain")]
use solana_sdk::pubkey::Pubkey;

use crate::new_types::{client::ClientId, tag::Tag, version::Version};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
    use bytemuck::{Pod, Zeroable};
    use serde::{Deserialize, Serialize};

    #[repr(u32)]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    pub enum InstrFlag {
        PerpActive = 0x40000000,
        ReadyToPerpUpgrade = 0x01000000,
        ZeroFees = 0x1,
        FixedFees = 0x2,
        SimilarAssets = 0x4,
        UsdStablecoin = 0x8,
        Forex = 0x10,
        Suspended = 0x20,
        LongMarginCall = 0x40,
        ShortMarginCall = 0x80,
    }

    impl std::fmt::Display for InstrFlag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub trait SimpleInstrMask {
        fn get_flag(&self, flag: InstrFlag) -> bool;
        fn set_flag(&mut self, flag: InstrFlag);
        fn clear_flag(&mut self, flag: InstrFlag);
    }

    #[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct InstrMask(pub u32);

    impl InstrMask {
        pub fn merge(&mut self, input: InstrInputMask) {
            self.0 |= (input.0 as u32) & InstrInputMask::ALLOWED_FLAGS;
        }
    }

    impl SimpleInstrMask for InstrMask {
        fn get_flag(&self, flag: InstrFlag) -> bool {
            self.0 & flag as u32 != 0
        }
        fn set_flag(&mut self, flag: InstrFlag) {
            self.0 |= flag as u32
        }
        fn clear_flag(&mut self, flag: InstrFlag) {
            self.0 &= !(flag as u32)
        }
    }

    #[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct InstrInputMask(u8);

    impl InstrInputMask {
        pub const ALLOWED_FLAGS: u32 = InstrFlag::ZeroFees as u32
            | InstrFlag::FixedFees as u32
            | InstrFlag::SimilarAssets as u32
            | InstrFlag::UsdStablecoin as u32;
    }

    impl SimpleInstrMask for InstrInputMask {
        fn get_flag(&self, flag: InstrFlag) -> bool {
            if flag as u32 > u8::MAX as u32 {
                return false;
            }
            self.0 & flag as u8 != 0
        }
        fn set_flag(&mut self, flag: InstrFlag) {
            if flag as u32 > u8::MAX as u32 {
                return;
            }
            self.0 |= flag as u8
        }
        fn clear_flag(&mut self, flag: InstrFlag) {
            if flag as u32 > u8::MAX as u32 {
                return;
            }
            self.0 &= !(flag as u8)
        }
    }

    #[test]
    fn merge_test() {
        let mut instr_mask = InstrMask(0);
        instr_mask.set_flag(InstrFlag::ReadyToPerpUpgrade);

        let mut input_instr_mask = InstrInputMask(0);

        input_instr_mask.set_flag(InstrFlag::SimilarAssets);
        input_instr_mask.set_flag(InstrFlag::Forex);

        instr_mask.merge(input_instr_mask);

        assert!(instr_mask.get_flag(InstrFlag::Forex));
        assert!(instr_mask.get_flag(InstrFlag::ReadyToPerpUpgrade));
        assert!(instr_mask.get_flag(InstrFlag::SimilarAssets));
    }
}

pub mod token_mask {
    use bytemuck::{Pod, Zeroable};
    use serde::{Deserialize, Serialize};

    #[repr(u32)]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    pub enum TokenFlag {
        Token2022 = 0x80000000,
        BaseCrncy = 0x40000000,
        WrappedToken = 0x20000000,
        SMACrncy = 0x10000000,
    }

    impl std::fmt::Display for TokenFlag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TokenMask(pub u32);

    impl TokenMask {
        const DECIMALS_MASK: u32 = 0xFF;

        pub fn get_flag(&self, flag: TokenFlag) -> bool {
            self.0 & flag as u32 != 0
        }

        pub fn set_flag(&mut self, flag: TokenFlag) {
            self.0 |= flag as u32;
        }

        pub fn clear_flag(&mut self, flag: TokenFlag) {
            self.0 &= !(flag as u32);
        }

        pub fn decimals(&self) -> u8 {
            (self.0 & Self::DECIMALS_MASK) as u8
        }

        pub fn set_decimals(&mut self, decimals: u8) {
            self.0 = (self.0 & !Self::DECIMALS_MASK) | (decimals as u32);
        }
    }

    #[test]
    fn test_token_mask() {
        let mut mask = TokenMask(0);

        mask.set_decimals(9);
        assert_eq!(mask.decimals(), 9);

        mask.set_flag(TokenFlag::BaseCrncy);
        mask.set_flag(TokenFlag::SMACrncy);

        assert!(mask.get_flag(TokenFlag::BaseCrncy));
        assert!(mask.get_flag(TokenFlag::SMACrncy));
        assert!(!mask.get_flag(TokenFlag::WrappedToken));

        assert_eq!(mask.decimals(), 9);

        mask.clear_flag(TokenFlag::BaseCrncy);
        assert!(!mask.get_flag(TokenFlag::BaseCrncy));

        assert_eq!(mask.decimals(), 9);
        assert!(mask.get_flag(TokenFlag::SMACrncy));
    }
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
#[derive(Copy, Clone, Default, Debug, Zeroable, Pod, PartialEq)]
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
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
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
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
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

pub mod vm_status {
    use bytemuck::{Pod, Zeroable};
    use serde::{Deserialize, Serialize};

    use crate::constants::TradingSection;

    #[repr(u32)]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub enum VmFlag {
        Active = 0x80000000,
        Change = 0x40000000,
        Withdraw = 0x20000000,
    }

    #[derive(Clone, Copy, Pod, Zeroable)]
    #[repr(transparent)]
    pub struct VmMask(u32);

    impl VmMask {
        pub fn get_flag(&self, flag: VmFlag) -> bool {
            self.0 & flag as u32 != 0
        }
        pub fn set_flag(&mut self, flag: VmFlag) {
            self.0 |= flag as u32
        }
        pub fn clear_flag(&mut self, flag: VmFlag) {
            self.0 &= !(flag as u32)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SlotFlags(u8);

    #[repr(u8)]
    pub enum SlotFlag {
        Spot = 0b001,
        Perp = 0b010,
        Option = 0b100,
    }

    impl From<TradingSection> for SlotFlag {
        #[inline(always)]
        /// Safety: Bit is guaranteed to be in {1, 2, 4}
        fn from(section: TradingSection) -> Self {
            let bit = 1u8 << (section as u8);

            unsafe { core::mem::transmute::<u8, SlotFlag>(bit) }
        }
    }

    impl SlotFlags {
        pub fn spot(&self) -> bool {
            self.0 & SlotFlag::Spot as u8 != 0
        }
        pub fn perp(&self) -> bool {
            self.0 & SlotFlag::Perp as u8 != 0
        }
        pub fn option(&self) -> bool {
            self.0 & SlotFlag::Option as u8 != 0
        }

        pub fn get_slot_flag(&self, flag: SlotFlag) -> bool {
            self.0 & flag as u8 != 0
        }

        pub fn new(spot: bool, perp: bool, option: bool) -> Self {
            let mut v = 0;
            if spot {
                v |= SlotFlag::Spot as u8
            }
            if perp {
                v |= SlotFlag::Perp as u8
            }
            if option {
                v |= SlotFlag::Option as u8
            }
            Self(v)
        }

        pub fn none() -> Self {
            SlotFlags(0b000)
        }

        pub fn all() -> Self {
            SlotFlags(0b111)
        }
    }

    impl VmMask {
        const SLOT_BITS: u32 = 3;
        const SLOT_MASK: u32 = 0b111;

        fn slot_shift(index: usize) -> u32 {
            // TODO remove
            assert!(index < 8);
            (index as u32) * Self::SLOT_BITS
        }

        pub fn slot(&self, index: usize) -> SlotFlags {
            let shift = Self::slot_shift(index);
            let v = ((self.0 >> shift) & Self::SLOT_MASK) as u8;
            SlotFlags(v)
        }

        pub fn set_slot(&mut self, index: usize, flags: SlotFlags) {
            let shift = Self::slot_shift(index);
            self.0 &= !(Self::SLOT_MASK << shift);
            self.0 |= (flags.0 as u32) << shift;
        }

        pub fn reset_slots(&mut self) {
            self.0 &= 0xFF000000
        }
    }

    impl IntoIterator for VmMask {
        type Item = SlotFlags;

        type IntoIter = VmMaskIter;

        fn into_iter(self) -> Self::IntoIter {
            VmMaskIter {
                mask: self,
                index: 0,
            }
        }
    }

    pub struct VmMaskIter {
        pub mask: VmMask,
        pub index: usize,
    }

    impl Iterator for VmMaskIter {
        type Item = SlotFlags;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= 8 {
                return None;
            }

            let idx = self.index;
            self.index += 1;

            Some(self.mask.slot(idx))
        }
    }

    impl<'a> From<&'a [SlotFlags; 8]> for VmMask {
        fn from(value: &'a [SlotFlags; 8]) -> Self {
            let mut mask = VmMask(0);

            value.into_iter().zip(0..8).for_each(|(flags, index)| {
                mask.set_slot(index, *flags);
            });

            mask
        }
    }

    #[test]
    fn test_mask_iterator() {
        let flags = &[
            SlotFlags::none(),
            SlotFlags::all(),
            SlotFlags::none(),
            SlotFlags::all(),
            SlotFlags::none(),
            SlotFlags::new(true, false, false),
            SlotFlags::new(false, true, false),
            SlotFlags::new(false, false, true),
        ];

        let mask: VmMask = flags.into();

        let (size, spot) = mask.clone().into_iter().fold(
            (0_usize, 0_usize),
            |(counter, spot_counter), slot_flags| {
                if slot_flags.spot() {
                    (counter + 1, spot_counter + 1)
                } else {
                    (counter + 1, spot_counter)
                }
            },
        );

        assert_eq!(spot, 3, "Incorrect spot amount");
        assert_eq!(size, 8, "Incorrect spot amount");
    }
}

pub mod quote_status {
    use crate::constants::MAX_QUOTE_ORDERS;

    use super::*;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Zeroable, Pod)]
    pub struct QuoteOrder {
        pub new_price: i64,
        pub new_qty: i64,
        pub old_id: i64,
    }

    /// A mask that stores quote information in a u16:
    /// - First 4 bits (0-3): amount field (0-15)
    /// - Next 12 bits (4-15): array of booleans where bid = 1 and ask = 0
    #[derive(Clone, Copy, Pod, Zeroable, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct QuoteMask(pub u16);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct QuoteEntry {
        pub position: usize,
        pub quote_side: OrderSide,
    }

    impl QuoteMask {
        const AMOUNT_BITS: u16 = 4;
        const AMOUNT_MASK: u16 = 0b1111;
        const QUOTE_ARRAY_SIZE: usize = MAX_QUOTE_ORDERS as usize;

        pub fn new(amount: u8) -> Self {
            assert!(amount <= Self::QUOTE_ARRAY_SIZE as u8);
            Self(amount as u16)
        }

        pub fn amount(&self) -> u8 {
            (self.0 & Self::AMOUNT_MASK) as u8
        }

        pub fn set_amount(&mut self, amount: u8) {
            assert!(amount <= Self::QUOTE_ARRAY_SIZE as u8);
            self.0 = (self.0 & !Self::AMOUNT_MASK) | (amount as u16);
        }

        pub fn quote_side(&self, position: usize) -> OrderSide {
            let bit_position = Self::AMOUNT_BITS + position as u16;
            if (self.0 >> bit_position) & 1 != 0 {
                OrderSide::Bid
            } else {
                OrderSide::Ask
            }
        }

        /// bid: true for Bid, false for Ask
        pub fn set_quote(&mut self, position: usize, order_side: OrderSide) {
            assert!(position < Self::QUOTE_ARRAY_SIZE, "Position must be 0-11");
            let bit_position = Self::AMOUNT_BITS + position as u16;

            match order_side {
                OrderSide::Bid => self.0 |= 1 << bit_position,
                OrderSide::Ask => self.0 &= !(1 << bit_position),
            }
        }

        pub fn clear_quotes(&mut self) {
            self.0 &= Self::AMOUNT_MASK;
        }
    }

    impl IntoIterator for QuoteMask {
        type Item = QuoteEntry;
        type IntoIter = QuoteMaskIter;

        fn into_iter(self) -> Self::IntoIter {
            QuoteMaskIter {
                mask: self,
                len: self.amount() as usize,
                position: 0,
            }
        }
    }

    pub struct QuoteMaskIter {
        mask: QuoteMask,
        len: usize,
        position: usize,
    }

    impl Iterator for QuoteMaskIter {
        type Item = QuoteEntry;

        fn next(&mut self) -> Option<Self::Item> {
            if self.position >= self.len {
                return None;
            }

            let current_position = self.position;
            self.position += 1;

            Some(QuoteEntry {
                position: current_position,
                quote_side: self.mask.quote_side(current_position),
            })
        }
    }

    impl std::fmt::Display for QuoteMask {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuoteMask(amount: {}, quotes: [", self.amount())?;
            for (i, entry) in self.into_iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", entry.quote_side)?;
            }
            write!(f, "])")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_quote_mask_amount() {
            let mut mask = QuoteMask::new(5);
            assert_eq!(mask.amount(), 5);

            mask.set_amount(12);
            assert_eq!(mask.amount(), 12);
        }

        #[test]
        fn test_quote_mask_quotes() {
            let mut mask = QuoteMask::new(3);

            for i in 0..mask.amount() as usize {
                assert_eq!(mask.quote_side(i), OrderSide::Ask);
            }

            mask.set_quote(0, OrderSide::Bid);
            mask.set_quote(5, OrderSide::Bid);
            mask.set_quote(11, OrderSide::Bid);

            assert_eq!(mask.quote_side(0), OrderSide::Bid);
            assert_eq!(mask.quote_side(5), OrderSide::Bid);
            assert_eq!(mask.quote_side(11), OrderSide::Bid);

            assert_eq!(mask.quote_side(1), OrderSide::Ask);
            assert_eq!(mask.quote_side(6), OrderSide::Ask);
            assert_eq!(mask.quote_side(1), OrderSide::Ask);
        }

        #[test]
        fn test_quote_mask_iterator() {
            let mut mask = QuoteMask::new(7);

            for i in 0..mask.amount() as usize {
                mask.set_quote(
                    i,
                    if i % 2 == 0 {
                        OrderSide::Bid
                    } else {
                        OrderSide::Ask
                    },
                );
            }

            let entries: Vec<QuoteEntry> = mask.into_iter().collect();
            assert_eq!(entries.len(), 7);

            for (i, entry) in entries.iter().enumerate() {
                assert_eq!(entry.position, i);
                if i % 2 == 0 {
                    assert_eq!(entry.quote_side, OrderSide::Bid);
                } else {
                    assert_eq!(entry.quote_side, OrderSide::Ask);
                }
            }
        }
    }
}
