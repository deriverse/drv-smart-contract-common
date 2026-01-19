use crate::new_types::{client::u32, instrument::u32};
use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

pub mod log_type {
    pub const DEPOSIT: u8 = 1;
    pub const WITHDRAW: u8 = 2;
    pub const PERP_DEPOSIT: u8 = 3;
    pub const PERP_WITHDRAW: u8 = 4;
    pub const FEES_DEPOSIT: u8 = 5;
    pub const FEES_WITHDRAW: u8 = 6;
    pub const SPOT_LP_TRADE: u8 = 7;
    pub const EARNINGS: u8 = 8;
    pub const DRVS_AIRDROP: u8 = 9;
    pub const SPOT_PLACE_ORDER: u8 = 10;
    pub const SPOT_FILL_ORDER: u8 = 11;
    pub const SPOT_NEW_ORDER: u8 = 12;
    pub const SPOT_ORDER_CANCEL: u8 = 13;
    pub const SPOT_ORDER_REVOKE: u8 = 14;
    pub const SPOT_FEES: u8 = 15;
    pub const SPOT_PLACE_MASS_CANCEL: u8 = 16;
    pub const SPOT_MASS_CANCEL: u8 = 17;
    pub const PERP_PLACE_ORDER: u8 = 18;
    pub const PERP_FILL_ORDER: u8 = 19;
    pub const PERP_NEW_ORDER: u8 = 20;
    pub const PERP_ORDER_CANCEL: u8 = 21;
    pub const PERP_ORDER_REVOKE: u8 = 22;
    pub const PERP_FEES: u8 = 23;
    pub const PERP_FUNDING: u8 = 24;
    pub const PERP_PLACE_MASS_CANCEL: u8 = 25;
    pub const PERP_MASS_CANCEL: u8 = 26;
    pub const PERP_SOC_LOSS: u8 = 27;
    pub const PERP_CHANGE_LEVERAGE: u8 = 28;
    pub const BUY_MARKET_SEAT: u8 = 29;
    pub const SELL_MARKET_SEAT: u8 = 30;
    pub const SWAP_ORDER: u8 = 31;
    pub const MOVE_SPOT: u8 = 32;
    pub const NEW_PRIVATE_CLIENT: u8 = 33;
    pub const CHANGED_POINTS: u8 = 34;
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpChangeLeverageReport {
    pub tag: u8,
    pub leverage: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct DrvsAirdropReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub amount: i64,
    pub time: u32,
    pub padding_u32: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct EarningsReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct DepositReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct FeesDepositReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct FeesWithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpDepositReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct BuyMarketSeatReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub amount: i64,
    pub seat_price: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SellMarketSeatReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub seat_price: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct WithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmWithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpWithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotlpTradeReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub time: u32,
    pub instr_id: u32,
    pub order_id: i64,
    pub qty: i64,
    pub tokens: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpFillOrderReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub price: i64,
    pub rebates: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotFillOrderReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub price: i64,
    pub rebates: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpPlaceOrderReport {
    pub tag: u8,
    pub ioc: u8,
    pub side: u8,
    pub order_type: u8,
    pub client_id: u32,
    pub order_id: i64,
    pub perps: i64,
    pub price: i64,
    pub instr_id: u32,
    pub leverage: u32,
    pub time: u32,
    pub padding_u32: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotPlaceOrderReport {
    pub tag: u8,
    pub ioc: u8,
    pub side: u8,
    pub order_type: u8,
    pub client_id: u32,
    pub order_id: i64,
    pub qty: i64,
    pub price: i64,
    pub instr_id: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, Debug)]
pub struct SwapOrderReport {
    pub tag: u8,
    pub side: u8,
    pub order_type: u8,
    pub padding_u8: u8,
    pub padding_u32: u32,
    pub order_id: i64,
    pub qty: i64,
    pub price: i64,
    pub time: u32,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpPlaceMassCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotPlaceMassCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpMassCancelReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotMassCancelReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpFeesReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub ref_client_id: u32,
    pub fees: i64,
    pub ref_payment: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotFeesReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub ref_client_id: u32,
    pub fees: i64,
    pub ref_payment: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpFundingReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub funding: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpSocLossReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub soc_loss: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpNewOrderReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub perps: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotNewOrderReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpOrderCancelReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotOrderCancelReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpOrderRevokeReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotOrderRevokeReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct MoveSpotAvailFundsReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub instr_id: u32,
    pub time: u32,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct ChangePointsRecord {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: u32,
    pub points: u32,
    pub time: u32,
}
