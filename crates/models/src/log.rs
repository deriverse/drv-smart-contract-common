use crate::new_types::{client::ClientId, instrument::InstrId};
use bytemuck::{Pod, Zeroable};

pub mod log_type {
    pub const DEPOSIT: u8 = 1; // client
    pub const WITHDRAW: u8 = 2; // client
    pub const FEES_DEPOSIT: u8 = 5; // client
    pub const FEES_WITHDRAW: u8 = 6; // client
    pub const EARNINGS: u8 = 8; // client
    pub const DRVS_AIRDROP: u8 = 9; // client
    pub const VM_INIT_ACTIVATE: u8 = 36; // client
    pub const VM_INIT_ACTIVATE_CANCEL: u8 = 37; // client
    pub const VM_FINALIZE_ACTIVATE: u8 = 38; // client
    pub const VM_INIT_DEACTIVATE: u8 = 39; // client
    pub const VM_INIT_DEACTIVATE_CANCEL: u8 = 40; // client
    pub const VM_FINALIZE_DEACTIVATE: u8 = 41; // client
    pub const VM_CHANGE_LIST: u8 = 42; // client
    pub const VM_INIT_WITHDRAW: u8 = 43; // client
    pub const VM_INIT_WITHDRAW_CANCEL: u8 = 44; // client
    pub const VM_INIT_WITHDRAW_FINALIZE: u8 = 45; // client
    pub const CHANGED_POINTS: u8 = 34; // client
    pub const MOVE_SPOT: u8 = 32; // client

    pub const PERP_DEPOSIT: u8 = 3; // instrument
    pub const PERP_WITHDRAW: u8 = 4; // instrument
    pub const SPOT_LP_TRADE: u8 = 7; // instrument
    pub const SPOT_PLACE_ORDER: u8 = 10; // instrument
    pub const SPOT_FILL_ORDER: u8 = 11; // instrument
    pub const SPOT_NEW_ORDER: u8 = 12; // instrument
    pub const SPOT_ORDER_CANCEL: u8 = 13; // instrument
    pub const SPOT_ORDER_REVOKE: u8 = 14; // instrument
    pub const SPOT_FEES: u8 = 15; // instrument
    pub const SPOT_PLACE_MASS_CANCEL: u8 = 16; // instrument
    pub const SPOT_MASS_CANCEL: u8 = 17; // instrument
    pub const PERP_PLACE_ORDER: u8 = 18; // instrument
    pub const PERP_FILL_ORDER: u8 = 19; // instrument
    pub const PERP_NEW_ORDER: u8 = 20; // instrument
    pub const PERP_ORDER_CANCEL: u8 = 21; // instrument
    pub const PERP_ORDER_REVOKE: u8 = 22; // instrument
    pub const PERP_FEES: u8 = 23; // instrument
    pub const PERP_FUNDING: u8 = 24; // instrument
    pub const PERP_PLACE_MASS_CANCEL: u8 = 25; // instrument
    pub const PERP_MASS_CANCEL: u8 = 26; // instrument
    pub const PERP_SOC_LOSS: u8 = 27; // instrument
    pub const PERP_CHANGE_LEVERAGE: u8 = 28; // instrument
    pub const BUY_MARKET_SEAT: u8 = 29; // instrument
    pub const SELL_MARKET_SEAT: u8 = 30; // instrument
    pub const SWAP_ORDER: u8 = 31; // instrument
    pub const NEW_PRIVATE_CLIENT: u8 = 33; // unused
    pub const SWAP_FEES: u8 = 35; // instrument
    pub const PERP_LOSS_COVERAGE: u8 = 46; // instrument
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpLossCoverageReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub loss_coverage: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpChangeLeverageReport {
    pub tag: u8,
    pub leverage: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct DrvsAirdropReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub amount: i64,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct EarningsReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
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
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct FeesDepositReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
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
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
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
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub amount: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct BuyMarketSeatReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub amount: i64,
    pub seat_price: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SellMarketSeatReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub seat_price: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct WithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpWithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub amount: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotlpTradeReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub instr_id: InstrId,
    pub order_id: i64,
    pub qty: i64,
    pub tokens: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpFillOrderReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub price: i64,
    pub rebates: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotFillOrderReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub price: i64,
    pub rebates: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpPlaceOrderReport {
    pub tag: u8,
    pub ioc: u8,
    pub side: u8,
    pub order_type: u8,
    pub client_id: ClientId,
    pub order_id: i64,
    pub perps: i64,
    pub price: i64,
    pub instr_id: InstrId,
    pub leverage: u32,
    pub time: u32,
    pub padding_u32: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotPlaceOrderReport {
    pub tag: u8,
    pub ioc: u8,
    pub side: u8,
    pub order_type: u8,
    pub client_id: ClientId,
    pub order_id: i64,
    pub qty: i64,
    pub price: i64,
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default, Debug)]
pub struct PlaceSwapOrderReport {
    pub tag: u8,
    pub side: u8,
    pub order_type: u8,
    pub padding_u8: u8,
    pub padding_u32: u32,
    pub order_id: i64,
    pub qty: i64,
    pub price: i64,
    pub time: u32,
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
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpPlaceMassCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotPlaceMassCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub seq_no: i64,
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
    pub seq_no: i64,
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
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpFeesReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub ref_client_id: ClientId,
    pub fees: i64,
    pub ref_payment: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotFeesReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub ref_client_id: ClientId,
    pub fees: i64,
    pub ref_payment: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpFundingReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub funding: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpSocLossReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub soc_loss: i64,
    pub seq_no: i64,
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
    pub seq_no: i64,
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
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpOrderCancelReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotOrderCancelReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct PerpOrderRevokeReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub order_id: i64,
    pub perps: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SpotOrderRevokeReport {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub order_id: i64,
    pub qty: i64,
    pub crncy: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct MoveSpotAvailFundsReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
    pub instr_id: InstrId,
    pub time: u32,
    pub qty: i64,
    pub crncy: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct ChangePointsReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
    pub points: u32,
    pub time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct SwapRefFeesReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub fees: i64,
    pub seq_no: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitActivateReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitActivateCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmFinalizeActivateReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitDeactivateReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitDeactivateCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmFinalizeDeactivateReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmChangeListReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitWithdrawReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitWithdrawCancelReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub seq_no: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod, Default)]
pub struct VmInitWithdrawFinalizeReport {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub seq_no: u32,
    pub client_id: ClientId,
    pub token_id: u32,
    pub time: u32,
    pub amount: i64,
}
