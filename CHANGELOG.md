# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

## Removed 
- `day_fees`, `prev_day_fees`, `alltime_fees`

## Fixed
- `pepr...` typo

## [v2.15] - 2025-12-03

## Added 
- `SeatPurchasingFeeInstruction` - for fee alignment 
- `MAX_DENOMINATOR` constants 
- `NewBaseCrncyData` instruction data
- Fields in `InstrAccountHeader`: `init_seat_price` and `ema_px`
- `MIN_AMOUNT_FOR_DIVIDENDS_ALLOCATION` constant
- `perp_last_trade_asset/crncy_tokens` and `last_trade_asset/crncy_tokens` statistic for last trade record

## Removed
- `init_seat_price` fields, return to `INIT_SEAT_PRICE` constant
- `READY_TO_DRV_UPGRADE` flag

## Fixed
- `lower_slippage_bound` and `upper_slippage_bound` to `i64`

## Changed
- `lower_slippage_bound` and `upper_slippage_bound` naming to `edge_price`

## [v2.13] - 2025-12-02

## Added
- `set_variance` `voting_reset` constants for instructions
- `INIT_VARIANCE`, `INIT_DAY_VOLATILITY` constants for new volatility/varaince calculations
- `perp_short_spot_price_for_withdrowal` and `perp_long_spot_price_for_withdrowal` prices before margin call which are used during avail funds calculations

## Changed
- `Fee rate` and `Margin call penalty rate` constants

## [v2.12] - 2025-11-28

## Added
- `MAX_INSTR_COUNT` constants 
- fees statistic records in `InstrAccountHeader`

## Changed
- increased `AssetTypes` literals, so for `SpotOrder` and `Perp` new max instr count can fit

## Removed 
- `SetOracleFeed` instruction data

## [v2.11] - 2025-11-27

## Added
- `purchasing_perp_seat_fee` field in `RootState`. Fee will be seted by operator admin in case of exsessive manipulations
- `spot_filled_orders` and `perp_filled_orders` in `ClientPrimaryAccountHeader`
- `locked_drvs_amount`, `locked_drvs_dividends_value`, `mask` fields in `BaseCrncyRecord` for dividends from pool issue solving
- Error messages for `spot_lp`
- Error message for incorrect slippage
- `edge_price` field in `SpotLpData` instruction data
- `MaxClientsOrderLimitReached` error
- `MAX_CLIENT_SIDE_ORDERS_COUNT` constants (temporary solution)
- Error messages for private mode authority validations
- `InstructionData` constant for `changePrivateModeAuthority` instruction

## [v2.10] - 2025-11-24

## Changed 
- Removed oracle

## Added
- `private_mode_authority` in `RootState`
- `denominator` field in `BaseCrncyRecord`
- Error message `Invalid Base Cnrcy`
- `upper/lower_slippage_bound` for `BuyMarketSeat` and `SellMarketSeat` instruction data

## [v2.9] - 2025-11-18

## Added
- min voting quorum
- `lp_time`, `fees_time`, `lp_day_fees`, `lp_prev_day_fees`, `lp_alltime_fees`

## Changed
- Field `side` from `SwapData` to `input_crncy`
- Naming of `private_mode` (`pm`) to `vault_mode` (`vm`)

## Removed 
- `feed_id` field from `InstrAccountHeader`

## [v2.8] - 2025-11-12

## Added
- `airdrop_authority_address` to `RootState`
- `ChangeAirdropAuthority` instruction

## [v2.7] - 2025-11-12

## Changed
- Updated `legacy` types

## [v2.6] - 2025-11-11

## Added
- `WALLET_RESERVE_LAMPORTS` constant

## Fix
- incorrect imports of `std::mem::size_of`
- Invalid min input accounts amount in `NewBaseCrncyInstruction`

## [v2.5] - 2025-11-06

## Fix
- Typos

## [v2.4] - 2025-11-05

## Added
- Reserved fields for future protected mode implementation in `ClientPrimaryAccountHeadert`
- Invalid variance error

## Changed
- Changed start fee rate for both spot/perp in voting constants (topic `Fee Rate`)

## [v2.3] - 2025-11-05

## Added
- Rust docs for models
- Rust docs for instruction data
