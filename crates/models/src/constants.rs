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

pub mod volatility {
    pub const MIN_VARIANCE: f64 = 0.0001;
    pub const MAX_VARIANCE: f64 = 0.25;
    pub const INIT_VARIANCE: f64 = 0.01f64;
    pub const INIT_DAY_VOLATILITY: f64 = 0.1f64;
}

pub const DF: f64 = 1000000000.0;
pub const RDF: f64 = 0.000000001;

pub const WALLET_RESERVE_LAMPORTS: u64 = 10_000_000;

pub const MIN_AMOUNT_FOR_DIVIDENDS_ALLOCATION: i64 = 10;

pub const MAX_DENOMINATOR: f64 = 500.0;
pub const MIN_DENOMINATOR: f64 = 0.01;

pub const EMA_PERIOD: f64 = 60f64;

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
    pub const MAX_CLIENT_SIDE_ORDERS_COUNT: u32 = 32;
    pub mod memory_maps {
        use crate::state::spots::spot_account_header::SpotTradeAccountHeader;

        use super::super::memory_maps::*;

        pub const BIDS_TREE_PT_OFFSET: usize =
            std::mem::size_of::<SpotTradeAccountHeader<0>>() + MEMORY_MAP_UNITS * 8;
        pub const ASKS_TREE_PT_OFFSET: usize = BIDS_TREE_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const BID_ORDERS_PT_OFFSET: usize = ASKS_TREE_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const ASK_ORDERS_PT_OFFSET: usize = BID_ORDERS_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;
        pub const LINES_PT_OFFSET: usize = ASK_ORDERS_PT_OFFSET + TRADE_MEMORY_MAP_UNITS * 8;

        pub const MAPS_SIZE: usize = LINES_PT_OFFSET + SMALL_MEMORY_MAP_UNITS * 8;
    }
}

pub mod perp {

    pub const MAX_LINES: usize = 2048 * 4;
    pub const MAX_ORDERS: u32 = (16 * 64 * 64 - MAX_LINES) as u32 - 2;

    pub const MAX_SUPPLY: i64 = 262_200;
    pub const INIT_SEAT_PRICE: f64 = 1.0;
    pub const MAX_PERP_CLIENTS: u32 = 262_143;

    pub const MIN_LIQUIDATION_THRESHOLD: f64 = 0.5 / MAX_PERP_LEVERAGE as f64;
    // change to 20
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

    pub const MIN_VOTING_QUORUM: i64 = 5;

    /// Topic 1 - Fee rate
    pub const MIN_FEE_RATE: u32 = 10;
    pub const MAX_FEE_RATE: u32 = 30;
    pub const FEE_RATE_STEP: f64 = 0.0005;
    pub const START_SPOT_FEE_RATE: u32 = 20;
    pub const START_PERP_FEE_RATE: u32 = 20;

    /// Topic 2 - Pool ratio
    pub const MIN_POOL_RATIO: u32 = 4;
    pub const MAX_POOL_RATIO: u32 = 36;
    pub const POOL_RATIO_STEP: f64 = 0.025;
    pub const START_SPOT_POOL_RATIO: u32 = 10;

    /// Topic 3 - Margin call penalty rate
    pub const MIN_MARGIN_CALL_PENALTY_RATE: u32 = 4;
    pub const MAX_MARGIN_CALL_PENALTY_RATE: u32 = 20;
    pub const MARGIN_CALL_PENALTY_RATE_STEP: f64 = 0.025;
    pub const START_MARGIN_CALL_PENALTY_RATE: u32 = 10;

    /// Topic 4 - Fee prepayment for max discount
    pub const START_FEES_PREPAYMENT_FOR_MAX_DISCOUNT: u32 = 50;
    pub const FEES_PREPAYMENT_STEP: f64 = 1000.0;
    pub const MIN_FEES_PREPAYMENT_FOR_MAX_DISCOUNT: u32 = 10;

    /// Topic 5 - Max discount rate
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
    pub const MARKET_DEPTH: usize = 20;
    pub const MAX_ORDER_ID: i64 = i64::MAX >> 1;
}

pub mod pool {
    pub const SPOT_POOL_UNIT: f64 = 0.0001;
}

pub mod instr_upgrade_params {
    pub const MAX_DURATION: usize = 28;
    pub const SPREAD_THRESHOLD: f64 = 0.005;
    pub const TRADES_THRESHOLD: i64 = 100000;
}

pub const INSTR_ACCOUNT_INITIAL_SIZE: usize =
    INSTR_ACCOUNT_HEADER_SIZE + 4 * LINE_QUOTES_SIZE * MARKET_DEPTH;

pub const MAX_INSTR_COUNT: u32 = 0x10000000;

pub mod rebates {
    pub const REBATES_RATIO: f64 = 0.125;

    pub const DEC_PRECISION: u32 = 63;
    pub const DEC_63: f64 = (1u64 << 63) as f64;
}

pub mod rebalancing {
    // reduce to 10
    pub const MAX_REBALANCING_CALLS: i64 = 25;
    // change to 900
    pub const REBALANCING_DELAY: u32 = 300;
}

pub mod margin_call {
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
    pub trait DrvInstruction {
        const INSTRUCTION_NUMBER: u8;
        const MIN_ACCOUNTS: usize;
    }

    pub struct NewHolderInstruction;
    impl DrvInstruction for NewHolderInstruction {
        const INSTRUCTION_NUMBER: u8 = 0;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct NewOperatorInstruction;
    impl DrvInstruction for NewOperatorInstruction {
        const INSTRUCTION_NUMBER: u8 = 1;
        const MIN_ACCOUNTS: usize = 4;
    }

    pub struct NewRootAccountInstruction;
    impl DrvInstruction for NewRootAccountInstruction {
        const INSTRUCTION_NUMBER: u8 = 2;
        const MIN_ACCOUNTS: usize = 12;
    }

    pub struct PerpWithdrawInstruction;
    impl DrvInstruction for PerpWithdrawInstruction {
        const INSTRUCTION_NUMBER: u8 = 3;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct NewBaseCrncyInstruction;
    impl DrvInstruction for NewBaseCrncyInstruction {
        const INSTRUCTION_NUMBER: u8 = 4;
        const MIN_ACCOUNTS: usize = 9;
    }

    pub struct FeesDepositInstruction;
    impl DrvInstruction for FeesDepositInstruction {
        const INSTRUCTION_NUMBER: u8 = 5;
        const MIN_ACCOUNTS: usize = 6;
    }

    pub struct DepositInstruction;
    impl DrvInstruction for DepositInstruction {
        const INSTRUCTION_NUMBER: u8 = 7;
        const MIN_ACCOUNTS: usize = 9;
    }

    pub struct WithdrawInstruction;
    impl DrvInstruction for WithdrawInstruction {
        const INSTRUCTION_NUMBER: u8 = 8;
        const MIN_ACCOUNTS: usize = 11;
    }

    pub struct NewInstrumentInstruction;
    impl DrvInstruction for NewInstrumentInstruction {
        const INSTRUCTION_NUMBER: u8 = 9;
        const MIN_ACCOUNTS: usize = 23;
    }

    pub struct UpgradeToPerpInstruction;
    impl DrvInstruction for UpgradeToPerpInstruction {
        const INSTRUCTION_NUMBER: u8 = 10;
        const MIN_ACCOUNTS: usize = 21;
    }

    pub struct PerpDepositInstruction;
    impl DrvInstruction for PerpDepositInstruction {
        const INSTRUCTION_NUMBER: u8 = 11;
        const MIN_ACCOUNTS: usize = 19;
    }

    pub struct NewSpotOrderInstruction;
    impl DrvInstruction for NewSpotOrderInstruction {
        const INSTRUCTION_NUMBER: u8 = 12;
        const MIN_ACCOUNTS: usize = 18;
    }

    pub struct SpotOrderCancelInstruction;
    impl DrvInstruction for SpotOrderCancelInstruction {
        const INSTRUCTION_NUMBER: u8 = 13;
        const MIN_ACCOUNTS: usize = 14;
    }

    pub struct SpotLpInstruction;
    impl DrvInstruction for SpotLpInstruction {
        const INSTRUCTION_NUMBER: u8 = 14;
        const MIN_ACCOUNTS: usize = 5;
    }

    pub struct SpotMassCancelInstruction;
    impl DrvInstruction for SpotMassCancelInstruction {
        const INSTRUCTION_NUMBER: u8 = 15;
        const MIN_ACCOUNTS: usize = 14;
    }

    pub struct NextVotingInstruction;
    impl DrvInstruction for NextVotingInstruction {
        const INSTRUCTION_NUMBER: u8 = 16;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct NewPerpOrderInstruction;
    impl DrvInstruction for NewPerpOrderInstruction {
        const INSTRUCTION_NUMBER: u8 = 19;
        const MIN_ACCOUNTS: usize = 21;
    }

    pub struct DividendsAllocationInstruction;
    impl DrvInstruction for DividendsAllocationInstruction {
        const INSTRUCTION_NUMBER: u8 = 25;
        const MIN_ACCOUNTS: usize = 4;
    }

    pub struct SwapInstruction;
    impl DrvInstruction for SwapInstruction {
        const INSTRUCTION_NUMBER: u8 = 26;
        const MIN_ACCOUNTS: usize = 27;
    }

    pub struct AirdropInstruction;
    impl DrvInstruction for AirdropInstruction {
        const INSTRUCTION_NUMBER: u8 = 27;
        const MIN_ACCOUNTS: usize = 12;
    }

    pub struct DividendsClaimInstruction;
    impl DrvInstruction for DividendsClaimInstruction {
        const INSTRUCTION_NUMBER: u8 = 28;
        const MIN_ACCOUNTS: usize = 6;
    }

    pub struct PerpOrderCancelInstruction;
    impl DrvInstruction for PerpOrderCancelInstruction {
        const INSTRUCTION_NUMBER: u8 = 30;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct VotingInstruction;
    impl DrvInstruction for VotingInstruction {
        const INSTRUCTION_NUMBER: u8 = 32;
        const MIN_ACCOUNTS: usize = 6;
    }

    pub struct SpotQuotesReplaceInstruction;
    impl DrvInstruction for SpotQuotesReplaceInstruction {
        const INSTRUCTION_NUMBER: u8 = 34;
        const MIN_ACCOUNTS: usize = 18;
    }

    pub struct PerpMassCancelInstruction;
    impl DrvInstruction for PerpMassCancelInstruction {
        const INSTRUCTION_NUMBER: u8 = 36;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct PerpChangeLeverageInstruction;
    impl DrvInstruction for PerpChangeLeverageInstruction {
        const INSTRUCTION_NUMBER: u8 = 37;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct FeesWithdrawInstruction;
    impl DrvInstruction for FeesWithdrawInstruction {
        const INSTRUCTION_NUMBER: u8 = 39;
        const MIN_ACCOUNTS: usize = 6;
    }

    pub struct SetInstrReadyForPerpUpgradeInstruction;
    impl DrvInstruction for SetInstrReadyForPerpUpgradeInstruction {
        const INSTRUCTION_NUMBER: u8 = 41;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct PerpQuotesReplaceInstruction;
    impl DrvInstruction for PerpQuotesReplaceInstruction {
        const INSTRUCTION_NUMBER: u8 = 42;
        const MIN_ACCOUNTS: usize = 21;
    }

    pub struct MoveSpotAvailFundsInstruction;
    impl DrvInstruction for MoveSpotAvailFundsInstruction {
        const INSTRUCTION_NUMBER: u8 = 43;
        const MIN_ACCOUNTS: usize = 6;
    }

    pub struct ChangeRefProgramInstruction;
    impl DrvInstruction for ChangeRefProgramInstruction {
        const INSTRUCTION_NUMBER: u8 = 44;
        const MIN_ACCOUNTS: usize = 2;
    }

    pub struct NewRefLinkInstruction;
    impl DrvInstruction for NewRefLinkInstruction {
        const INSTRUCTION_NUMBER: u8 = 45;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct PerpStatisticsResetInstruction;
    impl DrvInstruction for PerpStatisticsResetInstruction {
        const INSTRUCTION_NUMBER: u8 = 46;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct BuyMarketSeatInstruction;
    impl DrvInstruction for BuyMarketSeatInstruction {
        const INSTRUCTION_NUMBER: u8 = 47;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct SellMarketSeatInstruction;
    impl DrvInstruction for SellMarketSeatInstruction {
        const INSTRUCTION_NUMBER: u8 = 48;
        const MIN_ACCOUNTS: usize = 20;
    }

    pub struct NewPrivateClientInstruction;
    impl DrvInstruction for NewPrivateClientInstruction {
        const INSTRUCTION_NUMBER: u8 = 49;
        const MIN_ACCOUNTS: usize = 5;
    }

    pub struct TerminatePrivateModeInstruction;
    impl DrvInstruction for TerminatePrivateModeInstruction {
        const INSTRUCTION_NUMBER: u8 = 50;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct ChangePointsProgramExpirationInstruction;
    impl DrvInstruction for ChangePointsProgramExpirationInstruction {
        const INSTRUCTION_NUMBER: u8 = 51;
        const MIN_ACCOUNTS: usize = 2;
    }

    pub struct ChangeAirdropAuthority;
    impl DrvInstruction for ChangeAirdropAuthority {
        const INSTRUCTION_NUMBER: u8 = 52;

        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct ChangePrivateModeAuthority;
    impl DrvInstruction for ChangePrivateModeAuthority {
        const INSTRUCTION_NUMBER: u8 = 53;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct SetVariance;
    impl DrvInstruction for SetVariance {
        const INSTRUCTION_NUMBER: u8 = 54;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct VotingReset;
    impl DrvInstruction for VotingReset {
        const INSTRUCTION_NUMBER: u8 = 55;
        const MIN_ACCOUNTS: usize = 3;
    }

    pub struct ChangeDenominator;
    impl DrvInstruction for ChangeDenominator {
        const INSTRUCTION_NUMBER: u8 = 56;
        const MIN_ACCOUNTS: usize = 3;
    }
    pub struct PerpClientsProcessingInstruction;
    impl DrvInstruction for PerpClientsProcessingInstruction {
        const INSTRUCTION_NUMBER: u8 = 57;

        const MIN_ACCOUNTS: usize = 19;
    }

    pub struct SetSeatPurchuasingFee;
    impl DrvInstruction for SetSeatPurchuasingFee {
        const INSTRUCTION_NUMBER: u8 = 58;
        const MIN_ACCOUNTS: usize = 2;
    }
}
