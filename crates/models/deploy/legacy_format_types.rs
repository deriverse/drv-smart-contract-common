use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Copy, PartialEq)]
pub enum OrderSide {
    Bid,
    Ask,
}

pub mod order_type {
    pub const LIMIT: u8 = 0;
    pub const MARKET: u8 = 1;
    pub const MARGIN_CALL: u8 = 2;
    pub const FORCED_CLOSE: u8 = 3;
}

pub mod instr_mask {
    pub const DRV: u32 = 0x10000000;
    pub const READY_TO_DRV_UPGRADE: u32 = 0x20000000;
    pub const PERP: u32 = 0x40000000;
    pub const ORACLE: u32 = 0x80000000;
    pub const READY_TO_PERP_UPGRADE: u32 = 0x1000000;
}

pub mod account_type {
    pub const CLIENT_COMMUNITY: u32 = 35;
    pub const CLIENT_PRIMARY: u32 = 31;
    pub const COMMUNITY: u32 = 34;
    pub const FUTURES_ASK_ORDERS: u32 = 29;
    pub const FUTURES_ASKS_TREE: u32 = 27;
    pub const FUTURES_BID_ORDERS: u32 = 28;
    pub const FUTURES_BIDS_TREE: u32 = 26;
    pub const FUTURES_CLIENT_ACCOUNTS: u32 = 23;
    pub const FUTURES_CLIENT_INFOS: u32 = 24;
    pub const FUTURES_CLIENT_INFOS2: u32 = 25;
    pub const FUTURES_LINES: u32 = 30;
    pub const FUTURES_MAPS: u32 = 22;
    pub const HOLDER: u32 = 1;
    pub const ROOT: u32 = 2;
    pub const INSTR: u32 = 7;
    pub const SPOT_15M_CANDLES: u32 = 20;
    pub const SPOT_1M_CANDLES: u32 = 19;
    pub const SPOT_ASK_ORDERS: u32 = 17;
    pub const SPOT_ASKS_TREE: u32 = 15;
    pub const SPOT_BID_ORDERS: u32 = 16;
    pub const SPOT_BIDS_TREE: u32 = 14;
    pub const SPOT_CLIENT_INFOS: u32 = 12;
    pub const SPOT_CLIENT_INFOS2: u32 = 13;
    pub const SPOT_DAY_CANDLES: u32 = 21;
    pub const SPOT_LINES: u32 = 18;
    pub const SPOT_MAPS: u32 = 10;
    pub const TOKEN: u32 = 4;
    pub const PERP_ASK_ORDERS: u32 = 36;
    pub const PERP_ASKS_TREE: u32 = 37;
    pub const PERP_BID_ORDERS: u32 = 38;
    pub const PERP_BIDS_TREE: u32 = 39;
    pub const PERP_CLIENT_INFOS: u32 = 41;
    pub const PERP_CLIENT_INFOS2: u32 = 42;
    pub const PERP_CLIENT_INFOS3: u32 = 43;
    pub const PERP_CLIENT_INFOS4: u32 = 44;
    pub const PERP_CLIENT_INFOS5: u32 = 45;
    pub const PERP_LINES: u32 = 46;
    pub const PERP_MAPS: u32 = 47;
    pub const PERP_LONG_PX_TREE: u32 = 48;
    pub const PERP_SHORT_PX_TREE: u32 = 49;
    pub const PERP_REBALANCE_TIME_TREE: u32 = 50;
    pub const PRIVATE_CLIENTS: u32 = 51;
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
/// New path - src/state/client_community.rs
pub struct ClientCommunityRecord {
    pub dividends_rate: f64,
    pub dividends_value: i64,
    pub fees_prepayment: i64,
    pub fees_ratio: f64,
    pub ref_rewards: i64,
    pub ref_payments: i64,
    pub last_fees_prepayment_time: u32,
    pub crncy_token_id: u32,
}

pub const CLIENT_COMMUNITY_RECORD_SIZE: usize = std::mem::size_of::<ClientCommunityRecord>();

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ClientCommunityAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub id: u32,
    pub last_voting_time: u32,
    pub last_voting_counter: u32,
    pub current_voting_counter: u32,
    pub current_voting_tokens: i64,
    pub last_voting_tokens: i64,
    pub last_choice: u32,
    pub slot: u32,
    pub drvs_tokens: i64,
    pub count: u32,
    pub reserved: u32,
}
pub const CLIENT_COMMUNITY_ACCOUNT_HEADER_SIZE: usize =
    std::mem::size_of::<ClientCommunityAccountHeader>();

#[repr(C)]
#[derive(Zeroable, Default)]
/// New path - src/state/client_community.rs
pub struct CommunityAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub drvs_tokens: i64,
    pub min_amount: i64,
    pub voting_supply: i64,
    pub prev_voting_supply: i64,
    pub voting_decr: i64,
    pub prev_voting_decr: i64,
    pub voting_unchange: i64,
    pub prev_voting_unchange: i64,
    pub voting_incr: i64,
    pub prev_voting_incr: i64,
    pub voting_counter: u32,
    pub voting_start_slot: u32,
    pub voting_end_time: u32,
    pub spot_fee_rate: u32,
    pub perp_fee_rate: u32,
    pub spot_pool_ratio: u32,
    pub margin_call_penalty_rate: u32,
    pub fees_prepayment_for_max_discount: u32,
    pub max_discount: u32,
    pub reserved_value1: u32,
    pub reserved_value2: u32,
    pub reserved_value3: u32,
    pub reserved_value4: u32,
    pub reserved_value5: u32,
    pub reserved_value6: u32,
    pub reserved_value7: u32,
    pub reserved_value8: u32,
    pub count: u32,
}

pub const COMMUNITY_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<CommunityAccountHeader>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/spots/spot_account_header.rs
pub struct SpotTradeAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub instr_id: u32,
    pub slot: u32,
    pub asset_token_id: u32,
    pub crncy_token_id: u32,
}

pub const SPOT_TRADE_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<SpotTradeAccountHeader>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/perps/perp_account_header.rs
pub struct PerpTradeAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub id: u32,
    pub slot: u32,
    pub asset_token_id: u32,
    pub crncy_token_id: u32,
}

pub const PERP_TRADE_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<PerpTradeAccountHeader>();

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]
/// New path - src/state/holder.rs
pub struct HolderAccountHeader {
    pub tag: u32,
    pub operators_count: u32,
}

pub const HOLDER_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<HolderAccountHeader>();

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
/// New path - src/state/types.rs
pub struct Operator {
    pub operator_address: Pubkey,
    pub version: u32,
    pub reserved: u32,
}

pub const OPERATOR_SIZE: usize = std::mem::size_of::<Operator>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/types.rs
pub struct LineQuotes {
    pub px: i64,
    pub qty: i64,
}

pub const LINE_QUOTES_SIZE: usize = std::mem::size_of::<LineQuotes>();

#[repr(C)]
#[derive(Clone, Copy)]
/// New path - src/state/candle.rs
pub struct Candle {
    pub open: i64,
    pub close: i64,
    pub max: i64,
    pub min: i64,
    pub asset_tokens: i64,
    pub crncy_tokens: i64,
    pub time: u32,
    pub counter: u32,
}

pub const CANDLE_SIZE: usize = std::mem::size_of::<Candle>();

#[repr(C)]
#[derive(Zeroable, Default)]
/// New path - src/state/candle.rs
pub struct CandlesAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub id: u32,
    pub slot: u32,
    pub count: u32,
    pub last: u32,
}

pub const CANDLES_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<CandlesAccountHeader>();

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default)]
pub struct InstrAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub instr_id: u32,
    pub asset_token_id: u32,
    pub crncy_token_id: u32,
    pub mask: u32,
    pub asset_tokens: i64,
    pub crncy_tokens: i64,
    pub ps: i64,
    pub pool_fees: i64,
    pub last_px: i64,
    pub last_close: i64,
    pub alltime_trades: i64,
    pub prev_day_trades: i64,
    pub day_volatility: f64,
    pub perp_last_px: i64,
    pub perp_last_close: i64,
    pub perp_alltime_trades: i64,
    pub perp_prev_day_trades: i64,
    pub perp_open_int: i64,
    pub maps_address: Pubkey,
    pub perp_maps_address: Pubkey,
    pub lut_address: Pubkey,
    pub drv_count: u32,
    pub asset_token_decs_count: u32,
    pub crncy_token_decs_count: u32,
    pub slot: u32,
    pub creator: Pubkey,
    pub last_time: u32,
    pub distrib_time: u32,
    pub base_crncy_index: u32,
    pub instance_counter: u32,
    pub variance_counter: u32,
    pub bids_tree_nodes_count: u32,
    pub bids_tree_lines_entry: u32,
    pub bids_tree_orders_entry: u32,
    pub asks_tree_nodes_count: u32,
    pub asks_tree_lines_entry: u32,
    pub asks_tree_orders_entry: u32,
    pub bid_lines_begin: u32,
    pub bid_lines_end: u32,
    pub bid_lines_count: u32,
    pub ask_lines_begin: u32,
    pub ask_lines_end: u32,
    pub ask_lines_count: u32,
    pub bid_orders_count: u32,
    pub ask_orders_count: u32,
    pub fixing_time: u32,
    pub fixing_crncy_tokens: i64,
    pub fixing_asset_tokens: i64,
    pub counter: i64,
    pub protocol_fees: i64,
    pub hits_counter: i64,
    pub last_asset_tokens: i64,
    pub last_crncy_tokens: i64,
    pub last_trade_asset_tokens: i64,
    pub last_trade_crncy_tokens: i64,
    pub perp_underlying_px: i64,
    pub best_bid: i64,
    pub best_ask: i64,
    pub fixing_px: i64,
    pub variance: f64,
    pub avg_spread: f64,
    pub last_spread: f64,
    pub last_spread_time: u32,
    pub total_spread_period: u32,
    pub day_asset_tokens: i64,
    pub day_crncy_tokens: i64,
    pub day_low: i64,
    pub day_high: i64,
    pub prev_day_asset_tokens: i64,
    pub prev_day_crncy_tokens: i64,
    pub alltime_asset_tokens: f64,
    pub alltime_crncy_tokens: f64,
    pub day_trades: u32,
    pub lp_day_trades: u32,
    pub lp_alltime_fees: f64,
    pub lp_day_fees: i64,
    pub lp_prev_day_fees: i64,
    pub lp_prev_day_trades: u32,
    pub lp_time: u32,
    pub fees_time: u32,
    pub creation_time: u32,
    pub day_fees: i64,
    pub alltime_fees: f64,
    pub prev_day_fees: i64,
    pub dec_factor: i64,
    pub perp_clients_count: u32,
    pub perp_slot: u32,
    pub perp_time: u32,
    pub perp_funding_rate_slot: u32,
    pub perp_funding_rate_time: u32,
    pub perp_long_px_tree_nodes_count: u32,
    pub perp_long_px_tree_entry: u32,
    pub perp_short_px_tree_nodes_count: u32,
    pub perp_short_px_tree_entry: u32,
    pub perp_rebalance_time_tree_nodes_count: u32,
    pub perp_rebalance_time_tree_entry: u32,
    pub perp_bids_tree_nodes_count: u32,
    pub perp_bids_tree_lines_entry: u32,
    pub perp_bids_tree_orders_entry: u32,
    pub perp_asks_tree_nodes_count: u32,
    pub perp_asks_tree_lines_entry: u32,
    pub perp_asks_tree_orders_entry: u32,
    pub perp_bid_lines_begin: u32,
    pub perp_bid_lines_end: u32,
    pub perp_bid_lines_count: u32,
    pub perp_ask_lines_begin: u32,
    pub perp_ask_lines_end: u32,
    pub perp_ask_lines_count: u32,
    pub perp_bid_orders_count: u32,
    pub perp_ask_orders_count: u32,
    pub perp_day_trades: u32,
    pub perp_long_spot_price_for_withdrowal: i64,
    pub perp_short_spot_price_for_withdrowal: i64,
    pub perp_soc_loss_long_rate: f64,
    pub perp_soc_loss_short_rate: f64,
    pub perp_price_delta: f64,
    pub perp_funding_rate: f64,
    pub perp_funding_funds: i64,
    pub perp_soc_loss_funds: i64,
    pub perp_insurance_fund: i64,
    pub pepr_last_trade_asset_tokens: i64,
    pub perp_last_trade_crncy_tokens: i64,
    pub perp_best_bid: i64,
    pub perp_best_ask: i64,
    pub perp_day_asset_tokens: i64,
    pub perp_day_crncy_tokens: i64,
    pub perp_day_low: i64,
    pub perp_day_high: i64,
    pub perp_prev_day_asset_tokens: i64,
    pub perp_prev_day_crncy_tokens: i64,
    pub perp_alltime_asset_tokens: f64,
    pub perp_alltime_crncy_tokens: f64,
    pub max_leverage: f64,
    pub liquidation_threshold: f64,
    pub seats_reserve: i64,
    pub init_seat_price: f64,
    pub ema_px: f64,
    pub reserved_value1: i64,
    pub reserved_value2: i64,
    pub reserved_value3: i64,
    pub reserved_value4: i64,
    pub reserved_value5: i64,
    pub reserved_value6: i64,
    pub reserved_value7: i64,
    pub reserved_value8: i64,
    pub reserved_value9: i64,
    pub reserved_value10: i64,
}

pub const INSTR_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<InstrAccountHeader>();

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default)]
pub struct RootState {
    pub tag: u32,
    pub version: u32,
    pub operator_address: Pubkey,
    pub holder_address: Pubkey,
    pub drvs_mint_address: Pubkey,
    pub lut_address: Pubkey,
    pub airdrop_authority_address: Pubkey,
    pub private_mode_authority_address: Pubkey,
    pub ref_program_duration: u32,
    pub ref_link_duration: u32,
    pub ref_discount: f64,
    pub ref_ratio: f64,
    pub clients_count: u32,
    pub tokens_count: u32,
    pub instr_count: u32,
    pub ref_counter: u32,
    pub mask: u32,
    pub points_program_expiration: u32,
    pub purchasing_perp_seat_fee: f64,
}

pub const ROOT_ACCOUNT_SIZE: usize = std::mem::size_of::<RootState>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Default, Pod)]
/// New path - src/state/token/rs
pub struct TokenState {
    pub tag: u32,
    pub version: u32,
    pub address: Pubkey,
    pub program_address: Pubkey,
    pub id: u32,
    pub mask: u32,
    pub reserved: u32,
    pub base_crncy_index: u32,
}

pub const TOKEN_ACCOUNT_SIZE: usize = std::mem::size_of::<TokenState>();

#[repr(C)]
#[derive(Clone, Copy)]
/// New path - src/state/types.rs
pub struct BaseCrncyRecord {
    pub crncy_token_id: u32,
    pub decs_count: u32,
    pub funds: i64,
    pub rate: f64,
    pub denominator: f64,
    pub locked_drvs_amount: i64,
    pub locked_drvs_dividends_value: i64,
    pub mask: i64,
}

pub mod asset_type {
    pub const TOKEN: u32 = 0x1000000;
    pub const SPOT_LP: u32 = 0x2000000;
    pub const SPOT_ORDERS: u32 = 0x3000000;
    pub const PERP: u32 = 0x4000000;
}

#[repr(C)]
#[derive(Zeroable)]
/// New path - src/state/types.rs
pub struct ClientSpot {
    pub asset_id: u32,
    pub temp_client_id: u32,
    pub slot: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Zeroable)]
/// New path - src/state/types.rs
pub struct ClientPerp {
    pub asset_id: u32,
    pub temp_client_id: u32,
    pub slot: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/types.rs
pub struct AssetRecord {
    pub asset_id: u32,
    pub temp_id: u32,
    pub value: i64,
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/spots/spot_infos.rs
pub struct SpotClientInfo {
    pub client: u32,
    pub reserved: u32,
    pub bids_entry: u32,
    pub asks_entry: u32,
    pub avail_asset_tokens: i64,
    pub avail_crncy_tokens: i64,
}

pub const SPOT_CLIENT_INFO_SIZE: usize = std::mem::size_of::<SpotClientInfo>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/spots/spot_infos.rs
pub struct SpotClientInfo2 {
    pub in_orders_asset_tokens: i64,
    pub in_orders_crncy_tokens: i64,
    pub bid_slot: u32,
    pub ask_slot: u32,
    pub reserved: i64,
}

pub const SPOT_CLIENT_INFO2_SIZE: usize = std::mem::size_of::<SpotClientInfo2>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/perps/porp_infos.rs
pub struct PerpClientInfo {
    pub funds: i64,
    pub perps: i64,
    pub in_orders_funds: i64,
    pub in_orders_perps: i64,
}

pub const PERP_CLIENT_INFO_SIZE: usize = std::mem::size_of::<PerpClientInfo>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/perps/porp_infos.rs
pub struct PerpClientInfo2 {
    pub cost: i64,
    pub result: i64,
    pub bid_slot: u32,
    pub ask_slot: u32,
    pub px_node: u32,
    pub mask: u32,
}

pub const PERP_CLIENT_INFO2_SIZE: usize = std::mem::size_of::<PerpClientInfo2>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/perps/porp_infos.rs
pub struct PerpClientInfo3 {
    pub client: u32,
    pub reserved: u32,
    pub bids_entry: u32,
    pub asks_entry: u32,
    pub fees: i64,
    pub rebates: i64,
}

pub const PERP_CLIENT_INFO3_SIZE: usize = std::mem::size_of::<PerpClientInfo3>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/perps/porp_infos.rs
pub struct PerpClientInfo4 {
    pub last_soc_loss_rate: f64,
    pub last_soc_loss_perps: i64,
    pub soc_loss_funds: i64,
    pub loss_coverage: i64,
}

pub const PERP_CLIENT_INFO4_SIZE: usize = std::mem::size_of::<PerpClientInfo4>();

#[repr(C)]
#[derive(Copy, Clone, Zeroable)]
/// New path - src/state/perps/porp_infos.rs
pub struct PerpClientInfo5 {
    pub funding_funds: i64,
    pub last_funding_rate: f64,
    pub reserved: i64,
    pub rebalance_time: u32,
    pub funding_node: u32,
}

pub const PERP_CLIENT_INFO5_SIZE: usize = std::mem::size_of::<PerpClientInfo5>();

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ClientPrimaryAccountHeader {
    pub tag: u32,
    pub version: u32,
    pub wallet_address: Pubkey,
    pub lut_address: Pubkey,
    pub ref_address: Pubkey,
    pub vm_wallet_address: Pubkey,
    pub vm_instr_0: u32,
    pub vm_instr_1: u32,
    pub vm_instr_2: u32,
    pub vm_instr_3: u32,
    pub vm_instr_4: u32,
    pub vm_instr_5: u32,
    pub vm_instr_6: u32,
    pub vm_instr_7: u32,
    pub vm_withdraw_token_id: u32,
    pub vm_mask: u32,
    pub vm_withdraw_amount: i64,
    pub first_ref_link_discount: f64,
    pub second_ref_link_discount: f64,
    pub first_ref_link_ratio: f64,
    pub second_ref_link_ratio: f64,
    pub ref_program_discount: f64,
    pub ref_program_ratio: f64,
    pub reserved: i64,
    pub mask: i64,
    pub id: u32,
    pub ref_client_id: u32,
    pub ref_counter: u32,
    pub first_ref_link_id: u32,
    pub second_ref_link_id: u32,
    pub first_ref_link_expiration: u32,
    pub second_ref_link_expiration: u32,
    pub ref_program_expiration: u32,
    pub spot_trades: u32,
    pub perp_trades: u32,
    pub lp_trades: u32,
    pub points: u32,
    pub slot: u32,
    pub assets_count: u32,
    pub spot_filled_orders: u32,
    pub perp_filled_orders: u32,
    pub reserved_value1: i64,
    pub reserved_value2: i64,
    pub reserved_value3: i64,
    pub reserved_value4: i64,
    pub reserved_value5: i64,
    pub reserved_value6: i64,
    pub reserved_value7: i64,
    pub reserved_value8: i64,
}

pub const CLIENT_PRIMARY_ACCOUNT_HEADER_SIZE: usize =
    std::mem::size_of::<ClientPrimaryAccountHeader>();

#[repr(C)]
#[derive(Copy, Clone)]
/// New path - src/state/types.rs
pub struct Order {
    pub qty: i64,
    pub sum: i64,
    pub order_id: i64,
    pub orig_client_id: u32,
    pub client_id: u32,
    pub line: u32,
    pub prev: u32,
    pub next: u32,
    pub sref: u32,
    pub link: u32,
    pub cl_prev: u32,
    pub cl_next: u32,
    pub time: u32,
}

#[repr(C)]
/// New path - src/state/types.rs
pub struct PxOrders {
    pub price: i64,
    pub qty: i64,
    pub next: u32,
    pub prev: u32,
    pub sref: u32,
    pub link: u32,
    pub begin: u32,
    pub end: u32,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, Debug)]
pub struct PrivateClientHeader {
    pub tag: u32,
    pub version: u32,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Default, Debug)]
pub struct PrivateClient {
    pub creation_time: u32,
    pub expiration_time: u32,
    pub wallet: Pubkey,
}
