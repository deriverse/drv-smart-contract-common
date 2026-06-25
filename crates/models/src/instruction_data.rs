use crate::{
    new_types::{instrument::InstrId, version::Version},
    state::{
        masks::instr_mask::InstrInputMask,
        types::{quote_status::QuoteMask, vm_status::VmMask, CappedI64},
    },
};
use bytemuck::{Pod, Zeroable};
use shank::ShankType;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Operator Data
///
/// **Used in:** `new_operator` instruction
///
/// **Tag:** `1`
///
/// ### Fields
/// - `version` - smart contract version
pub struct NewOperatorData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub version: Version,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Root Account Data
///
/// **Used in:** `new_root_account` instruction
///
/// **Tag:** `2`
///
/// ### Fields
/// - `private_mode`: bool - Allow to enable private mode program
/// - `version` - smart contract version
/// - `lut_slot` - LUT creation slot
pub struct NewRootAccountData {
    #[skip]
    pub tag: u8,
    pub private_mode: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub version: Version,
    pub lut_slot: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Spot Order Data
///
/// **Used in:** `new_spot_order` instruction
///
/// **Tag:** `12`
///
/// ### Fields
/// - `ioc`: bool - Use immediate or cancel mode
/// - `order_type`: OrderType - new order type
/// - `order_side`: OrderSide - new order side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `price` - Price for **Limit** order
/// - `amount` - Orders qty in base crncy
/// - `edge_price` - Price used for slippage calculations
pub struct NewSpotOrderData {
    #[skip]
    pub tag: u8,
    pub ioc: u8,
    pub order_type: u8,
    pub side: u8,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub price: i64,
    #[idl_type(i64)]
    pub amount: CappedI64,
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Perp Order Data
///
/// **Used in:** `new_spot_order` instruction
///
/// **Tag:** `19`
///
/// ### Fields
/// - `ioc`: bool - Use immediate or cancel mode
/// - `leverage` - New leverage value, if is 0 change to max possible
/// - `order_type`: OrderType - new order type
/// - `order_side`: OrderSide - new order side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `price` - Price for **Limit** order
/// - `amount` - Orders qty in base crncy
/// - `edge_price` - Price used for slippage calculations
pub struct NewPerpOrderData {
    #[skip]
    pub tag: u8, //19
    pub ioc: u8,
    pub leverage: u8,
    pub order_type: u8,
    pub side: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub price: i64,
    #[idl_type(i64)]
    pub amount: CappedI64,
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Change Leverage Data
///
/// **Used in:** `perp_change_leverage` instruction
///
/// **Tag:** `37`
///
/// ### Fields
/// - `leverage` - New leverage value, if is 0 change to max possible
/// - `instr_id` - Instr pair id
pub struct PerpChangeLeverageData {
    #[skip]
    pub tag: u8,
    pub leverage: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Statistics Reset Data
///
/// **Used in:** `perp_statistic_reset` instruction
///
/// **Tag:** `46`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct PerpStatisticsResetData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Spot Order Cancel Data
///
/// **Used in:** `spot_order_cancel` instruction
///
/// **Tag:** `13`
///
/// ### Fields
/// - `side`: OrderSide - Orders side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `order_id` - Orders id in the system
pub struct SpotOrderCancelData {
    #[skip]
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub order_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Spot Mass Cancel Data
///
/// **Used in:** `spot_mass_cancel` instruction
///
/// **Tag:** `15`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct SpotMassCancelData {
    #[skip]
    pub tag: u8, //15
    #[padding]
    padding_u8: u8,
    #[padding]
    padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Spot LP Data
///
/// **Used in:** `spot_lp` instruction
///
/// **Tag:** `14`
///
/// ### Fields
/// - `side`: OrderSide - Orders side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `amount` - Orders qty in lp tokens
/// - `min_price` - Price used min slippage bound calculations
/// - `max_price` - Price used max slippage bound calculations
pub struct SpotLpData {
    #[skip]
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[idl_type(i64)]
    pub amount: CappedI64,
    pub min_price: i64,
    pub max_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Instrument Data
///
/// **Used in:** `new_instrument` instruction
///
/// **Tag:** `9`
///
/// ### Fields
/// - `crncy_token_id` - Id of token in the system with base crncy flag
/// - `lut_slot` - LUT creation slot
/// - `price` - Base price for an instrument
pub struct NewInstrumentData {
    #[skip]
    pub tag: u8,
    #[idl_type(u8)]
    pub mask: InstrInputMask,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub crncy_token_id: u32,
    pub lut_slot: u32,
    pub price: i64,
    #[idl_type(i64)]
    pub min_qty: CappedI64,
    pub fixed_fee_rate: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Deposit Data
///
/// **Used in:** `deposit` instruction
///
/// **Tag:** `7`
///
/// ### Fields
/// - `competition_id` - Deprecated
/// - `deposit_all`: bool - Flag for deposition all clinents funds
/// - `token_id` - Id of depositing token in the system
/// - `amount` - Amount of tokens to deposit, in case of deposit_all flag, does not count
/// - `lut_slot` - LUT creation slot
/// - `ref_id` - Optional referral id
pub struct DepositData {
    #[skip]
    pub tag: u8,
    pub competition_id: u8,
    pub deposit_all: u8,
    #[padding]
    pub padding_u8: u8,
    pub token_id: u32,
    #[idl_type(i64)]
    pub amount: CappedI64,
    pub lut_slot: u32,
    pub ref_id: u32,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Fees Deposit Data
///
/// **Used in:** `fees_deposit` instruction
///
/// **Tag:** `5`
///
/// ### Fields
/// - `token_id` - Id of token in the system, must be base crncy
/// - `amount` - Amount of tokens client wants to prepay
pub struct FeesDepositData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub token_id: u32,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Fees Withdraw Data
///
/// **Used in:** `fees_withdraw` instruction
///
/// **Tag:** `39`
///
/// ### Fields
/// - `token_id` - Id of token in the system, must be base crncy
/// - `amount` - Amount of tokens client wants to withdraw
pub struct FeesWithdrawData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub token_id: u32,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Deposit Data
///
/// **Used in:** `perp_deposit` instruction
///
/// **Tag:** `11`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `amount` - Amount of tokens client wants to move from spot to perp
pub struct PerpDepositData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Move Spot Available Funds Data
///
/// **Used in:** `move_spot_avail_funds` instruction
///
/// **Tag:** `43`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct MoveSpotAvailFundsData {
    #[skip]
    pub tag: u8, //43
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Withdraw Data
///
/// **Used in:** `perp_withdraw` instruction
///
/// **Tag:** `3`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `amount` - Amount of tokens client wants to move from perp to spot
pub struct PerpWithdrawData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Withdraw Data
///
/// **Used in:** `withdraw` instruction
///
/// **Tag:** `8`
///
/// ### Fields
/// - `token_id` - Id of a token in the system
/// - `amount` - Amount of tokens to withdraw
pub struct WithdrawData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8, // <- bump
    #[padding]
    pub padding_u16: u16,
    pub token_id: u32,
    #[idl_type(i64)]
    pub amount: CappedI64,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Swap Data
///
/// **Used in:** `swap` instruction
///
/// **Tag:** `26`
///
/// ### Fields
/// - `input_crncy`: u8 - Flag if 0 sell `crncy` else sell `asset`
/// - `instr_id` - Instr pair id
/// - `price` - Limit price for a swap
/// - `amount` - Swaps qty in base crncy
/// - `min_amount_out` - Min amount threshold for trade result, 0 by default
pub struct SwapData {
    #[skip]
    pub tag: u8,
    pub input_crncy: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub price: i64,
    #[idl_type(i64)]
    pub amount: CappedI64,
    pub min_amount_out: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Spot Quotes Replace Data
///
/// **Used in:** `spot_quotes_replace` instruction
///
/// **Tag:** `34`
///
/// ### Fields
/// - `mask` - Multiple quotes order manager
/// - `instr_id` - Instr pair id
pub struct SpotQuotesReplaceData {
    #[skip]
    pub tag: u8,
    pub bump: u8,
    pub order_type: u8,
    pub bail_on_order_not_found: u8,
    #[idl_type(u16)]
    pub mask: QuoteMask,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[padding]
    pub padding_u32: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Quotes Replace Data
///
/// **Used in:** `perp_quotes_replace` instruction
///
/// **Tag:** `42`
///
/// ### Fields
/// - `mask` - Multiple quotes order manager
/// - `instr_id` - Instr pair id
pub struct PerpQuotesReplaceData {
    #[skip]
    pub tag: u8,
    pub bump: u8,
    pub order_type: u8,
    pub bail_on_order_not_found: u8,
    #[idl_type(u16)]
    pub mask: QuoteMask,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[padding]
    pub padding_u32: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Voting Data
///
/// **Used in:** `voting` instruction
///
/// **Tag:** `32`
///
/// ### Fields
/// - `choice`: VoteOption - Voting choice
/// - `voting_counter` - Current voting counter
pub struct VotingData {
    #[skip]
    pub tag: u8,
    pub choice: u8,
    #[padding]
    pub padding_u16: u16,
    pub voting_counter: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Airdrop Data
///
/// **Used in:** `airdrop` instruction
///
/// **Tag:** `27`
///
/// ### Fields
/// - `ratio` - ratio DRVS token to airdrop token
pub struct AirdropData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub ratio: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Upgrade To Perp
///
/// **Used in:** `upgrade_to_perp` instruction
///
/// **Tag:** `10`
///
/// ### Fields
/// - `instr_id` - Upgradable instrument pair id
pub struct UpgradeToPerpData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Set Instrument Ready For Perp Upgrade Data
///
/// **Used in:** `set_instr_ready_for_perp_upgrade` instruction
///
/// **Tag:** `41`
///
/// ### Fields
/// - `instr_id` - Instrument pair id
pub struct SetInstrReadyForPerpUpgradeData {
    #[skip]
    pub tag: u8, // 41
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Order Cancel Data
///
/// **Used in:** `perp_order_cancel` instruction
///
/// **Tag:** `30`
///
/// ### Fields
/// - `side`: OrderSide - Orders side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `order_id` - Orders id in the system
pub struct PerpOrderCancelData {
    #[skip]
    pub tag: u8,
    pub side: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub order_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Mass Cancel Data
///
/// **Used in:** `perp_mass_cancel` instruction
///
/// **Tag:** `36`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct PerpMassCancelData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Change Ref Program Data
///
/// **Used in:** `change_ref_program` instruction
///
/// **Tag:** `44`
///
/// ### Fields
/// - `ref_program_duration` - New referral rpgoram duration
/// - `ref_link_duration` - New rerral link duration
/// - `ref_discount`- New rererral discount
/// - `ref_ratio` - New rerral ratio
pub struct ChangeRefProgramData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub ref_program_duration: u32,
    pub ref_link_duration: u32,
    pub ref_discount: f64,
    pub ref_ratio: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Buy Market Seat Data
///
/// **Used in:** `buy_market_seat` instruction
///
/// **Tag:** `47`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `amount` - Deposit amount in base crncy
/// - `edge_price` - Upper slippage bound for market seat purchase
pub struct BuyMarketSeatData {
    #[skip]
    pub tag: u8, //47
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub edge_price: i64,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Sell Market Seat Data
///
/// **Used in:** `sell_market_seat` instruction
///
/// **Tag:** `48`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `edge_price` - Lower slippage bound for market seat purchase
pub struct SellMarketSeatData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Private Client
///
/// **Used in:** `new_private_client` instruction
///
/// **Tag:** `49`
///
/// ### Fields
/// - `expiration_time` - Clients position in queue expiration time
pub struct NewPrivateClient {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub expiration_time: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Points Program Expiration
///
/// **Used in:** `change_points_program_expiration` instruction
///
/// **Tag:** `51`
///
/// ### Fields
/// - `new_expiration_time` - New points program expiration time
pub struct PointsProgramExpiration {
    #[skip]
    pub tag: u8, //51
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub new_expiration_time: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Set Variance Data
///
/// **Tag** `54`
///
/// ## Fields
/// - `variance` - Current price variance of given instrument
pub struct SetVarianceData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub variance: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Change Denominator
///
/// **Tag** `56`
///
/// ## Fields
/// - `base_crncy_id` - Base crncy id which denominator is being changed
/// - `denominator` - New denominator
pub struct ChangeDenominatorData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub base_crncy_id: u32,
    pub denominator: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// New Base Crncy
///
/// **Tag** `4`
///
/// ## Fields
/// - `denominator` - New denominator
pub struct NewBaseCrncyData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub denominator: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Clients Processing Data
///
/// **Tag** `57`
///
/// ## Fields
/// - `instr_id` - Instruments Id
pub struct PerpClientsProcessingData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Set Seat purchasing Fee
///
/// **Tag** `58`
///
/// ## Fields
/// - `fee` - seat fee, aligned by admin
pub struct SetSeatPurchasingFeeData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
    pub fee: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Change Vote Data
///
/// **Used in:** `chante_vote` instruction
///
/// **Tag:** `59`
///
/// ### Fields
/// - `new_choice`: VoteOption - Voting choice
/// - `voting_counter` - Current voting counter
pub struct ChangeVotingData {
    #[skip]
    pub tag: u8,
    pub new_choice: u8,
    #[padding]
    pub padding_u16: u16,
    pub voting_counter: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Garbage Collector Data
///
/// **Tag:** `60`
///
/// ### Fields
/// - `instr_id` - Instrument Id
pub struct GarbageCollectorData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Set Ref Id Data
///
/// **Tag** `61`
///
/// ### FIelds
/// - `ref_id` - New referral id
pub struct ActivateClientRefProgramData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub ref_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Clean Candles Data
///
/// **Tag** `62`
///
/// ### FIelds
/// - `instr_id` - instrument id
pub struct CleanCandlesData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Extend Candles Data
///
/// **Tag** `62`
///
/// ### FIelds
/// - `instr_id` - instrument id
pub struct ExtendCandlesData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct VmInitWithdrawData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub token_id: u32,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct VmChangeWhitelistData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub mask: VmMask,
    pub whitelist: [u32; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
/// Perp Withdraw Data
///
/// **Tag:** `74`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `amount` - Amount of tokens client wants to move from perp to spot
pub struct WithdrawSwapFeesData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[idl_type(i64)]
    pub amount: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct SetSAMMinQtyData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    #[idl_type(i64)]
    pub min_qty: CappedI64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct ChangeSAMFeesPolicyData {
    #[skip]
    pub tag: u8,
    pub sam_fee_type: u8, // 0 - zero fees, 1 - fixed_fees
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub fee_rate: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct SuspendInstrumentData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct VmDirectWithdrawData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct VmInitActivateData {
    #[skip]
    pub tag: u8,
    pub multisig: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct VmAddKaminoData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct VmRemoveKaminoData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[padding]
    pub padding_u32: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct KaminoInitObligationData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct KaminoInitInstrumentData {
    #[skip]
    pub tag: u8,
    #[padding]
    pub padding_u8: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
}

/// Kamino Change Position Data
///
/// **Used in:** `kamino_change_position` instruction
///
/// **Tag:** `85`
///
/// ### Fields
/// - `flags` - kamino_flags - REPAY_ALL/WITHDRAW_ALL
/// - `instr_id` - Instrument id
/// - `borrow_delta` - Position change of liquidity in kamino
/// - `collateral_delta` - Position change of collateral in kamino
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct KaminoChangePositionData {
    #[skip]
    pub tag: u8,
    pub flags: u8,
    #[padding]
    pub padding_u16: u16,
    #[idl_type(u32)]
    pub instr_id: InstrId,
    pub borrow_delta: i64,
    pub collateral_delta: i64,
    pub custom_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug, ShankType)]
pub struct SetForeignDepositData {
    #[skip]
    pub tag: u8,
    pub foreign_deposit: u8,
}
