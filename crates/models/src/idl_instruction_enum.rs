use bytemuck::Zeroable;
use shank::ShankInstruction;

use crate::instruction_data::*;

#[derive(Debug, Clone, Copy, ShankInstruction)]
#[repr(u8)]
pub enum DrvInstructionIdl {
    /// Create the holder account owned by the hard-coded holder admin.
    #[account(
        0,
        signer,
        writable,
        name = "holder_admin",
        desc = "Holder admin that funds the PDA creation"
    )]
    #[account(1, writable, name = "holder_account", desc = "Holder PDA", seeds = [b"drvs001", holder_admin.key()], bump)]
    #[account(2, name = "system_program", desc = "System program")]
    NewHolder = 0,

    /// Append a new operator entry to the holder account.
    #[account(
        0,
        signer,
        writable,
        name = "holder_admin",
        desc = "Holder admin authority"
    )]
    #[account(1, writable, name = "holder_account", desc = "Holder PDA", seeds = [b"drvs001", holder_admin.key()], bump)]
    #[account(2, name = "operator_address", desc = "New operator authority pubkey")]
    #[account(3, name = "system_program", desc = "System program")]
    NewOperator(NewOperatorData) = 1,

    /// Create the root, community, DRVS token metadata, and root lookup table.
    #[account(0, signer, writable, name = "admin", desc = "Root creation authority")]
    #[account(
        1,
        name = "holder_account",
        desc = "Holder PDA used to validate the operator version",
        seeds = [b"drvs001", admin.key()],
        bump
    )]
    #[account(2, writable, name = "root", desc = "Root PDA", seeds = [args.version.to_le_bytes(), [2u8, 0u8, 0u8, 0u8], deriverse_authority.key()], bump)]
    #[account(3, name = "deriverse_authority", desc = "Deriverse authority PDA", seeds = [b"ndxnt"], bump)]
    #[account(4, writable, name = "community", desc = "Community PDA", seeds = [args.version.to_le_bytes(), [34u8, 0u8, 0u8, 0u8], deriverse_authority.key()], bump)]
    #[account(5, name = "drvs_mint", desc = "DRVS mint")]
    #[account(6, writable, name = "drvs_token", desc = "DRVS token metadata PDA")]
    #[account(
        7,
        signer,
        writable,
        name = "drvs_program_token_account",
        desc = "New DRVS program token account"
    )]
    #[account(8, writable, name = "lut_account", desc = "Root lookup table account")]
    #[account(9, name = "system_program", desc = "System program")]
    #[account(10, name = "lut_program", desc = "Address lookup table program")]
    #[account(11, name = "token_2022_program", desc = "Token 2022 program id")]
    NewRootAccount(NewRootAccountData) = 2,

    /// Move available perp funds back into the client's spot balance.
    #[account(0, signer, writable, name = "signer", desc = "Perp account owner")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(
        2,
        writable,
        name = "client_primary",
        desc = "Client primary account",
        seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()],
        bump
    )]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    PerpWithdraw(PerpWithdrawData) = 3,

    /// Create or mark a base-currency token inside community state.
    #[account(
        0,
        signer,
        writable,
        name = "admin",
        desc = "Base-currency creation authority"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "token", desc = "Token metadata PDA")]
    #[account(
        3,
        writable,
        name = "program_token_account",
        desc = "Program token account for the mint",
        seeds = [mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(4, name = "deriverse_authority", desc = "Deriverse authority PDA")]
    #[account(5, name = "token_program", desc = "Token program id")]
    #[account(6, name = "mint", desc = "Base-currency mint")]
    #[account(7, name = "system_program", desc = "System program")]
    #[account(8, writable, name = "community", desc = "Community account")]
    NewBaseCrncy(NewBaseCrncyData) = 4,

    /// Prepay community fees using a base-currency balance already held in the client account state.
    #[account(0, signer, name = "signer", desc = "Client wallet")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account")]
    #[account(3, writable, name = "community", desc = "Community account")]
    #[account(
        4,
        writable,
        name = "client_community",
        desc = "Client community account"
    )]
    #[account(5, name = "system_program", desc = "System program")]
    FeesDeposit(FeesDepositData) = 5,

    // 6 reserved
    /// Deposit tokens into Deriverse and create the client account if needed.
    #[account(0, signer, writable, name = "signer", desc = "Depositing wallet")]
    #[account(
        1,
        writable,
        name = "client_token_account",
        desc = "Client associated token account",
        seeds = [signer.key(), token_program.key(), mint.key()],
        bump
    )]
    #[account(
        2,
        writable,
        name = "program_token_account",
        desc = "Program token vault",
        seeds = [mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(3, name = "mint", desc = "Token mint")]
    #[account(4, name = "root", desc = "Root PDA")]
    #[account(5, name = "token", desc = "Token metadata PDA")]
    #[account(6, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(7, name = "system_program", desc = "System program")]
    #[account(8, name = "token_program", desc = "Token program id")]
    Deposit(DepositData) = 7,

    /// Withdraw tokens from Deriverse back to the client token account.
    #[account(0, signer, writable, name = "signer", desc = "Withdrawing wallet")]
    #[account(
        1,
        writable,
        name = "client_token_account",
        desc = "Client token destination"
    )]
    #[account(
        2,
        writable,
        name = "program_token_account",
        desc = "Program token vault",
        seeds = [mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(3, name = "mint", desc = "Token mint")]
    #[account(4, name = "root", desc = "Root PDA")]
    #[account(5, name = "token", desc = "Token metadata PDA")]
    #[account(6, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(7, name = "system_program", desc = "System program")]
    #[account(8, name = "token_program", desc = "Token program id")]
    #[account(
        9,
        name = "associated_token_program",
        desc = "Associated token program placeholder consumed by the current handler"
    )]
    Withdraw(WithdrawData) = 8,

    /// Create a new non-perp instrument and its spot trading storage.
    #[account(
        0,
        signer,
        writable,
        name = "signer",
        desc = "Instrument creation authority"
    )]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "asset_token", desc = "Asset token metadata PDA")]
    #[account(3, name = "currency_token", desc = "Currency token metadata PDA")]
    #[account(
        4,
        writable,
        name = "asset_token_program_account",
        desc = "Program token account for the asset mint",
        seeds = [asset_mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(5, name = "asset_mint", desc = "Asset mint")]
    #[account(
        6,
        writable,
        name = "lut_account",
        desc = "Instrument lookup table account"
    )]
    #[account(7, name = "system_program", desc = "System program")]
    #[account(8, name = "token_program", desc = "Token program id")]
    #[account(9, name = "lut_program", desc = "Address lookup table program")]
    #[account(10, name = "deriverse_authority", desc = "Deriverse authority PDA", seeds = [b"ndxnt"], bump)]
    #[account(11, writable, name = "instrument", desc = "Instrument account", seeds = [root.discriminator.version.to_le_bytes(), [7u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(12, writable, name = "bids_tree", desc = "Bids tree account", seeds = [root.discriminator.version.to_le_bytes(), [14u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(13, writable, name = "asks_tree", desc = "Asks tree account", seeds = [root.discriminator.version.to_le_bytes(), [15u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(14, writable, name = "bid_orders", desc = "Bid orders account", seeds = [root.discriminator.version.to_le_bytes(), [16u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(15, writable, name = "ask_orders", desc = "Ask orders account", seeds = [root.discriminator.version.to_le_bytes(), [17u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(16, writable, name = "lines", desc = "Spot lines account", seeds = [root.discriminator.version.to_le_bytes(), [18u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(17, writable, name = "maps", desc = "Spot maps account")]
    #[account(
        18,
        writable,
        name = "client_infos",
        desc = "Spot client infos account",
        seeds = [root.discriminator.version.to_le_bytes(), [12u8, 0u8, 0u8, 0u8], asset_token.id.to_le_bytes(), args.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    NewInstrument(NewInstrumentData) = 9,

    /// Upgrade an instrument that is already marked ready into the perp market.
    #[account(0, signer, writable, name = "signer", desc = "Upgrade initiator")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    #[account(
        3,
        writable,
        name = "lut_account",
        desc = "Instrument lookup table account"
    )]
    #[account(4, name = "system_program", desc = "System program")]
    #[account(5, name = "lut_program", desc = "Address lookup table program")]
    #[account(6, name = "deriverse_authority", desc = "Deriverse authority PDA")]
    #[account(7, writable, name = "perp_bids_tree", desc = "Perp bids tree account", seeds = [root.discriminator.version.to_le_bytes(), [39u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(8, writable, name = "perp_asks_tree", desc = "Perp asks tree account", seeds = [root.discriminator.version.to_le_bytes(), [37u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(
        9,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account",
        seeds = [root.discriminator.version.to_le_bytes(), [38u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        10,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account",
        seeds = [root.discriminator.version.to_le_bytes(), [36u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(11, writable, name = "perp_lines", desc = "Perp lines account", seeds = [root.discriminator.version.to_le_bytes(), [46u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()], bump)]
    #[account(12, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        13,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account",
        seeds = [root.discriminator.version.to_le_bytes(), [41u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account",
        seeds = [root.discriminator.version.to_le_bytes(), [42u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        15,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account",
        seeds = [root.discriminator.version.to_le_bytes(), [43u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        16,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account",
        seeds = [root.discriminator.version.to_le_bytes(), [44u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        17,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account",
        seeds = [root.discriminator.version.to_le_bytes(), [45u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        18,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account",
        seeds = [root.discriminator.version.to_le_bytes(), [48u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        19,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account",
        seeds = [root.discriminator.version.to_le_bytes(), [49u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    #[account(
        20,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account",
        seeds = [root.discriminator.version.to_le_bytes(), [50u8, 0u8, 0u8, 0u8], instrument.asset_token_id.to_le_bytes(), instrument.crncy_token_id.to_le_bytes(), deriverse_authority.key()],
        bump
    )]
    UpgradeToPerp(UpgradeToPerpData) = 10,

    /// Move funds from spot balance into perp collateral for one instrument.
    #[account(0, signer, writable, name = "signer", desc = "Perp account owner")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "system_program", desc = "System program")]
    PerpDeposit(PerpDepositData) = 11,

    /// Create a new spot order.
    #[account(0, signer, writable, name = "signer", desc = "Order owner")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "client_community", desc = "Client community account", seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(4, writable, name = "instrument", desc = "Instrument account")]
    #[account(5, writable, name = "bids_tree", desc = "Bids tree account")]
    #[account(6, writable, name = "asks_tree", desc = "Asks tree account")]
    #[account(7, writable, name = "bid_orders", desc = "Bid orders account")]
    #[account(8, writable, name = "ask_orders", desc = "Ask orders account")]
    #[account(9, writable, name = "lines", desc = "Spot lines account")]
    #[account(10, writable, name = "maps", desc = "Spot maps account")]
    #[account(
        11,
        writable,
        name = "client_infos",
        desc = "Spot client infos account"
    )]
    #[account(12, name = "community", desc = "Community account")]
    #[account(13, name = "system_program", desc = "System program")]
    NewSpotOrder(NewSpotOrderData) = 12,

    /// Cancel one existing spot order by side and id.
    #[account(0, signer, writable, name = "signer", desc = "Order owner")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "tree", desc = "Bid or ask tree based on side")]
    #[account(
        5,
        writable,
        name = "orders",
        desc = "Bid or ask order storage based on side"
    )]
    #[account(6, writable, name = "lines", desc = "Spot lines account")]
    #[account(7, writable, name = "maps", desc = "Spot maps account")]
    #[account(8, writable, name = "client_infos", desc = "Spot client infos account")]
    #[account(9, name = "system_program", desc = "System program")]
    SpotOrderCancel(SpotOrderCancelData) = 13,

    /// Trade against the internal spot LP pool.
    #[account(0, signer, name = "signer", desc = "LP trader")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    #[account(3, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(4, name = "system_program", desc = "System program")]
    SpotLp(SpotLpData) = 14,

    /// Cancel all open spot orders for one instrument.
    #[account(0, signer, writable, name = "signer", desc = "Order owner")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "bids_tree", desc = "Bids tree account")]
    #[account(5, writable, name = "asks_tree", desc = "Asks tree account")]
    #[account(6, writable, name = "bid_orders", desc = "Bid orders account")]
    #[account(7, writable, name = "ask_orders", desc = "Ask orders account")]
    #[account(8, writable, name = "lines", desc = "Spot lines account")]
    #[account(9, writable, name = "maps", desc = "Spot maps account")]
    #[account(
        10,
        writable,
        name = "client_infos",
        desc = "Spot client infos account"
    )]
    #[account(11, name = "community", desc = "Community account")]
    #[account(12, name = "system_program", desc = "System program")]
    SpotMassCancel(SpotMassCancelData) = 15,

    /// Advance to the next governance voting topic once the previous session is finalized.
    #[account(0, signer, name = "signer", desc = "Voting advancement authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "community", desc = "Community account")]
    NextVoting = 16,

    // 17, 18 reserved
    /// Create a new perp order.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "client_community", desc = "Client community account", seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(4, writable, name = "instrument", desc = "Instrument account")]
    #[account(5, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(6, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        7,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        8,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(9, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(10, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        11,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        16,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        18,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(19, name = "community", desc = "Community account")]
    #[account(20, name = "system_program", desc = "System program")]
    NewPerpOrder(NewPerpOrderData) = 19,

    // 20-24 reserved
    /// Sweep protocol fees from one or more instruments into community dividend records.
    #[account(
        0,
        name = "authority",
        desc = "Caller account carried through the instruction"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "community", desc = "Community account")]
    #[account(
        3,
        writable,
        name = "instrument",
        desc = "First instrument account to sweep; additional instrument accounts may follow"
    )]
    DividendsAllocation = 25,

    /// Swap using the system engine without creating a Deriverse client account.
    #[account(0, signer, writable, name = "signer", desc = "Swap initiator")]
    #[account(1, name = "asset_mint", desc = "Asset mint account")]
    #[account(2, name = "currency_mint", desc = "Currency mint account")]
    #[account(
        3,
        writable,
        name = "asset_vault_token_account",
        desc = "Program asset token vault",
        seeds = [asset_mint.key(), instrument.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(
        4,
        writable,
        name = "currency_vault_token_account",
        desc = "Program currency token vault",
        seeds = [currency_mint.key(), instrument.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(5, writable, name = "instrument", desc = "Instrument account")]
    #[account(
        6,
        writable,
        name = "tree",
        desc = "Bid or ask tree, depending on input direction"
    )]
    #[account(
        7,
        writable,
        name = "orders",
        desc = "Bid or ask order storage, depending on input direction"
    )]
    #[account(8, writable, name = "lines", desc = "Spot lines account")]
    #[account(9, writable, name = "maps", desc = "Spot maps account")]
    #[account(
        10,
        writable,
        name = "client_infos",
        desc = "Spot client infos account"
    )]
    #[account(
        11,
        writable,
        name = "client_asset_token_account",
        desc = "Client ATA for the asset mint"
    )]
    #[account(
        12,
        writable,
        name = "client_currency_token_account",
        desc = "Client ATA for the currency mint"
    )]
    #[account(
        13,
        name = "asset_token_program",
        desc = "Token program for the asset mint"
    )]
    Swap(SwapData) = 26,

    /// Airdrop DRVS into an existing client account using the configured authority.
    #[account(
        0,
        signer,
        name = "airdrop_authority",
        desc = "Configured airdrop authority"
    )]
    #[account(
        1,
        writable,
        name = "authority_associated_token_account",
        desc = "Authority source token account",
        seeds = [airdrop_authority.key(), token_program.key(), drvs_mint.key()],
        bump
    )]
    #[account(
        2,
        name = "wallet",
        desc = "Wallet that owns the target client account"
    )]
    #[account(3, writable, name = "root", desc = "Root PDA")]
    #[account(4, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], wallet.key()], bump)]
    #[account(
        5,
        writable,
        name = "client_community",
        desc = "Client community account",
        seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], wallet.key()],
        bump
    )]
    #[account(6, name = "drvs_mint", desc = "DRVS mint account")]
    #[account(7, name = "drvs_token", desc = "Token metadata PDA for DRVS")]
    #[account(
        8,
        writable,
        name = "drvs_program_token_account",
        desc = "Program DRVS token vault",
        seeds = [drvs_mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(9, writable, name = "community", desc = "Community account")]
    #[account(10, name = "system_program", desc = "System program")]
    #[account(11, name = "token_program", desc = "Token program id")]
    Airdrop(AirdropData) = 27,

    /// Claim accumulated community dividends into the client account state.
    #[account(0, signer, name = "signer", desc = "Dividend claimant")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "community", desc = "Community account")]
    #[account(3, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(
        4,
        writable,
        name = "client_community",
        desc = "Client community account",
        seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()],
        bump
    )]
    #[account(5, name = "system_program", desc = "System program")]
    DividendsClaim = 28,

    // 29 reserved
    /// Cancel one existing perp order by side and id.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    PerpOrderCancel(PerpOrderCancelData) = 30,

    // 31 reserved
    /// Vote on the current community governance topic.
    #[account(0, signer, name = "signer", desc = "Voter authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "community", desc = "Community account")]
    #[account(
        4,
        writable,
        name = "client_community",
        desc = "Client community account",
        seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()],
        bump
    )]
    #[account(5, name = "system_program", desc = "System program")]
    Voting(VotingData) = 32,

    // 33 reserved
    /// Replace a set of spot quotes using a packed header plus packed quote tail.
    #[account(0, signer, writable, name = "signer", desc = "Quote owner")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "client_community", desc = "Client community account", seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(4, writable, name = "instrument", desc = "Instrument account")]
    #[account(5, writable, name = "bids_tree", desc = "Bids tree account")]
    #[account(6, writable, name = "asks_tree", desc = "Asks tree account")]
    #[account(7, writable, name = "bid_orders", desc = "Bid orders account")]
    #[account(8, writable, name = "ask_orders", desc = "Ask orders account")]
    #[account(9, writable, name = "lines", desc = "Spot lines account")]
    #[account(10, writable, name = "maps", desc = "Spot maps account")]
    #[account(
        11,
        writable,
        name = "client_infos",
        desc = "Spot client infos account"
    )]
    #[account(12, name = "community", desc = "Community account")]
    #[account(13, name = "system_program", desc = "System program")]
    SpotQuotesReplace(SpotQuotesReplaceData) = 34,

    // 35 reserved
    /// Cancel all currently open perp orders for one instrument.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    PerpMassCancel(PerpMassCancelData) = 36,

    /// Change the leverage setting used for one perp client position.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    PerpChangeLeverage(PerpChangeLeverageData) = 37,

    // 38 reserved
    /// Withdraw previously prepaid community fees back into the client balance state.
    #[account(0, signer, name = "signer", desc = "Client wallet")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "community", desc = "Community account")]
    #[account(4, writable, name = "client_community", desc = "Client community account", seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(5, name = "system_program", desc = "System program")]
    FeesWithdraw(FeesWithdrawData) = 39,

    // 40 reserved
    /// Mark one non-perp instrument as ready for a later upgrade_to_perp call.
    #[account(0, signer, writable, name = "admin", desc = "Operator authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    SetInstrReadyForPerpUpgrade(SetInstrReadyForPerpUpgradeData) = 41,

    /// Replace a set of perp quotes using a packed header plus packed quote tail.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "client_community", desc = "Client community account", seeds = [root.discriminator.version.to_le_bytes(), [35u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(4, writable, name = "instrument", desc = "Instrument account")]
    #[account(5, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(6, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        7,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        8,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(9, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(10, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        11,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        16,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        18,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(19, name = "community", desc = "Community account")]
    #[account(20, name = "system_program", desc = "System program")]
    PerpQuotesReplace(PerpQuotesReplaceData) = 42,

    /// Move available spot funds from temporary client-info storage into the wallet account state.
    #[account(0, signer, writable, name = "signer", desc = "Client wallet")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "maps", desc = "Spot maps account")]
    #[account(4, writable, name = "client_infos", desc = "Spot client infos account")]
    #[account(5, name = "system_program", desc = "System program")]
    MoveSpotAvailFunds(MoveSpotAvailFundsData) = 43,

    /// Update referral-program durations and fee split parameters.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    ChangeRefProgram(ChangeRefProgramData) = 44,

    /// Create or refresh one of the two referral-link slots owned by the caller-authorized client primary.
    #[account(0, signer, name = "signer", desc = "Referral-link owner authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    NewRefLink = 45,

    /// Reset one client's accumulated perp statistics before running the usual perp maintenance path.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    PerpStatisticsReset(PerpStatisticsResetData) = 46,

    /// Buy a perp market seat for one instrument, optionally depositing extra perp collateral.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    BuyMarketSeat(BuyMarketSeatData) = 47,

    /// Sell a perp market seat and close the perp client account when allowed.
    #[account(0, signer, writable, name = "signer", desc = "Perp trader")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, writable, name = "instrument", desc = "Instrument account")]
    #[account(4, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(5, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        6,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        7,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(8, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(9, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        10,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        15,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        17,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(18, name = "community", desc = "Community account")]
    #[account(19, name = "system_program", desc = "System program")]
    SellMarketSeat(SellMarketSeatData) = 48,

    /// Add a wallet to the private-mode queue.
    #[account(
        0,
        signer,
        name = "private_mode_authority",
        desc = "Private mode authority"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(
        2,
        writable,
        name = "private_clients",
        desc = "Private clients queue account"
    )]
    #[account(3, name = "system_program", desc = "System program")]
    #[account(4, name = "wallet", desc = "Wallet being added to the private queue")]
    #[account(
        5,
        name = "client_primary",
        desc = "Derived client primary PDA for the wallet",
        seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], wallet.key()],
        bump
    )]
    NewPrivateClient(NewPrivateClient) = 49,

    /// Terminate private mode and drain the private-clients queue account back to the operator.
    #[account(0, signer, writable, name = "admin", desc = "Operator authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(
        2,
        writable,
        name = "private_clients",
        desc = "Private clients queue account"
    )]
    TerminatePrivateMode = 50,

    /// Extend the points-program expiration timestamp.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    ChangePointsProgramExpiration(PointsProgramExpiration) = 51,

    /// Change the configured airdrop authority address.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "airdrop_authority", desc = "New airdrop authority address")]
    #[account(2, writable, name = "root", desc = "Root PDA")]
    ChangeAirdropAuthority = 52,

    /// Change the configured private-mode authority address.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(
        1,
        name = "private_mode_authority",
        desc = "New private-mode authority address"
    )]
    #[account(2, writable, name = "root", desc = "Root PDA")]
    ChangePrivateModeAuthority = 53,

    /// Set an instrument variance value.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    SetVariance(SetVarianceData) = 54,

    /// Reset launch-era governance parameters back to their starting values.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "community", desc = "Community account")]
    VotingReset = 55,

    /// Change the denominator used by one base-currency record.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "community", desc = "Community account")]
    ChangeDenominator(ChangeDenominatorData) = 56,

    /// Run the periodic perp maintenance path for one instrument.
    #[account(0, signer, writable, name = "signer", desc = "Maintenance caller")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    #[account(3, writable, name = "perp_bids_tree", desc = "Perp bids tree account")]
    #[account(4, writable, name = "perp_asks_tree", desc = "Perp asks tree account")]
    #[account(
        5,
        writable,
        name = "perp_bid_orders",
        desc = "Perp bid orders account"
    )]
    #[account(
        6,
        writable,
        name = "perp_ask_orders",
        desc = "Perp ask orders account"
    )]
    #[account(7, writable, name = "perp_lines", desc = "Perp lines account")]
    #[account(8, writable, name = "perp_maps", desc = "Perp maps account")]
    #[account(
        9,
        writable,
        name = "perp_client_infos",
        desc = "Perp client infos account"
    )]
    #[account(
        10,
        writable,
        name = "perp_client_infos2",
        desc = "Perp client infos2 account"
    )]
    #[account(
        11,
        writable,
        name = "perp_client_infos3",
        desc = "Perp client infos3 account"
    )]
    #[account(
        12,
        writable,
        name = "perp_client_infos4",
        desc = "Perp client infos4 account"
    )]
    #[account(
        13,
        writable,
        name = "perp_client_infos5",
        desc = "Perp client infos5 account"
    )]
    #[account(
        14,
        writable,
        name = "perp_long_px_tree",
        desc = "Perp long price tree account"
    )]
    #[account(
        15,
        writable,
        name = "perp_short_px_tree",
        desc = "Perp short price tree account"
    )]
    #[account(
        16,
        writable,
        name = "perp_rebalance_time_tree",
        desc = "Perp rebalance time tree account"
    )]
    #[account(17, name = "community", desc = "Community account")]
    #[account(18, name = "system_program", desc = "System program")]
    PerpClientsProcessing(PerpClientsProcessingData) = 57,

    /// Set the market-seat purchasing fee stored on root state.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    SetSeatPurchasingFee(SetSeatPurchasingFeeData) = 58,

    /// Change an already-cast vote on the current governance topic.
    #[account(0, signer, name = "signer", desc = "Voter authority")]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account")]
    #[account(3, writable, name = "community", desc = "Community account")]
    #[account(
        4,
        writable,
        name = "client_community",
        desc = "Client community account"
    )]
    #[account(5, name = "system_program", desc = "System program")]
    ChangeVoting(ChangeVotingData) = 59,

    /// Finalize temporary spot balances for one or more client accounts and clean reusable spot records.
    #[account(0, signer, name = "signer", desc = "Collector authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, name = "maps", desc = "Spot maps account")]
    #[account(3, name = "client_infos", desc = "Spot client infos account")]
    #[account(4, name = "system_program", desc = "System program")]
    #[account(
        5,
        writable,
        name = "client_primary",
        desc = "First client primary account to finalize; additional client primary accounts may follow"
    )]
    GarbageCollector(GarbageCollectorData) = 60,

    /// Activate the referral program for a client using an existing referral link.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Referral program activation authority"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(
        2,
        writable,
        name = "client_primary",
        desc = "Client primary account receiving the referral relationship"
    )]
    #[account(
        3,
        writable,
        name = "ref_client_primary",
        desc = "Existing client primary account that owns the referral link being activated"
    )]
    ActivateClientRefProgram(ActivateClientRefProgramData) = 61,

    /// Reinitialize candle storage for an instrument inside the maps account.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    #[account(
        3,
        writable,
        name = "maps",
        desc = "Spot maps account containing candle storage"
    )]
    CleanCandles(CleanCandlesData) = 62,

    /// Start VM activation by assigning a dedicated VM authority.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Main wallet that owns the client account"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, name = "vm_mode_authority", desc = "Future VM authority wallet")]
    VmInitActivate(VmInitActivateData) = 63,

    /// Cancel a pending VM activation before it is finalized.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Main wallet that owns the client account"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    VmInitActivateCancel = 64,

    /// Finalize VM activation from the assigned VM authority wallet.
    #[account(0, signer, name = "signer", desc = "VM authority wallet")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    VmFinalizeActivate = 65,

    /// Start VM deactivation from the main wallet side.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Main wallet that owns the client account"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    VmInitDeactivate = 66,

    /// Cancel a pending VM deactivation before finalization.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Main wallet that owns the client account"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    VmInitDeactivateCancel = 67,

    /// Finalize VM deactivation from the VM authority wallet.
    #[account(0, signer, name = "signer", desc = "VM authority wallet")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    VmFinalizeDeactivate = 68,

    /// Begin a VM withdrawal by moving balance into the client header's pending withdraw fields.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Main wallet that owns the client account"
    )]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, name = "system_program", desc = "System program")]
    VmInitWithdraw(VmInitWithdrawData) = 69,

    /// Cancel a pending VM withdrawal and move the reserved balance back into the client state.
    #[account(
        0,
        signer,
        name = "signer",
        desc = "Main wallet that owns the client account"
    )]
    #[account(1, writable, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(3, name = "system_program", desc = "System program")]
    VmInitWithdrawCancel = 70,

    /// Finalize a pending VM withdrawal into the configured VM wallet token account.
    #[account(0, signer, name = "vm_mode_authority", desc = "VM authority wallet")]
    #[account(
        1,
        writable,
        name = "vm_mode_authority_token_account",
        desc = "Destination token account for the VM authority"
    )]
    #[account(
        2,
        writable,
        name = "program_token_account",
        desc = "Program token vault",
        seeds = [mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(3, name = "mint", desc = "Token mint")]
    #[account(4, name = "root", desc = "Root PDA")]
    #[account(5, name = "token", desc = "Token metadata PDA")]
    #[account(6, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], vm_mode_authority.key()], bump)]
    #[account(7, name = "system_program", desc = "System program")]
    #[account(8, name = "token_program", desc = "Token program id")]
    #[account(
        9,
        name = "associated_token_program",
        desc = "Associated token program placeholder consumed by the current handler"
    )]
    VmInitWithdrawFinalize = 71,

    /// Replace the VM whitelist instrument slots.
    #[account(0, signer, name = "signer", desc = "VM authority wallet")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    VmChangeWhitelist(VmChangeWhitelistData) = 72,

    /// Extend the spot maps account with a fresh candles header and candles storage block.
    #[account(
        0,
        signer,
        writable,
        name = "signer",
        desc = "Funding authority for the reallocation"
    )]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    #[account(
        3,
        writable,
        name = "maps",
        desc = "Spot maps account extended with candles storage"
    )]
    #[account(4, name = "system_program", desc = "System program")]
    ExtendCandles(ExtendCandlesData) = 73,

    /// Withdraw swap fees from one instrument's currency vault.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(
        1,
        writable,
        name = "client_token_account",
        desc = "Destination token account"
    )]
    #[account(
        2,
        writable,
        name = "program_token_account",
        desc = "Program token vault",
        seeds = [mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(3, name = "mint", desc = "Currency mint")]
    #[account(4, name = "root", desc = "Root PDA")]
    #[account(5, writable, name = "instrument", desc = "Instrument account")]
    #[account(6, name = "token", desc = "Currency token metadata PDA")]
    #[account(7, name = "token_program", desc = "Token program id")]
    WithdrawSwapFees(WithdrawSwapFeesData) = 74,

    /// Set the minimum SAM quantity for one similar-assets instrument.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    SetSAMMinQty(SetSAMMinQtyData) = 75,

    /// Change the SAM fee policy for one similar-assets instrument.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    ChangeSAMFeesPolicy(ChangeSAMFeesPolicyData) = 76,

    /// Suspend one admin-created non-perp instrument.
    #[account(0, signer, name = "admin", desc = "Operator authority")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "instrument", desc = "Instrument account")]
    #[account(3, name = "community", desc = "Community account")]
    SuspendInstrument(SuspendInstrumentData) = 77,

    /// Add a withdrawal token authority to the VM whitelist.
    #[account(0, signer, name = "signer", desc = "VM authority wallet")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(
        3,
        writable,
        name = "client_vm_account",
        desc = "Client VM whitelist account",
        seeds = [root.discriminator.version.to_le_bytes(), [52u8, 0u8, 0u8, 0u8], signer.key()],
        bump
    )]
    #[account(
        4,
        name = "withdrawal_token_authority",
        desc = "Whitelisted withdrawal token authority"
    )]
    AddWithdrawalAddress = 78,

    /// Remove a withdrawal token authority from the VM whitelist.
    #[account(0, signer, name = "signer", desc = "VM authority wallet")]
    #[account(1, name = "root", desc = "Root PDA")]
    #[account(2, writable, name = "client_primary", desc = "Client primary account")]
    #[account(
        3,
        writable,
        name = "client_vm_account",
        desc = "Client VM whitelist account"
    )]
    #[account(
        4,
        name = "withdrawal_token_authority",
        desc = "Whitelisted withdrawal token authority"
    )]
    RemoveWithdrawalAddress = 79,

    /// Withdraw directly to a pre-whitelisted VM withdrawal address.
    #[account(0, signer, name = "signer", desc = "VM authority wallet")]
    #[account(
        1,
        writable,
        name = "withdrawal_token_account",
        desc = "Whitelisted withdrawal token account"
    )]
    #[account(
        2,
        writable,
        name = "program_token_account",
        desc = "Program token vault",
        seeds = [mint.key(), root.discriminator.version.to_le_bytes()],
        bump
    )]
    #[account(3, name = "mint", desc = "Token mint")]
    #[account(4, writable, name = "root", desc = "Root PDA")]
    #[account(5, name = "token", desc = "Token metadata PDA")]
    #[account(6, writable, name = "client_primary", desc = "Client primary account", seeds = [root.discriminator.version.to_le_bytes(), [31u8, 0u8, 0u8, 0u8], signer.key()], bump)]
    #[account(7, name = "client_vm_account", desc = "Client VM whitelist account")]
    #[account(8, name = "system_program", desc = "System program")]
    #[account(9, name = "token_program", desc = "Token program id")]
    VmDirectWithdraw(VmDirectWithdrawData) = 80,
    // VmAddKamino = 81,          // not yet listed
    // VmRemoveKamino = 82,       // not yet listed
    // KaminoInitObligation = 83, // not yet listed
    // KaminoInitInstrument = 84, // not yet listed
    // KaminoChangePosition = 85, // not yet listed
    // CloseAccount = 86,         // not yet listed
    // SetForeignDeposit = 87,    // not yet listed
}

impl DrvInstructionIdl {
    pub fn tag(&self) -> u8 {
        // SAFETY: #[repr(u8)] guarantees the discriminant is stored as the first byte.
        unsafe { *(self as *const Self).cast::<u8>() }
    }

    pub fn instruction_number(&self) -> u8 {
        self.tag()
    }

    pub fn min_accounts_amount(&self) -> usize {
        match self {
            Self::NewHolder => 3,
            Self::NewOperator(_) => 4,
            Self::NewRootAccount(_) => 12,
            Self::PerpWithdraw(_) => 20,
            Self::NewBaseCrncy(_) => 8,
            Self::FeesDeposit(_) => 6,
            Self::Deposit(_) => 9,
            Self::Withdraw(_) => 9,
            Self::NewInstrument(_) => 19,
            Self::UpgradeToPerp(_) => 21,
            Self::PerpDeposit(_) => 19,
            Self::NewSpotOrder(_) => 14,
            Self::SpotOrderCancel(_) => 10,
            Self::SpotLp(_) => 5,
            Self::SpotMassCancel(_) => 13,
            Self::NextVoting => 3,
            Self::NewPerpOrder(_) => 21,
            Self::DividendsAllocation => 4,
            Self::Swap(_) => 14,
            Self::Airdrop(_) => 12,
            Self::DividendsClaim => 6,
            Self::PerpOrderCancel(_) => 20,
            Self::Voting(_) => 6,
            Self::SpotQuotesReplace(_) => 14,
            Self::PerpMassCancel(_) => 20,
            Self::PerpChangeLeverage(_) => 20,
            Self::FeesWithdraw(_) => 6,
            Self::SetInstrReadyForPerpUpgrade(_) => 3,
            Self::PerpQuotesReplace(_) => 21,
            Self::MoveSpotAvailFunds(_) => 6,
            Self::ChangeRefProgram(_) => 2,
            Self::NewRefLink => 3,
            Self::PerpStatisticsReset(_) => 20,
            Self::BuyMarketSeat(_) => 20,
            Self::SellMarketSeat(_) => 20,
            Self::NewPrivateClient(_) => 6,
            Self::TerminatePrivateMode => 3,
            Self::ChangePointsProgramExpiration(_) => 2,
            Self::ChangeAirdropAuthority => 3,
            Self::ChangePrivateModeAuthority => 3,
            Self::SetVariance(_) => 3,
            Self::VotingReset => 3,
            Self::ChangeDenominator(_) => 3,
            Self::PerpClientsProcessing(_) => 19,
            Self::SetSeatPurchasingFee(_) => 2,
            Self::ChangeVoting(_) => 6,
            Self::GarbageCollector(_) => 6,
            Self::ActivateClientRefProgram(_) => 4,
            Self::CleanCandles(_) => 4,
            Self::VmInitActivate(_) => 4,
            Self::VmInitActivateCancel => 3,
            Self::VmFinalizeActivate => 3,
            Self::VmInitDeactivate => 3,
            Self::VmInitDeactivateCancel => 3,
            Self::VmFinalizeDeactivate => 3,
            Self::VmInitWithdraw(_) => 4,
            Self::VmInitWithdrawCancel => 4,
            Self::VmInitWithdrawFinalize => 10,
            Self::VmChangeWhitelist(_) => 3,
            Self::ExtendCandles(_) => 5,
            Self::WithdrawSwapFees(_) => 11,
            Self::SetSAMMinQty(_) => 3,
            Self::ChangeSAMFeesPolicy(_) => 3,
            Self::SuspendInstrument(_) => 4,
            Self::AddWithdrawalAddress => 5,
            Self::RemoveWithdrawalAddress => 5,
            Self::VmDirectWithdraw(_) => 3,
        }
    }
}

impl TryFrom<u8> for DrvInstructionIdl {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NewHolder),
            1 => Ok(Self::NewOperator(Zeroable::zeroed())),
            2 => Ok(Self::NewRootAccount(Zeroable::zeroed())),
            3 => Ok(Self::PerpWithdraw(Zeroable::zeroed())),
            4 => Ok(Self::NewBaseCrncy(Zeroable::zeroed())),
            5 => Ok(Self::FeesDeposit(Zeroable::zeroed())),
            7 => Ok(Self::Deposit(Zeroable::zeroed())),
            8 => Ok(Self::Withdraw(Zeroable::zeroed())),
            9 => Ok(Self::NewInstrument(Zeroable::zeroed())),
            10 => Ok(Self::UpgradeToPerp(Zeroable::zeroed())),
            11 => Ok(Self::PerpDeposit(Zeroable::zeroed())),
            12 => Ok(Self::NewSpotOrder(Zeroable::zeroed())),
            13 => Ok(Self::SpotOrderCancel(Zeroable::zeroed())),
            14 => Ok(Self::SpotLp(Zeroable::zeroed())),
            15 => Ok(Self::SpotMassCancel(Zeroable::zeroed())),
            16 => Ok(Self::NextVoting),
            19 => Ok(Self::NewPerpOrder(Zeroable::zeroed())),
            25 => Ok(Self::DividendsAllocation),
            26 => Ok(Self::Swap(Zeroable::zeroed())),
            27 => Ok(Self::Airdrop(Zeroable::zeroed())),
            28 => Ok(Self::DividendsClaim),
            30 => Ok(Self::PerpOrderCancel(Zeroable::zeroed())),
            32 => Ok(Self::Voting(Zeroable::zeroed())),
            34 => Ok(Self::SpotQuotesReplace(Zeroable::zeroed())),
            36 => Ok(Self::PerpMassCancel(Zeroable::zeroed())),
            37 => Ok(Self::PerpChangeLeverage(Zeroable::zeroed())),
            39 => Ok(Self::FeesWithdraw(Zeroable::zeroed())),
            41 => Ok(Self::SetInstrReadyForPerpUpgrade(Zeroable::zeroed())),
            42 => Ok(Self::PerpQuotesReplace(Zeroable::zeroed())),
            43 => Ok(Self::MoveSpotAvailFunds(Zeroable::zeroed())),
            44 => Ok(Self::ChangeRefProgram(Zeroable::zeroed())),
            45 => Ok(Self::NewRefLink),
            46 => Ok(Self::PerpStatisticsReset(Zeroable::zeroed())),
            47 => Ok(Self::BuyMarketSeat(Zeroable::zeroed())),
            48 => Ok(Self::SellMarketSeat(Zeroable::zeroed())),
            49 => Ok(Self::NewPrivateClient(Zeroable::zeroed())),
            50 => Ok(Self::TerminatePrivateMode),
            51 => Ok(Self::ChangePointsProgramExpiration(Zeroable::zeroed())),
            52 => Ok(Self::ChangeAirdropAuthority),
            53 => Ok(Self::ChangePrivateModeAuthority),
            54 => Ok(Self::SetVariance(Zeroable::zeroed())),
            55 => Ok(Self::VotingReset),
            56 => Ok(Self::ChangeDenominator(Zeroable::zeroed())),
            57 => Ok(Self::PerpClientsProcessing(Zeroable::zeroed())),
            58 => Ok(Self::SetSeatPurchasingFee(Zeroable::zeroed())),
            59 => Ok(Self::ChangeVoting(Zeroable::zeroed())),
            60 => Ok(Self::GarbageCollector(Zeroable::zeroed())),
            61 => Ok(Self::ActivateClientRefProgram(Zeroable::zeroed())),
            62 => Ok(Self::CleanCandles(Zeroable::zeroed())),
            63 => Ok(Self::VmInitActivate(Zeroable::zeroed())),
            64 => Ok(Self::VmInitActivateCancel),
            65 => Ok(Self::VmFinalizeActivate),
            66 => Ok(Self::VmInitDeactivate),
            67 => Ok(Self::VmInitDeactivateCancel),
            68 => Ok(Self::VmFinalizeDeactivate),
            69 => Ok(Self::VmInitWithdraw(Zeroable::zeroed())),
            70 => Ok(Self::VmInitWithdrawCancel),
            71 => Ok(Self::VmInitWithdrawFinalize),
            72 => Ok(Self::VmChangeWhitelist(Zeroable::zeroed())),
            73 => Ok(Self::ExtendCandles(Zeroable::zeroed())),
            74 => Ok(Self::WithdrawSwapFees(Zeroable::zeroed())),
            75 => Ok(Self::SetSAMMinQty(Zeroable::zeroed())),
            76 => Ok(Self::ChangeSAMFeesPolicy(Zeroable::zeroed())),
            77 => Ok(Self::SuspendInstrument(Zeroable::zeroed())),
            78 => Ok(Self::AddWithdrawalAddress),
            79 => Ok(Self::RemoveWithdrawalAddress),
            80 => Ok(Self::VmDirectWithdraw(Zeroable::zeroed())),
            _ => Err(value),
        }
    }
}
