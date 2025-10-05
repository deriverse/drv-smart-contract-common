use std::ops::Deref;

use crate::state::{
    instrument::INSTR_ACCOUNT_HEADER_SIZE, spots::spot_account_header::SpotTradeAccountHeader,
    types::LINE_QUOTES_SIZE,
};

pub mod voting {

    pub enum VoteOption {
        Increment,
        Decrement,
        Unchange,
    }

    impl VoteOption {
        pub const INCREMENT: u8 = 2;
        pub const DECREMENT: u8 = 0;
    }
}

pub mod candles {

    use crate::state::types::account_type::AccountType;

    use super::*;
    #[derive(Clone, Copy)]
    pub struct CandleParams {
        pub tag: u32,
        pub capacity: u32,
        pub duration: u32,
    }

    pub struct CandleRegister {
        pub candles: &'static [CandleParams],
    }

    impl Deref for CandleRegister {
        type Target = &'static [CandleParams];

        fn deref(&self) -> &Self::Target {
            &self.candles
        }
    }

    pub const CANDLES: CandleRegister = CandleRegister {
        candles: &[
            CandleParams {
                tag: AccountType::Spot1MCandles as u32,
                capacity: 10080,
                duration: 60,
            },
            CandleParams {
                tag: AccountType::Spot15MCandles as u32,
                capacity: 2688,
                duration: 900,
            },
            CandleParams {
                tag: AccountType::SpotDayCandles as u32,
                capacity: 5844,
                duration: 86400,
            },
        ],
    };
}

pub const MINT_DECIMALS_OFFSET: usize = 44;
pub const HOLDER_SEED: &[u8; 7] = b"drvs001";
pub const DRVS_SEED: &[u8; 5] = b"ndxnt";

pub const HOUR: u32 = 3600;
pub const DAY: u32 = 86400;
pub const WEEK: u32 = DAY * 7;
pub const MONTH: u32 = WEEK * 4;
pub const QUARTER: u32 = 365 * DAY + 6 * HOUR;
pub const YEAR: u32 = WEEK * 52;
pub const FINAL_DURATION: u32 = 1800;
pub const SETTLEMENT: u32 = 28800;
pub const FIXING_DURATION: u32 = 300;
pub const DF_1: f64 = 1048576f64;
pub const RDF_1: f64 = 1f64 / 1048576f64;
pub const DI_1: i64 = 1048576;
pub const MIN_SIGMA: f64 = 0.01;
pub const MIN_SIGMA2: f64 = 0.0001;
pub const MAX_SIGMA: f64 = 0.2;
pub const DF: f64 = 1000000000.0;
pub const RDF: f64 = 0.000000001;
pub const NULL_NODE: u32 = 0xFFFFFFFF;
pub const NULL_ORDER: u32 = 0xFFFF;
pub const NULL_THREAD: u32 = 0xFFFF;
pub const NULL_INDEX: usize = 0xFFFF;
pub const NULL_CLIENT: u32 = 0xFFFFFF;
pub const NULL_INSTR: u32 = 0xFFFFFFF;
pub const NULL_TOKEN: u32 = 0xFFFFFFF;
pub const MEMORY_MAP_UNITS: usize = 4161;
pub const TRADE_MEMORY_MAP_UNITS: usize = 261;
pub const SMALL_MEMORY_MAP_UNITS: usize = 65;
pub const TOTAL_MEMORY_MAP_UNITS: usize =
    MEMORY_MAP_UNITS + 4 * TRADE_MEMORY_MAP_UNITS + SMALL_MEMORY_MAP_UNITS;
pub const TOTAL_PERP_MEMORY_MAP_UNITS: usize =
    4 * MEMORY_MAP_UNITS + 4 * TRADE_MEMORY_MAP_UNITS + SMALL_MEMORY_MAP_UNITS;
pub const BIDS_TREE_PT_OFFSET: usize =
    std::mem::size_of::<SpotTradeAccountHeader<0>>() + MEMORY_MAP_UNITS * 8;
pub const ASKS_TREE_PT_OFFSET: usize = BIDS_TREE_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
pub const BID_ORDERS_PT_OFFSET: usize = ASKS_TREE_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
pub const ASK_ORDERS_PT_OFFSET: usize = BID_ORDERS_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
pub const LINES_PT_OFFSET: usize = ASK_ORDERS_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
pub const LONG_PX_TREE_PT_OFFSET: usize = LINES_PT_OFFSET + SMALL_MEMORY_MAP_UNITS * 8;
pub const SHORT_PX_TREE_PT_OFFSET: usize = LONG_PX_TREE_PT_OFFSET + MEMORY_MAP_UNITS * 8;
pub const REBALANCING_PT_OFFSET: usize = SHORT_PX_TREE_PT_OFFSET + MEMORY_MAP_UNITS * 8;
pub const FUTURES_MAX_QTY: i64 = 0x40000000;
pub const PDF_SIGMA_WIDTH: f64 = 10.0;
pub const SIGMA_FACTOR: f64 = 0.845154f64;
pub const PDF_WIDTH: f64 = PDF_SIGMA_WIDTH / SIGMA_FACTOR;
pub const MIN_FEE_RATE: u32 = 4;
pub const FEE_RATE_STEP: f64 = 0.000025;
pub const START_SPOT_FEE_RATE: u32 = 20;
pub const START_PERP_FEE_RATE: u32 = 20;
pub const START_FUTURES_FEE_RATE: u32 = 20;
pub const START_OPTIONS_FEE_RATE: u32 = 20;
pub const MIN_POOL_RATIO: u32 = 4;
pub const MAX_POOL_RATIO: u32 = 36;
pub const MIN_MARGIN_CALL_PENALTY_RATE: u32 = 5;
pub const POOL_RATIO_STEP: f64 = 0.025;
pub const MARGIN_CALL_PENALTY_RATE_STEP: f64 = 0.001;
pub const START_SPOT_POOL_RATIO: u32 = 10;
pub const START_PERP_POOL_RATIO: u32 = 10;
pub const START_MARGIN_CALL_PENALTY_RATE: u32 = 10;
pub const START_OPTIONS_POOL_RATIO: u32 = 10;
pub const START_FEES_PREPAYMENT_FOR_MAX_DISCOUNT: u32 = 50;
pub const MARKET_DEPTH: usize = 20;
pub const STRIKES_COUNT: usize = 100;
pub const MAPS_SIZE: usize = 42160;
pub const PERP_MAPS_SIZE: usize = 175312;
pub const FRACTIONS: usize = 65;
pub const MAX_LINES: usize = 2048;
pub const MAX_ORDERS: u32 = 14336;
pub const MAX_DURATION: usize = 28;
pub const OPTIONS_POOL_TOKEN_DECIMALS: u8 = 6;
pub const INSTR_ACCOUNT_INITIAL_SIZE: usize =
    INSTR_ACCOUNT_HEADER_SIZE + 4 * LINE_QUOTES_SIZE * MARKET_DEPTH;
pub const MAX_SUM: f64 = 1_000_000_000_000_000_000.0;
pub const SPOT_POOL_UNIT: f64 = 0.0001;
pub const MAX_PRICE: i64 = i64::MAX >> 4;
pub const MIN_INIT_PRICE: i64 = 1000;
pub const SPOT_MAX_AMOUNT: i64 = i64::MAX >> 8;
pub const MAX_ORDER_ID: i64 = i64::MAX >> 1;
pub const PERP_REBATE: i64 = 15;
pub const MAX_FEES_DISCOUNT: f64 = 0.75;
pub const REBATES_RATIO: f64 = 0.125;
pub const FEES_PREPAYMENT_STEP: f64 = 1000.0;
pub const MIN_FEES_PREPAYMENT_FOR_MAX_DISCOUNT: u32 = 10;
pub const DEC_30: f64 = (1i64 << 30) as f64;
pub const MARGIN_CALL_LEVEL: i64 = 32;
pub const LONG_MARGIN_CALL_RATIO: f64 = 1.0 + 1.0 / MARGIN_CALL_LEVEL as f64;
pub const SHORT_MARGIN_CALL_RATIO: f64 = 1.0 - 1.0 / MARGIN_CALL_LEVEL as f64;
pub const MAX_MARGIN_CALL_TRADES: i64 = 10;
pub const MAX_REBALANCING_CALLS: i64 = 25;
pub const REBALANCING_DELAY: u32 = 300;
pub const MAX_PERP_CLIENTS_COUNT: u32 = 250000;
pub const MAX_PERP_CLIENTS_THRESHOLD: u32 = MAX_PERP_CLIENTS_COUNT - 1000;
pub const MIN_LIQUIDATION_THRESHOLD: f64 = 0.5 / MAX_PERP_LEVERAGE as f64;
pub const MAX_PERP_LEVERAGE: u8 = 15;
pub const FEES_PREPAYMENT_LOCKUP_PERIOD: u32 = 91 * DAY;
pub const SPREAD_THRESHOLD: f64 = 0.005;
pub const TRADES_THRESHOLD: i64 = 100000;
pub const MAX_REF_DISCOUNT: f64 = 0.1;
pub const MAX_REF_RATIO: f64 = 0.5;
pub const MAX_SUPPLY: i64 = 250_000;
pub const INIT_SEAT_PRICE: f64 = 1.0;
pub const MIN_MAX_DISCOUNT_RATE: u32 = 10;
pub const MAX_MAX_DISCOUNT_RATE: u32 = 30;
pub const MAX_DISCOUNT_STEP: f64 = 0.025;
pub const START_MAX_DISCOUNT: u32 = 30;
// TODO wrap in the feature
pub mod competition {
    pub const COMPETITION_ID: u8 = 3;
    pub const COMPETITION_START: u32 = 1753948800;
    pub const COMPETITION_END: u32 = 1755158400;
    pub const COMPETITION_CRNCY_ID: u32 = 1;
    pub const COMPETITION_SUM: i64 = 10_000_000_000;
}
