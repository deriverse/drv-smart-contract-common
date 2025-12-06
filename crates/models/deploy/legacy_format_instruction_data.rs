#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// New Operator Data
///
/// **Used in:** `new_operator` instruction
///
/// **Tag:** `1`
///
/// ### Fields
/// - `version` - smart contract version
pub struct NewOperatorData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub version: Version,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub private_mode: u8,
    pub padding_u16: u16,
    pub version: Version,
    pub lut_slot: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// New Spot Order Data
///
/// **Used in:** `new_spot_order` instruction
///
/// **Tag:** `12`
///
/// ### Fields
/// - `ioc`: bool - Use immediate or candel mode
/// - `order_type`: OrderType - new order type
/// - `order_side`: OrderSide - new order side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `price` - Price for **Limit** order
/// - `amount` - Orders qty in base crncy
pub struct NewSpotOrderData {
    pub tag: u8,
    pub ioc: u8,
    pub order_type: u8,
    pub side: u8,
    pub instr_id: u32,
    pub price: i64,
    pub amount: i64,
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// New Perp Order Data
///
/// **Used in:** `new_spot_order` instruction
///
/// **Tag:** `19`
///
/// ### Fields
/// - `ioc`: bool - Use immediate or candel mode
/// - `leverage` - New leverage value, if is 0 change to max possible
/// - `order_type`: OrderType - new order type
/// - `order_side`: OrderSide - new order side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `price` - Price for **Limit** order
/// - `amount` - Orders qty in base crncy
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
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub leverage: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Perp Statistics Reset Data
///
/// **Used in:** `perp_statistic_reset` instruction
///
/// **Tag:** `46`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct PerpStatisticsResetData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub order_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Spot Mass Cancel Data
///
/// **Used in:** `spot_mass_cancel` instruction
///
/// **Tag:** `15`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct SpotMassCancelData {
    pub tag: u8, //15
    padding_u8: u8,
    padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Spot LP Data
///
/// **Used in:** `spot_lp` instruction
///
/// **Tag:** `14`
///
/// ### Fields
/// - `side`: OrderSide - Orders side (Bid/Ask)
/// - `instr_id` - Instr pair id
/// - `amount` - Orders qty in base crncy
pub struct SpotLpData {
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub crncy_token_id: u32,
    pub lut_slot: u32,
    pub price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub competition_id: u8,
    pub deposit_all: u8,
    pub padding_u8: u8,
    pub token_id: u32,
    pub amount: i64,
    pub lut_slot: u32,
    pub ref_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Move Spot Available Funds Data
///
/// **Used in:** `move_spot_avail_funds` instruction
///
/// **Tag:** `43`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct MoveSpotAvailFundsData {
    pub tag: u8, //43
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    padding_u8: u8,
    padding_u16: u16,
    pub token_id: u32,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SwapData {
    pub tag: u8,
    pub input_crncy: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub price: i64,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Spot Quotes Replace Data
///
/// **Used in:** `spot_quotes_replace` instruction
///
/// **Tag:** `34`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `new_bid_price` - Limit price for a swap on Bid
/// - `new_bid_qty` - New Bid order qty
/// - `old_bid_order_id` - Old Bid order id
/// - `new_ask_price` - Limit price for a swap on Ask
/// - `new_ask_qty` - New Ask order qty
/// - `old_ask_order_id` - Old Asko order id
pub struct SpotQuotesReplaceData {
    pub tag: u8,
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
/// Perp Quotes Replace Data
///
/// **Used in:** `perp_quotes_replace` instruction
///
/// **Tag:** `42`
///
/// ### Fields
/// - `instr_id` - Instr pair id
/// - `new_bid_price` - Limit price for a swap on Bid
/// - `new_bid_qty` - New Bid order qty
/// - `old_bid_order_id` - Old Bid order id
/// - `new_ask_price` - Limit price for a swap on Ask
/// - `new_ask_qty` - New Ask order qty
/// - `old_ask_order_id` - Old Asko order id
pub struct PerpQuotesReplaceData {
    pub tag: u8,
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
    pub tag: u8,
    pub choice: u8,
    pub padding_u16: u16,
    pub voting_counter: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Airdrop Data
///
/// **Used in:** `airdrop` instruction
///
/// **Tag:** `27`
///
/// ### Fields
/// - `ratio` - ratio DRVS token to airdrop token
pub struct AirdropData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub ratio: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Upgrade To Perp
///
/// **Used in:** `upgrade_to_perp` instruction
///
/// **Tag:** `10`
///
/// ### Fields
/// - `instr_id` - Upgradable instrument pair id
pub struct UpgradeToPerpData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Set Instrument Ready For Perp Upgrade Data
///
/// **Used in:** `set_instr_ready_for_perp_upgrade` instruction
///
/// **Tag:** `41`
///
/// ### Fields
/// - `instr_id` - Instrument pair id
/// - `variance` - Current price variance of given instrument
pub struct SetInstrReadyForPerpUpgradeData {
    pub tag: u8, // 41
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub variance: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
    pub side: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub order_id: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Perp Mass Cancel Data
///
/// **Used in:** `perp_mass_cancel` instruction
///
/// **Tag:** `36`
///
/// ### Fields
/// - `instr_id` - Instr pair id
pub struct PerpMassCancelData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
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
    pub tag: u8,
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
    pub edge_price: i64,
    pub amount: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SellMarketSeatData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
    pub edge_price: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// New Private Client
///
/// **Used in:** `new_private_client` instruction
///
/// **Tag:** `49`
///
/// ### Fields
/// - `expiration_time` - Clients position in queue expiration time
pub struct NewPrivateClient {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub expiration_time: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
/// Points Program Expiration
///
/// **Used in:** `change_points_program_expiration` instruction
///
/// **Tag:** `51`
///
/// ### Fields
/// - `new_expiration_time` - New points program expiration time
pub struct PointsProgramExpiration {
    pub tag: u8, //51
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub new_expiration_time: u32,
}

pub struct SetVarianceData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: InstrId,
    pub variance: f64,
}

pub struct ChangeDenominatorData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub base_crncy_id: u32,
    pub denominator: f64,
}

pub struct NewBaseCrncyData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub denominator: f64,
}

pub struct PerpClientsProcessingData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

pub struct SetSeatpurchasingFeeData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub padding_u32: u32,
    pub fee: f64,
}

pub struct ChangeVotingData {
    pub tag: u8,
    pub new_choice: u8,
    pub padding_u16: u16,
    pub voting_counter: u32,
}

pub struct GarbageCollectorData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub instr_id: u32,
}

pub struct ActivateClientRefProgramData {
    pub tag: u8,
    pub padding_u8: u8,
    pub padding_u16: u16,
    pub ref_id: u32,
}
