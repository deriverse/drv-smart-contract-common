use crate::program::{
    constants::{
        MAX_ORDER_ID, MAX_PERP_LEVERAGE, MAX_PRICE, MAX_REF_DISCOUNT, MAX_REF_RATIO,
        SPOT_MAX_AMOUNT,
    },
    error::DeriverseError::{
        InvalidDataFormat, InvalidInstrId, InvalidLeverage, InvalidOrderId, InvalidPrice,
        InvalidQuantity, InvalidRefProgramParameters, InvalidTokenId,
    },
};
use bytemuck::{Pod, Zeroable};
use solana_program::program_error::ProgramError;

use super::constants::MIN_INIT_PRICE;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewOperatorData {
    pub tag: u8, // 1
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub version: u32,
}

impl NewOperatorData {
    pub fn new(instruction_data: &[u8]) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<NewOperatorData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        Ok(data)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewRootAccountData {
    pub tag: u8, // 2
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub version: u32,
    pub lut_slot: u32,
}

impl NewRootAccountData {
    pub fn new(instruction_data: &[u8]) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<NewRootAccountData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        Ok(data)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewSpotOrderData {
    pub tag: u8, //12
    pub ioc: u8,
    pub order_type: u8,
    pub side: u8,
    pub instr_id: u32,
    pub price: i64,
    pub amount: i64,
}

impl NewSpotOrderData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<NewSpotOrderData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if data.order_type == 0 && !(1..MAX_PRICE).contains(&data.price) {
            Err(InvalidPrice.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewPerpOrderData {
    pub tag: u8, //19
    pub ioc: u8,
    pub leverage: u8,
    pub order_type: u8,
    pub side: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub instr_id: u32,
    pub price: i64,
    pub amount: i64,
}

impl NewPerpOrderData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<NewPerpOrderData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if data.order_type == 0 && !(1..MAX_PRICE).contains(&data.price) {
            Err(InvalidPrice.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else if data.leverage > MAX_PERP_LEVERAGE {
            Err(InvalidLeverage.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpChangeLeverageData {
    pub tag: u8, //37
    pub leverage: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

impl PerpChangeLeverageData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpChangeLeverageData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if data.leverage == 0 || data.leverage > MAX_PERP_LEVERAGE {
            Err(InvalidLeverage.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpStatisticsResetData {
    pub tag: u8, //46
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

impl PerpStatisticsResetData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpStatisticsResetData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SpotOrderCancelData {
    pub tag: u8, //13
    pub side: u8,
    padding_u16: u16,
    pub instr_id: u32,
    pub order_id: i64,
}

impl SpotOrderCancelData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SpotOrderCancelData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(0..MAX_ORDER_ID).contains(&data.order_id) {
            Err(InvalidOrderId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SpotMassCancelData {
    pub tag: u8, //15
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
}

impl SpotMassCancelData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SpotMassCancelData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SpotLpData {
    pub tag: u8, // 14
    pub side: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

impl SpotLpData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SpotLpData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewInstrumentData {
    pub tag: u8, //9
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub crncy_token_id: u32,
    pub lut_slot: u32,
    pub price: i64,
}

impl NewInstrumentData {
    pub fn new(instruction_data: &[u8], tokens_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<NewInstrumentData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.crncy_token_id >= tokens_count {
            Err(InvalidTokenId.into())
        } else if !(MIN_INIT_PRICE..MAX_PRICE).contains(&data.price) {
            Err(InvalidPrice.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct DepositData {
    pub tag: u8, //7
    pub competition_id: u8,
    pub deposit_all: u8, // bool
    pub padding_u8: u8,
    pub token_id: u32,
    pub amount: i64,
    pub lut_slot: u32,
    pub ref_id: u32,
}

impl DepositData {
    pub fn new(instruction_data: &[u8], tokens_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<DepositData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.token_id >= tokens_count {
            Err(InvalidTokenId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FeesDepositData {
    pub tag: u8, //5
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

impl FeesDepositData {
    pub fn new(instruction_data: &[u8], tokens_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<FeesDepositData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.token_id >= tokens_count {
            Err(InvalidTokenId.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FeesWithdrawData {
    pub tag: u8, //39
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

impl FeesWithdrawData {
    pub fn new(instruction_data: &[u8], tokens_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<FeesWithdrawData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.token_id >= tokens_count {
            Err(InvalidTokenId.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpDepositData {
    pub tag: u8, //11
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

impl PerpDepositData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpDepositData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MoveSpotAvailFundsData {
    pub tag: u8, //43
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

impl MoveSpotAvailFundsData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<MoveSpotAvailFundsData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpWithdrawData {
    pub tag: u8, //3
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

impl PerpWithdrawData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpWithdrawData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(0..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WithdrawData {
    pub tag: u8, //8
    padding_u8: u8,
    padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

impl WithdrawData {
    pub fn new(instruction_data: &[u8], tokens_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<WithdrawData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.token_id >= tokens_count {
            Err(InvalidTokenId.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SwapData {
    pub tag: u8, //26
    pub side: u8,
    padding_u16: u16,
    pub instr_id: u32,
    pub price: i64,
    pub amount: i64,
}

impl SwapData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SwapData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(0..MAX_PRICE).contains(&data.price) {
            Err(InvalidPrice.into())
        } else if !(1..SPOT_MAX_AMOUNT).contains(&data.amount) {
            Err(InvalidQuantity.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SpotQuotesReplaceData {
    pub tag: u8, //34
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
    pub new_bid_price: i64,
    pub new_bid_qty: i64,
    pub old_bid_order_id: i64,
    pub new_ask_price: i64,
    pub new_ask_qty: i64,
    pub old_ask_order_id: i64,
}

impl SpotQuotesReplaceData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SpotQuotesReplaceData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(0..MAX_PRICE).contains(&data.new_bid_price)
            || !(0..MAX_PRICE).contains(&data.new_ask_price)
            || data.new_bid_price >= data.new_ask_price
        {
            Err(InvalidPrice.into())
        } else if !(0..SPOT_MAX_AMOUNT).contains(&data.new_bid_qty)
            || !(0..SPOT_MAX_AMOUNT).contains(&data.new_ask_qty)
        {
            Err(InvalidQuantity.into())
        } else if !(0..MAX_ORDER_ID).contains(&data.old_bid_order_id)
            || !(0..MAX_ORDER_ID).contains(&data.old_ask_order_id)
        {
            Err(InvalidOrderId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpQuotesReplaceData {
    pub tag: u8, //42
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
    pub new_bid_price: i64,
    pub new_bid_qty: i64,
    pub old_bid_order_id: i64,
    pub new_ask_price: i64,
    pub new_ask_qty: i64,
    pub old_ask_order_id: i64,
}

impl PerpQuotesReplaceData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpQuotesReplaceData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(0..MAX_PRICE).contains(&data.new_bid_price)
            || !(0..MAX_PRICE).contains(&data.new_ask_price)
            || data.new_bid_price >= data.new_ask_price
        {
            Err(InvalidPrice.into())
        } else if !(0..SPOT_MAX_AMOUNT).contains(&data.new_bid_qty)
            || !(0..SPOT_MAX_AMOUNT).contains(&data.new_ask_qty)
        {
            Err(InvalidQuantity.into())
        } else if !(0..MAX_ORDER_ID).contains(&data.old_bid_order_id)
            || !(0..MAX_ORDER_ID).contains(&data.old_ask_order_id)
        {
            Err(InvalidOrderId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct VotingData {
    pub tag: u8, //32
    pub choice: u8,
    padding_u16: u16,
    pub voting_counter: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AirdropData {
    pub tag: u8, // 27
    padding_u8: u8,
    padding_u16: u16,
    padding_u32: u32,
    pub ratio: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UpgradeToPerpData {
    pub tag: u8, // 10
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

impl UpgradeToPerpData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<UpgradeToPerpData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count || (data.instr_id.0 & 1) != 0 {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SetInstrOracleFeedData {
    pub tag: u8, // 40
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub variance: f64,
}

impl SetInstrOracleFeedData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SetInstrOracleFeedData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count || (*data.instr_id & 1) != 0 {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SetInstrReadyForPerpUpgradeData {
    pub tag: u8, // 41
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub variance: f64,
}

impl SetInstrReadyForPerpUpgradeData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<SetInstrReadyForPerpUpgradeData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count || (*data.instr_id & 1) != 0 {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewTokenData {
    pub tag: u8, // 4
    pub crncy: u8,
    pub need_initialization: u8,
    pub padding_u8: u8,
    pub padding_u32: u32,
}

impl NewTokenData {
    pub fn new(instruction_data: &[u8]) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<NewTokenData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        Ok(data)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpOrderCancelData {
    pub tag: u8, //30
    pub side: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub order_id: i64,
}

impl PerpOrderCancelData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpOrderCancelData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else if !(0..MAX_ORDER_ID).contains(&data.order_id) {
            Err(InvalidOrderId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpMassCancelData {
    pub tag: u8, //36
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

impl PerpMassCancelData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpMassCancelData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpForcedCloseData {
    pub tag: u8, //38
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
}

impl PerpForcedCloseData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<PerpForcedCloseData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChangeRefProgramData {
    pub tag: u8, //44
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub ref_program_duration: u32,
    pub ref_link_duration: u32,
    pub ref_discount: f64,
    pub ref_ratio: f64,
}

impl ChangeRefProgramData {
    pub fn new(instruction_data: &[u8]) -> Result<&Self, ProgramError> {
        let data = bytemuck::try_from_bytes::<ChangeRefProgramData>(instruction_data)
            .map_err(|_| InvalidDataFormat)?;
        if data.ref_discount > MAX_REF_DISCOUNT
            || data.ref_discount < 0.0
            || data.ref_ratio > MAX_REF_RATIO
            || data.ref_ratio < 0.0
        {
            Err(InvalidRefProgramParameters.into())
        } else {
            Ok(data)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BuyMarketSeatData {
    pub tag: u8, //47
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

impl BuyMarketSeatData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data =
            bytemuck::try_from_bytes::<Self>(instruction_data).map_err(|_| InvalidDataFormat)?;

        if !(0..SPOT_MAX_AMOUNT).contains(&data.amount) {
            return Err(InvalidQuantity.into());
        }
        if data.instr_id >= instr_count {
            return Err(InvalidInstrId.into());
        }

        Ok(data)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SellMarketSeatData {
    pub tag: u8, //48
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

impl SellMarketSeatData {
    pub fn new(instruction_data: &[u8], instr_count: u32) -> Result<&Self, ProgramError> {
        let data =
            bytemuck::try_from_bytes::<Self>(instruction_data).map_err(|_| InvalidDataFormat)?;
        if data.instr_id >= instr_count {
            Err(InvalidInstrId.into())
        } else {
            Ok(data)
        }
    }
}
