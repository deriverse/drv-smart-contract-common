use crate::{
    log::log_type::*,
    new_types::{client::ClientId, instrument::InstrId},
};
use bytemuck::{Pod, Zeroable};

pub mod log_type {
    // Client logs
    pub const DEPOSIT: u8 = 1;
    pub const WITHDRAW: u8 = 2;
    pub const FEES_DEPOSIT: u8 = 5;
    pub const FEES_WITHDRAW: u8 = 6;
    pub const EARNINGS: u8 = 8;
    pub const DRVS_AIRDROP: u8 = 9;
    pub const VM_INIT_ACTIVATE: u8 = 36;
    pub const VM_INIT_ACTIVATE_CANCEL: u8 = 37;
    pub const VM_FINALIZE_ACTIVATE: u8 = 38;
    pub const VM_INIT_DEACTIVATE: u8 = 39;
    pub const VM_INIT_DEACTIVATE_CANCEL: u8 = 40;
    pub const VM_FINALIZE_DEACTIVATE: u8 = 41;
    pub const VM_CHANGE_LIST: u8 = 42;
    pub const VM_INIT_WITHDRAW: u8 = 43;
    pub const VM_INIT_WITHDRAW_CANCEL: u8 = 44;
    pub const VM_INIT_WITHDRAW_FINALIZE: u8 = 45;
    pub const CHANGED_POINTS: u8 = 34;
    pub const MOVE_SPOT: u8 = 32;
    pub const VM_DIRECT_WITHDRAW: u8 = 47;
    pub const KAMINO_CHANGE_POSITION: u8 = 48;

    // Instrument logs
    pub const PERP_DEPOSIT: u8 = 3;
    pub const PERP_WITHDRAW: u8 = 4;
    pub const SPOT_LP_TRADE: u8 = 7;
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
    pub const SWAP_FEES: u8 = 35;
    pub const PERP_LOSS_COVERAGE: u8 = 46;
}

pub trait Log {
    const TAG: u8;
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpLossCoverageReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub loss_coverage: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpChangeLeverageReport {
    pub tag: u8,
    pub leverage: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct DrvsAirdropReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub amount: i64,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct EarningsReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct DepositReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct FeesDepositReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct FeesWithdrawReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpDepositReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub amount: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct BuyMarketSeatReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub amount: i64,
    pub seat_price: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SellMarketSeatReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub seat_price: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct WithdrawReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpWithdrawReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub amount: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotlpTradeReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub order_id: i64,
    pub qty: i64,
    pub tokens: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpFillOrderReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub price: i64,
    pub rebates: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotFillOrderReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub price: i64,
    pub rebates: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpPlaceOrderReport {
    pub tag: u8,
    pub ioc: u8,
    pub side: u8,
    pub order_type: u8,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub order_id: i64,
    pub perps: i64,
    pub price: i64,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub leverage: u32,
    pub time: u32,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotPlaceOrderReport {
    pub tag: u8,
    pub ioc: u8,
    pub side: u8,
    pub order_type: u8,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub order_id: i64,
    pub qty: i64,
    pub price: i64,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, Debug, shank::ShankType)]
pub struct PlaceSwapOrderReport {
    pub tag: u8,
    pub side: u8,
    pub order_type: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u32: u32,
    pub order_id: i64,
    pub qty: i64,
    pub price: i64,
    pub time: u32,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub swap_ref_rate: f64,
    pub seq_no: i64,
}

// impl std::fmt::Display for SwapOrderReport {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let side: OrderSide = ;
//         let order_type: OrderSide = self.order_type.try_into();
//         write!(f, "SwapOrderReport {{\n")?;
//         write!(f, "  tag: {},\n", self.tag)?;
//         write!(
//             f,
//             "  side: {},\n",
//             side.map(|side| format!("{:?}", side))
//                 .unwrap_or_else(|err| format!("Error while construction
// OrderSide {:?}", err))         )?;
//         write!(
//             f,
//             "  order_type: {},\n",
//             order_type
//                 .map(|order_type| format!("{:?}", order_type))
//                 .unwrap_or_else(|err| format!("Error while construction
// OrderType {:?}", err))         )?;
//         write!(f, "  order_id: {},\n", self.order_id)?;
//         write!(f, "  qty: {},\n", self.qty)?;
//         write!(f, "  price: {},\n", self.price)?;
//         write!(f, "  time: {},\n", self.time)?;
//         write!(f, "  instr_id: {:?},\n", self.instr_id)?;
//         write!(f, "}}")
//     }
// }

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpPlaceMassCancelReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotPlaceMassCancelReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpMassCancelReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotMassCancelReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpFeesReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub ref_client_id: ClientId,
    pub fees: i64,
    pub ref_payment: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotFeesReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub ref_client_id: ClientId,
    pub fees: i64,
    pub ref_payment: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpFundingReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub funding: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpSocLossReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub soc_loss: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpNewOrderReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub perps: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotNewOrderReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub qty: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpOrderCancelReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotOrderCancelReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct PerpOrderRevokeReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct SpotOrderRevokeReport {
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct MoveSpotAvailFundsReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct ChangePointsReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub points: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitActivateReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitActivateCancelReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmFinalizeActivateReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitDeactivateReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitDeactivateCancelReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmFinalizeDeactivateReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmChangeListReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitWithdrawReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitWithdrawCancelReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmInitWithdrawFinalizeReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct VmDirectWithdrawReport {
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub withdrawal_record_id: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, shank::ShankType)]
pub struct KaminoChangePositionReport {
    pub tag: u8,
    pub assets_is_collateral: u8,
    pub withdraw_all: u8,
    pub repay_all: u8,
    #[padding]
    pub padding_u32: u32,
    pub seq_no: u32,
    #[idl_type(u32)]
    pub client_id: ClientId,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub time: u32,
    pub borrow_delta: i64,
    pub collateral_delta: i64,
    pub custom_id: i64,
}

impl Log for DepositReport {
    const TAG: u8 = DEPOSIT;
}
impl Log for WithdrawReport {
    const TAG: u8 = WITHDRAW;
}
impl Log for PerpDepositReport {
    const TAG: u8 = PERP_DEPOSIT;
}
impl Log for PerpWithdrawReport {
    const TAG: u8 = PERP_WITHDRAW;
}
impl Log for FeesDepositReport {
    const TAG: u8 = FEES_DEPOSIT;
}
impl Log for FeesWithdrawReport {
    const TAG: u8 = FEES_WITHDRAW;
}
impl Log for SpotlpTradeReport {
    const TAG: u8 = SPOT_LP_TRADE;
}
impl Log for EarningsReport {
    const TAG: u8 = EARNINGS;
}
impl Log for DrvsAirdropReport {
    const TAG: u8 = DRVS_AIRDROP;
}
impl Log for SpotPlaceOrderReport {
    const TAG: u8 = SPOT_PLACE_ORDER;
}
impl Log for SpotFillOrderReport {
    const TAG: u8 = SPOT_FILL_ORDER;
}
impl Log for SpotNewOrderReport {
    const TAG: u8 = SPOT_NEW_ORDER;
}
impl Log for SpotOrderCancelReport {
    const TAG: u8 = SPOT_ORDER_CANCEL;
}
impl Log for SpotOrderRevokeReport {
    const TAG: u8 = SPOT_ORDER_REVOKE;
}
impl Log for SpotFeesReport {
    const TAG: u8 = SPOT_FEES;
}
impl Log for SpotPlaceMassCancelReport {
    const TAG: u8 = SPOT_PLACE_MASS_CANCEL;
}
impl Log for SpotMassCancelReport {
    const TAG: u8 = SPOT_MASS_CANCEL;
}
impl Log for PerpPlaceOrderReport {
    const TAG: u8 = PERP_PLACE_ORDER;
}
impl Log for PerpFillOrderReport {
    const TAG: u8 = PERP_FILL_ORDER;
}
impl Log for PerpNewOrderReport {
    const TAG: u8 = PERP_NEW_ORDER;
}
impl Log for PerpOrderCancelReport {
    const TAG: u8 = PERP_ORDER_CANCEL;
}
impl Log for PerpOrderRevokeReport {
    const TAG: u8 = PERP_ORDER_REVOKE;
}
impl Log for PerpFeesReport {
    const TAG: u8 = PERP_FEES;
}
impl Log for PerpFundingReport {
    const TAG: u8 = PERP_FUNDING;
}
impl Log for PerpPlaceMassCancelReport {
    const TAG: u8 = PERP_PLACE_MASS_CANCEL;
}
impl Log for PerpMassCancelReport {
    const TAG: u8 = PERP_MASS_CANCEL;
}
impl Log for PerpSocLossReport {
    const TAG: u8 = PERP_SOC_LOSS;
}
impl Log for PerpChangeLeverageReport {
    const TAG: u8 = PERP_CHANGE_LEVERAGE;
}
impl Log for BuyMarketSeatReport {
    const TAG: u8 = BUY_MARKET_SEAT;
}
impl Log for SellMarketSeatReport {
    const TAG: u8 = SELL_MARKET_SEAT;
}
impl Log for PlaceSwapOrderReport {
    const TAG: u8 = SWAP_ORDER;
}
impl Log for MoveSpotAvailFundsReport {
    const TAG: u8 = MOVE_SPOT;
}
impl Log for ChangePointsReport {
    const TAG: u8 = CHANGED_POINTS;
}
impl Log for VmInitActivateReport {
    const TAG: u8 = VM_INIT_ACTIVATE;
}
impl Log for VmInitActivateCancelReport {
    const TAG: u8 = VM_INIT_ACTIVATE_CANCEL;
}
impl Log for VmFinalizeActivateReport {
    const TAG: u8 = VM_FINALIZE_ACTIVATE;
}
impl Log for VmInitDeactivateReport {
    const TAG: u8 = VM_INIT_DEACTIVATE;
}
impl Log for VmInitDeactivateCancelReport {
    const TAG: u8 = VM_INIT_DEACTIVATE_CANCEL;
}
impl Log for VmFinalizeDeactivateReport {
    const TAG: u8 = VM_FINALIZE_DEACTIVATE;
}
impl Log for VmChangeListReport {
    const TAG: u8 = VM_CHANGE_LIST;
}
impl Log for VmInitWithdrawReport {
    const TAG: u8 = VM_INIT_WITHDRAW;
}
impl Log for VmInitWithdrawCancelReport {
    const TAG: u8 = VM_INIT_WITHDRAW_CANCEL;
}
impl Log for VmInitWithdrawFinalizeReport {
    const TAG: u8 = VM_INIT_WITHDRAW_FINALIZE;
}
impl Log for VmDirectWithdrawReport {
    const TAG: u8 = VM_DIRECT_WITHDRAW;
}
impl Log for PerpLossCoverageReport {
    const TAG: u8 = PERP_LOSS_COVERAGE;
}
impl Log for KaminoChangePositionReport {
    const TAG: u8 = KAMINO_CHANGE_POSITION;
}
