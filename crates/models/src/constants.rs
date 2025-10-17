use std::ops::Deref;

use trading_limitations::MARKET_DEPTH;

use crate::state::{
    instrument::INSTR_ACCOUNT_HEADER_SIZE,
    types::{
        account_type::{SPOT_15M_CANDLES, SPOT_1M_CANDLES, SPOT_DAY_CANDLES},
        LINE_QUOTES_SIZE,
    },
};

pub mod candles {

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
                tag: SPOT_1M_CANDLES,
                capacity: 10080,
                duration: 60,
            },
            CandleParams {
                tag: SPOT_15M_CANDLES,
                capacity: 2688,
                duration: 900,
            },
            CandleParams {
                tag: SPOT_DAY_CANDLES,
                capacity: 5844,
                duration: 86400,
            },
        ],
    };
}

pub mod price_helper {
    pub const MINT_DECIMALS_OFFSET: usize = 44;
    pub const MIN_DECS_COUNT: u32 = 4;
    pub const MAX_DECS_COUNT: u32 = 9;
}

pub mod seeds {
    pub const HOLDER_SEED: &[u8; 7] = b"drvs001";
    pub const DRVS_SEED: &[u8; 5] = b"ndxnt";
}

pub mod time {
    pub const HOUR: u32 = 3600;
    pub const DAY: u32 = 86400;
    pub const WEEK: u32 = DAY * 7;
    pub const MONTH: u32 = WEEK * 4;
    pub const QUARTER: u32 = 365 * DAY + 6 * HOUR;
    pub const YEAR: u32 = WEEK * 52;
    pub const SETTLEMENT: u32 = 28800;
    pub const FIXING_DURATION: u32 = 300;
    pub const FEES_PREPAYMENT_LOCKUP_PERIOD: u32 = 91 * DAY;
}

pub const DF: f64 = 1000000000.0;
pub const RDF: f64 = 0.000000001;

pub mod nulls {
    pub const NULL_NODE: u32 = 0xFFFFFFFF;
    pub const NULL_ORDER: u32 = 0xFFFF;
    pub const NULL_THREAD: u32 = 0xFFFF;
    pub const NULL_INDEX: usize = 0xFFFF;
    pub const NULL_CLIENT: u32 = 0xFFFFFF;
    pub const NULL_INSTR: u32 = 0xFFFFFFF;
    pub const NULL_TOKEN: u32 = 0xFFFFFFF;
}
pub mod memory_maps {
    pub const MEMORY_MAP_UNITS: usize = 1 + 64 + 64 * 64;
    pub const EXTENDED_MEMORY_MAP_UNITS: usize = 1 + 16 + 16 * 64;
    pub const TRADE_MEMORY_MAP_UNITS: usize = 1 + 4 + 4 * 64;
    pub const SMALL_MEMORY_MAP_UNITS: usize = 1 + 64;
}

pub mod spot {
    pub const MAX_LINES: usize = 2048;
    pub const MAX_ORDERS: u32 = (4 * 64 * 64 - MAX_LINES) as u32 - 2;
    pub mod memory_maps {
        use crate::state::spots::spot_account_header::SpotTradeAccountHeader;

        use super::super::memory_maps::*;

        pub const BIDS_TREE_PT_OFFSET: usize =
            std::mem::size_of::<SpotTradeAccountHeader<0>>() + MEMORY_MAP_UNITS * 8;
        pub const ASKS_TREE_PT_OFFSET: usize = BIDS_TREE_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const BID_ORDERS_PT_OFFSET: usize = ASKS_TREE_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const ASK_ORDERS_PT_OFFSET: usize = BID_ORDERS_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const LINES_PT_OFFSET: usize = ASK_ORDERS_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;

        pub const MAPS_SIZE: usize = LINES_PT_OFFSET + MEMORY_MAP_UNITS * 8;
    }
}

pub mod perp {
    pub const MAX_LINES: usize = 2048 * 4;
    pub const MAX_ORDERS: u32 = (16 * 64 * 64 - MAX_LINES) as u32 - 2;

    pub const MAX_SUPPLY: i64 = 250_000;
    pub const INIT_SEAT_PRICE: f64 = 1.0;

    pub const MIN_LIQUIDATION_THRESHOLD: f64 = 0.5 / MAX_PERP_LEVERAGE as f64;
    pub const MAX_PERP_LEVERAGE: u8 = 15;

    pub mod memory_maps {
        use super::super::memory_maps::*;
        use crate::state::spots::spot_account_header::SpotTradeAccountHeader;

        pub const BIDS_TREE_PT_OFFSET: usize =
            std::mem::size_of::<SpotTradeAccountHeader<0>>() + MEMORY_MAP_UNITS * 8;
        pub const ASKS_TREE_PT_OFFSET: usize = BIDS_TREE_PT_OFFSET + EXTENDED_MEMORY_MAP_UNITS * 8;
        pub const BID_ORDERS_PT_OFFSET: usize = ASKS_TREE_PT_OFFSET + EXTENDED_MEMORY_MAP_UNITS * 8;
        pub const ASK_ORDERS_PT_OFFSET: usize =
            BID_ORDERS_PT_OFFSET + EXTENDED_MEMORY_MAP_UNITS * 8;
        pub const LINES_PT_OFFSET: usize = ASK_ORDERS_PT_OFFSET + EXTENDED_MEMORY_MAP_UNITS * 8;
        pub const LONG_PX_TREE_PT_OFFSET: usize = LINES_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const SHORT_PX_TREE_PT_OFFSET: usize = LONG_PX_TREE_PT_OFFSET + MEMORY_MAP_UNITS * 8;
        pub const REBALANCING_PT_OFFSET: usize = SHORT_PX_TREE_PT_OFFSET + MEMORY_MAP_UNITS * 8;

        pub const MAPS_SIZE: usize = REBALANCING_PT_OFFSET + MEMORY_MAP_UNITS * 8;
    }
}

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

    /// Topic 1 - Fee rate
    pub const MIN_FEE_RATE: u32 = 4;
    pub const FEE_RATE_STEP: f64 = 0.000025;
    pub const START_SPOT_FEE_RATE: u32 = 20;
    pub const START_PERP_FEE_RATE: u32 = 20;

    // Topic 2 - Pool ratio
    pub const MIN_POOL_RATIO: u32 = 4;
    pub const MAX_POOL_RATIO: u32 = 36;
    pub const POOL_RATIO_STEP: f64 = 0.025;
    pub const START_SPOT_POOL_RATIO: u32 = 10;
    pub const START_PERP_POOL_RATIO: u32 = 10;

    // Topic 3 - Margin call penalty rate
    pub const MIN_MARGIN_CALL_PENALTY_RATE: u32 = 5;
    pub const MARGIN_CALL_PENALTY_RATE_STEP: f64 = 0.001;
    pub const START_MARGIN_CALL_PENALTY_RATE: u32 = 10;

    // Topic 4 - Fee prepayment for max discount
    pub const START_FEES_PREPAYMENT_FOR_MAX_DISCOUNT: u32 = 50;
    pub const FEES_PREPAYMENT_STEP: f64 = 1000.0;
    pub const MIN_FEES_PREPAYMENT_FOR_MAX_DISCOUNT: u32 = 10;

    // Topic 5 - Max discount rate
    pub const MIN_MAX_DISCOUNT_RATE: u32 = 10;
    pub const MAX_MAX_DISCOUNT_RATE: u32 = 30;
    pub const MAX_DISCOUNT_STEP: f64 = 0.025;
    pub const START_MAX_DISCOUNT: u32 = 30;
}

pub mod trading_limitations {
    pub const MAX_SUM: f64 = 1_000_000_000_000_000_000.0;
    pub const MIN_QTY: i64 = 10000;
    pub const MAX_PRICE: i64 = i64::MAX >> 4;
    pub const MIN_PRICE: i64 = 1000;
    pub const SPOT_MAX_AMOUNT: i64 = i64::MAX >> 8;
    // questianable
    pub const MARKET_DEPTH: usize = 20;
    pub const MAX_ORDER_ID: i64 = i64::MAX >> 1;
}

pub mod pool {
    pub const SPOT_POOL_UNIT: f64 = 0.0001;
}

pub mod volatility_params {
    pub const MAX_DURATION: usize = 28;
    pub const SPREAD_THRESHOLD: f64 = 0.005;
    pub const TRADES_THRESHOLD: i64 = 100000;
}

pub const INSTR_ACCOUNT_INITIAL_SIZE: usize =
    INSTR_ACCOUNT_HEADER_SIZE + 4 * LINE_QUOTES_SIZE * MARKET_DEPTH;

pub mod rebates {
    pub const REBATES_RATIO: f64 = 0.125;

    pub const DEC_PRECISION: u32 = 63;
    pub const DEC_63: f64 = (1u64 << 63) as f64;

    pub const MAX_REBALANCING_CALLS: i64 = 25;
    pub const REBALANCING_DELAY: u32 = 300;
}

pub mod margin_call {
    pub const MARGIN_CALL_LEVEL: i64 = 32;
    pub const LONG_MARGIN_CALL_RATIO: f64 = 1.0 + 1.0 / MARGIN_CALL_LEVEL as f64;
    pub const SHORT_MARGIN_CALL_RATIO: f64 = 1.0 - 1.0 / MARGIN_CALL_LEVEL as f64;
    pub const MAX_MARGIN_CALL_TRADES: i64 = 10;
}

pub mod ref_constants {
    pub const MAX_REF_DISCOUNT: f64 = 0.1;
    pub const MAX_REF_RATIO: f64 = 0.5;
}

pub mod private_mode {
    #[cfg(not(feature = "test-sbf"))]
    pub const MAX_PRIVATE_CLIENTS_IN_QUEUE: u32 = 512;
    #[cfg(feature = "test-sbf")]
    pub const MAX_PRIVATE_CLIENTS_IN_QUEUE: u32 = 2;
}

#[cfg(feature = "competition")]
pub mod competition {
    pub const COMPETITION_ID: u8 = 3;
    pub const COMPETITION_START: u32 = 1753948800;
    pub const COMPETITION_END: u32 = 1755158400;
    pub const COMPETITION_CRNCY_ID: u32 = 1;
    pub const COMPETITION_SUM: i64 = 10_000_000_000;
}

pub mod instructions {
    pub mod new_holder_account {
        pub const INSTRUCTION_CODE: u8 = 0;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 3;
    }

    pub mod new_operator {
        pub const INSTRUCTION_CODE: u8 = 1;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 4;
    }

    pub mod new_root_account {
        pub const INSTRUCTION_CODE: u8 = 2;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 12;
    }

    pub mod perp_withdraw {
        pub const INSTRUCTION_CODE: u8 = 3;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 20;
    }

    pub mod new_base_crncy {
        pub const INSTRUCTION_CODE: u8 = 4;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 8;
    }

    pub mod fees_deposit {
        pub const INSTRUCTION_CODE: u8 = 5;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 6;
    }

    pub mod deposit {
        pub const INSTRUCTION_CODE: u8 = 7;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 9;
    }

    pub mod withdraw {
        pub const INSTRUCTION_CODE: u8 = 8;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 11;
    }

    pub mod new_instrument {
        pub const INSTRUCTION_CODE: u8 = 9;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 23;
    }

    pub mod upgrade_to_perp {
        pub const INSTRUCTION_CODE: u8 = 10;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 21;
    }

    pub mod perp_deposit {
        pub const INSTRUCTION_CODE: u8 = 11;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 19;
    }

    pub mod new_spot_order {
        pub const INSTRUCTION_CODE: u8 = 12;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 18;
    }

    pub mod spot_order_cancel {
        pub const INSTRUCTION_CODE: u8 = 13;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 14;
    }

    pub mod spot_lp {
        pub const INSTRUCTION_CODE: u8 = 14;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 5;
    }

    pub mod spot_mass_cancel {
        pub const INSTRUCTION_CODE: u8 = 15;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 14;
    }

    pub mod next_voting {
        pub const INSTRUCTION_CODE: u8 = 16;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 3;
    }

    pub mod new_perp_order {
        pub const INSTRUCTION_CODE: u8 = 19;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 21;
    }

    pub mod dividends_allocation {
        pub const INSTRUCTION_CODE: u8 = 25;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 4;
    }

    pub mod swap {
        pub const INSTRUCTION_CODE: u8 = 26;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 27;
    }

    pub mod airdrop {
        pub const INSTRUCTION_CODE: u8 = 27;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 11;
    }

    pub mod dividends_claim {
        pub const INSTRUCTION_CODE: u8 = 28;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 6;
    }

    pub mod perp_order_cancel {
        pub const INSTRUCTION_CODE: u8 = 30;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 20;
    }

    pub mod voting {
        pub const INSTRUCTION_CODE: u8 = 32;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 6;
    }

    pub mod spot_quotes_replace {
        pub const INSTRUCTION_CODE: u8 = 34;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 18;
    }

    pub mod perp_mass_cancel {
        pub const INSTRUCTION_CODE: u8 = 36;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 20;
    }

    pub mod perp_change_leverage {
        pub const INSTRUCTION_CODE: u8 = 37;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 20;
    }

    pub mod fees_withdraw {
        pub const INSTRUCTION_CODE: u8 = 39;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 6;
    }

    pub mod set_instr_oracle_feed {
        pub const INSTRUCTION_CODE: u8 = 40;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 4;
    }

    pub mod set_instr_ready_for_perp_upgrade {
        pub const INSTRUCTION_CODE: u8 = 41;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 3;
    }

    pub mod perp_quotes_replace {
        pub const INSTRUCTION_CODE: u8 = 42;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 21;
    }

    pub mod move_spot_avail_funds {
        pub const INSTRUCTION_CODE: u8 = 43;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 6;
    }

    pub mod change_ref_program {
        pub const INSTRUCTION_CODE: u8 = 44;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 2;
    }

    pub mod new_ref_link {
        pub const INSTRUCTION_CODE: u8 = 45;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 3;
    }

    pub mod perp_statistics_reset {
        pub const INSTRUCTION_CODE: u8 = 46;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 20;
    }

    pub mod buy_market_seat {
        pub const INSTRUCTION_CODE: u8 = 47;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 19;
    }

    pub mod sell_market_seat {
        pub const INSTRUCTION_CODE: u8 = 48;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 20;
    }

    pub mod new_private_client {
        pub const INSTRUCTION_CODE: u8 = 49;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 5;
    }

    pub mod terminate_private_mode {
        pub const INSTRUCTION_CODE: u8 = 50;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 3;
    }

    pub mod change_points_program_expiration {
        pub const INSTRUCTION_CODE: u8 = 51;
        pub const MIN_ACCOUNTS_AMOUNT: usize = 2;
    }
}
