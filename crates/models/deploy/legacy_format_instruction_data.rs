#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewOperatorData {
    pub tag: u8, // 1
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub u32: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewRootAccountData {
    pub tag: u8, // 2
    pub private_mode: u8,
    pub padding_u16: u16,
    pub u32: u32,
    pub lut_slot: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpChangeLeverageData {
    pub tag: u8, //37
    pub leverage: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpStatisticsResetData {
    pub tag: u8, //46
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SpotMassCancelData {
    pub tag: u8, //15
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct FeesDepositData {
    pub tag: u8, //5
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpDepositData {
    pub tag: u8, //11
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MoveSpotAvailFundsData {
    pub tag: u8, //43
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WithdrawData {
    pub tag: u8, //8
    padding_u8: u8,
    padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SwapData {
    pub tag: u8, //26
    pub side: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub price: i64,
    pub amount: i64,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SetInstrOracleFeedData {
    pub tag: u8, // 40
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewTokenData {
    pub tag: u8, // 4
    pub crncy: u8,
    pub need_initialization: u8,
    pub padding_u8: u8,
    pub padding_u32: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpMassCancelData {
    pub tag: u8, //36
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PerpForcedCloseData {
    pub tag: u8, //38
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
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

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BuyMarketSeatData {
    pub tag: u8, //47
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SellMarketSeatData {
    pub tag: u8, //48
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NewPrivateClient {
    pub tag: u8, // 49
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub wallet: Pubkey,
    pub expiration_time: u32,
}
