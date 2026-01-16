use std::error;

use drv_errors_derive::DrvError;
use drv_models::{
    constants::TradingSection,
    new_types::instrument::InstrId,
    state::types::{
        account_type::AccountType, vm_status::VmFlag, AssetType, OrderSide, OrderType, TokenProgram,
    },
};
use serde::{Deserialize, Serialize};
#[cfg(feature = "on-chain")]
use solana_program::{msg, program_error::ProgramError, pubkey::Pubkey};
#[cfg(feature = "off-chain")]
use solana_sdk::pubkey::Pubkey;
#[cfg(not(any(feature = "on-chain", feature = "off-chain")))]
compile_error!("Either 'on-chain' or 'off-chain' feature must be enabled");

pub trait ResultExt<T, E> {
    fn context<C>(self, ctx: C) -> Result<T, C>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn context<C>(self, ctx: C) -> Result<T, C> {
        self.map_err(|_| ctx)
    }
}
#[cfg(feature = "on-chain")]
impl From<ProgramError> for DeriverseErrorKind {
    fn from(e: ProgramError) -> Self {
        DeriverseErrorKind::SystemError { error: e }
    }
}

#[cfg(feature = "on-chain")]
impl From<DeriverseErrorKind> for ProgramError {
    fn from(e: DeriverseErrorKind) -> Self {
        msg!("{}", e.to_json().to_string());
        ProgramError::Custom(e.code())
    }
}

#[cfg(feature = "on-chain")]
impl From<DeriverseError> for ProgramError {
    fn from(e: DeriverseError) -> Self {
        msg!("{}", e.to_json().to_string());
        ProgramError::Custom(e.code())
    }
}

#[cfg(feature = "off-chain")]
impl From<DeriverseError> for u32 {
    fn from(e: DeriverseError) -> Self {
        e.code()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorLocation {
    pub file: &'static str,
    pub line: u32,
}

#[derive(Debug)]
pub struct DeriverseError {
    pub error: DeriverseErrorKind,
    pub location: ErrorLocation,
}

pub type DeriverseResult = Result<(), DeriverseError>;

impl DeriverseError {
    pub fn new(error: DeriverseErrorKind, file: &'static str, line: u32) -> Self {
        Self {
            error,
            location: ErrorLocation { file, line },
        }
    }

    pub fn code(&self) -> u32 {
        self.error.code()
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut json = self.error.to_json();
        json["location"] = serde_json::json!({
            "file": self.location.file,
            "line": self.location.line
        });
        json
    }
}

#[macro_export]
macro_rules! drv_err {
    ($error:expr) => {
        ::drv_errors::errors::DeriverseError::new($error, file!(), line!())
    };
}

#[macro_export]
macro_rules! bail {
    ($error:expr) => {
        return Err(::drv_errors::errors::DeriverseError::new(
            $error,
            file!(),
            line!(),
        ))
    };
}

#[macro_export]
macro_rules! drv_result {
    ($error:expr) => {
        Err(::drv_errors::errors::DeriverseError::new(
            $error,
            file!(),
            line!(),
        ))
    };
}

#[test]
fn some_test() {
    let error = DeriverseErrorKind::InvalidMintAccount {
        token_id: 10,
        expected_address: Pubkey::new_unique(),
        actual_address: Pubkey::new_unique(),
    };

    println!("{:?}", error.to_json());
}

#[derive(Debug, DrvError, Serialize, Deserialize, PartialEq)]
pub enum DeriverseErrorKind {
    #[cfg(feature = "on-chain")]
    #[error(code = 100, msg = "System error {error}")]
    SystemError { error: ProgramError },

    #[error(
        code = 101,
        msg = "Invalid provided accounts number: expected {expected}, got {actual}"
    )]
    InvalidAccountsNumber { expected: usize, actual: usize },

    #[error(
        code = 102,
        msg = "Invalid instructions data length, expected {expected} < actual {actual}"
    )]
    InvalidDataLength { expected: usize, actual: usize },

    #[error(code = 104, msg = "{address} must be signer")]
    MustBeSigner { address: Pubkey },

    #[error(code = 246, msg = "{address} must be read only")]
    MustBeReadOnly { address: Pubkey },

    #[error(code = 246, msg = "{address} must be writable")]
    MustBeWritable { address: Pubkey },
    #[error(
        code = 122,
        msg = "Invalid token program ID, expected {expected} != actual {actual}"
    )]
    InvalidTokenProgramId {
        expected: TokenProgram,
        actual: TokenProgram,
    },
    #[error(code = 123, msg = "Unsupported token program id")]
    UnsupportedTokenProgramId { address: Pubkey },

    #[error(code = 124, msg = "Invalid mint address for token {token_id}")]
    InvalidMintAccount {
        token_id: u32,
        expected_address: Pubkey,
        actual_address: Pubkey,
    },
    #[error(
        code = 126,
        msg = "Invalid mint token program, expected {expected} != actual {actual}"
    )]
    InvalidMintProgramId {
        expected: TokenProgram,
        actual: TokenProgram,
        mint_address: Pubkey,
    },
    #[error(code = 127, msg = "Invalid LUT program ID")]
    InvalidLutProgramId { actual_address: Pubkey },

    #[error(code = 128, msg = "Invalid LUT account")]
    InvalidLutAccount {
        expected_address: Pubkey,
        actual_address: Pubkey,
    },

    #[error(code = 129, msg = "Invalid System program ID")]
    InvalidSystemProgramId { actual_address: Pubkey },
    #[error(
        code = 130,
        msg = "Invalid quantity value {value}, acceptable range: {min_value}..{max_value}"
    )]
    InvalidQuantity {
        value: i64,
        min_value: i64,
        max_value: i64,
    },
    #[error(
        code = 131,
        msg = "Invalid price value {price}, acceptable range: {min_price}..{max_price}"
    )]
    InvalidPrice {
        price: i64,
        min_price: i64,
        max_price: i64,
    },
    #[error(code = 132, msg = "Insufficient funds")]
    InsufficientFunds,

    #[error(
        code = 136,
        msg = "Too many lines on {side} side provided value is {amount}"
    )]
    TooManyLines { side: OrderSide, amount: u32 },
    #[error(code = 137, msg = "Allocator failed")]
    AllocatorFailed,

    #[error(code = 137, msg = "Order allocator failed, side: {side}")]
    OrderAllocatorFailed { side: OrderSide },
    #[error(
        code = 138,
        msg = "Attempted to trade with yourself, order id: {order_id}"
    )]
    CrossOrder { order_id: i64, qty: i64, sum: i64 },

    #[error(code = 139, msg = "Engine matching failed")]
    MatchingEngineFailed,

    #[error(code = 141, msg = "Invalid PDA for {account_type}")]
    InvalidPDA {
        expected_address: Pubkey,
        actual_address: Pubkey,
        account_type: AccountType,
    },

    #[error(
        code = 144,
        msg = "Invalid new {account_type} account. Owner has to be SystemProgram"
    )]
    InvalidNewAccount {
        address: Pubkey,
        owner: Pubkey,
        account_type: AccountType,
    },
    #[error(
        code = 143,
        msg = "Invalid program {account_type} account. Owner have to be {program_id_address}"
    )]
    InvalidProgramAccount {
        address: Pubkey,
        owner: Pubkey,
        program_id_address: Pubkey,
        account_type: AccountType,
    },
    #[error(code = 146, msg = "Invalid Holder Admin account")]
    InvalidHolderAdminAccount { actual_address: Pubkey },
    #[error(code = 147, msg = "Invalid Admin account")]
    InvalidAdminAccount {
        expected_address: Pubkey,
        actual_address: Pubkey,
    },
    #[error(
        code = 148,
        msg = "Invalid new Operator account, operator with this address already exist"
    )]
    InvalidNewOperatorAccount { address: Pubkey },
    #[error(
        code = 150,
        msg = "Invalid Operator account, operator with this address does not exist"
    )]
    InvalidOperatorAccount { address: Pubkey },
    #[error(code = 151, msg = "Invalid token id {id}")]
    InvalidTokenId { id: u32 },
    #[error(
        code = 151,
        msg = "Unexpected token id, expected {expected} != actual {actual}"
    )]
    UnexpectedTokenId { expected: u32, actual: u32 },
    #[error(code = 212, msg = "Token with id {id} was not found")]
    TokenWasNotFound {
        id: u32,
        community_state_address: Pubkey,
    },
    #[error(
        code = 155,
        msg = "Invalid program's token account for {token_id} token id"
    )]
    InvalidProgramsTokenAccount {
        token_id: u32,
        expected_address: Pubkey,
        actual_address: Pubkey,
    },
    #[error(code = 166, msg = "Invalid instrument id {id}")]
    InvalidInstrId { id: u32 },

    #[error(
        code = 166,
        msg = "Invalid instrument id in {account_type:?} account, expected {expected} != actual {actual}"
    )]
    UnexpectedInstrumentId {
        expected: u32,
        actual: u32,
        address: Pubkey,
        account_type: AccountType,
    },
    #[error(
        code = 175,
        msg = "Order node on the side {side} with id {id} was not found"
    )]
    NodeWasNotFound { id: i64, side: OrderSide },

    #[error(
        code = 175,
        msg = "Order on the side {side} with link {link} was not found"
    )]
    OrderWasNotFound { link: u32, side: OrderSide },

    #[error(code = 183, msg = "Client data destruction")]
    ClientDataDestruction,
    #[error(code = 185, msg = "Token with id {id} must be base currency")]
    TokenMustBeBaseCrncy { id: u32, mask: u32 },

    #[error(code = 189, msg = "Impossible to upgrade instrument with id {id}")]
    ImpossibleToUpgrade { id: u32, mask: u32 },

    #[error(code = 190, msg = "Invalid Bid Orders count")]
    InvalidBidOrdersCount,
    #[error(code = 191, msg = "Invalid Ask Orders count")]
    InvalidAskOrdersCount,

    #[error(code = 192, msg = "Invalid Bid Lines count")]
    InvalidBidLinesCount,

    #[error(code = 193, msg = "Invalid Ask Lines count")]
    InvalidAskLinesCount,

    #[error(
        code = 196,
        msg = "Invalid associated token address for token {token_id}"
    )]
    InvalidAssociatedTokenAddress {
        token_id: u32,
        expected_address: Pubkey,
        actual_address: Pubkey,
    },
    #[error(
        code = 201,
        msg = "Too early to distribute funds, current time is {current_time} < allowed time {limit_time}"
    )]
    TooEarlyToDistribFunds { limit_time: u32, current_time: u32 },

    #[error(
        code = 203,
        msg = "Insufficient Deriverse tokens supply, amount {amount} < min_amount {min_amount}"
    )]
    InsufficientDeriverseTokensSupply { amount: i64, min_amount: i64 },

    #[error(code = 204, msg = "Invalid client Id in client community account")]
    InvalidClientId {
        address: Pubkey,
        expected: u32,
        actual: u32,
    },
    #[error(
        code = 205,
        msg = "Invalid voting counter, expected {expected} != actual {actual}"
    )]
    InvalidVotingCounter { actual: u32, expected: u32 },

    #[error(code = 206, msg = "Already voted")]
    AlreadyVoted,

    #[error(code = 210, msg = "No Trade (IOC)")]
    NoTradeIOC,

    #[error(code = 212, msg = "Asset {asset_type} with id {id} was not found")]
    AssetNotFound { asset_type: AssetType, id: u32 },

    #[error(code = 214, msg = "Null Pointer")]
    NullPointer,

    #[error(code = 217, msg = "Community account has to be read only")]
    CommunityAccountHasToBeReadOnly { address: Pubkey },

    #[error(code = 218, msg = "Invalid token type")]
    InvalidTokenType,

    #[error(code = 219, msg = "Null Index")]
    NullIndex,
    #[error(code = 223, msg = "Arithmetic overflow")]
    ArithmeticOverflow,

    #[error(code = 224, msg = "Invalid data format")]
    InvalidClientDataFormat,

    #[error(
        code = 225,
        msg = "Invalid order id {value}, acceptable range: {min_value}.."
    )]
    InvalidOrderId {
        value: i64,
        min_value: i64,
        max_value: i64,
    },
    #[error(code = 226, msg = "Perp on instrument id {id} is not available")]
    PerpIsNotAvailable { id: u32, mask: u32 },

    #[error(code = 229, msg = "Impossible to withdraw funds during margin call")]
    ImpossibleToWithdrawFundsDuringMarginCall,

    #[error(code = 230, msg = "Perp client count can not be zero")]
    PerpClientCountCanNotBeZero,

    #[error(code = 231, msg = "Max perp clients count reached")]
    MaxPerpClientsCountReached { value: u32, max: u32 },

    #[error(
        code = 232,
        msg = "Invalid leverage, leverage {value} > max leverage {max} or equal to 0"
    )]
    InvalidLeverage { value: u32, max: u32 },
    #[error(code = 233, msg = "Invalid Socialized Loss Open Interest")]
    InvalidSocializedLossOpenInterest,

    #[error(code = 234, msg = "Impossible to close perp position")]
    ImpossibleToClosePerpPosition,

    #[error(
        code = 235,
        msg = "Too early to withdraw fees, current time is {current_time} < allowed time {limit_time}"
    )]
    TooEarlyToWithdrawFees { limit_time: u32, current_time: u32 },

    #[error(code = 236, msg = "Fees withdrawal is too large")]
    FeesWithdrawalIsTooLarge { value: i64 },

    #[error(code = 237, msg = "Invalid oracle feed")]
    InvalidOracleFeed {
        expected_address: Pubkey,
        actual_address: Pubkey,
    },
    #[error(
        code = 238,
        msg = "Invalid Ref discount {discount}, expected to be in range {min}..{max}"
    )]
    InvalidRefDiscount { discount: f64, min: f64, max: f64 },

    #[error(
        code = 238,
        msg = "Invalid Ref ratio {ratio}, expected to be in range {min}..{max}"
    )]
    InvalidRefRatio { ratio: f64, min: f64, max: f64 },

    #[error(code = 239, msg = "Ref program is inactive")]
    RefProgramInactive,

    #[error(
        code = 240,
        msg = "Invalid ref link ID {ref_id}, acceptable ids: {first_ref_id}, {second_ref_id}"
    )]
    InvalidRefLinkId {
        ref_id: u32,
        first_ref_id: u32,
        second_ref_id: u32,
    },

    #[error(
        code = 241,
        msg = "Ref link with id {ref_id} is expired, current time is {current_time} > expiration time {expiration_time}"
    )]
    RefLinkExpired {
        ref_id: u32,
        expiration_time: u32,
        current_time: u32,
    },

    #[error(code = 242, msg = "Invalid referral client primary account address")]
    InvalidRefAddress {
        expected_address: Pubkey,
        actual_address: Pubkey,
    },

    #[error(code = 243, msg = "Operation Rejected")]
    OperationRejected,

    #[error(code = 244, msg = "Memory map creation or general error")]
    MemoryMapFailed,

    #[error(code = 245, msg = "Memory map deallocation error")]
    MemoryMapFreeFailed,

    #[error(
        code = 247,
        msg = "Invalid account tag in account, expected {{*expected_account_type as u32}} != actual {actual_tag}"
    )]
    InvalidAccountTag {
        expected_account_type: AccountType,
        actual_tag: u32,
        address: Pubkey,
    },
    #[error(
        code = 249,
        msg = "Invalid version in {account_type:?}, expected {expected} != actual {actual}"
    )]
    InvalidVersion {
        address: Pubkey,
        account_type: AccountType,
        expected: u32,
        actual: u32,
    },
    #[error(code = 250, msg = "Invalid data alignment in {address}")]
    InvalidDataAlignment { address: Pubkey },

    #[error(
        code = 251,
        msg = "Invalid amount of provided accounts, expected {expected} != actual {actual}"
    )]
    InvalidAccountsAmount { expected: u32, actual: u32 },
    #[error(
        code = 252,
        msg = "wSOL minting at legacy solana_native address is not supported"
    )]
    LegacyNativeMintNotSupported,

    #[error(code = 253, msg = "Identical tokens are not allowed in trading pair")]
    IdenticalTokensInPair { token_id: u32 },
    #[error(
        code = 255,
        msg = "wSOL minting at solana_native address is not supported"
    )]
    Token2022NativeMintNotSupported,

    #[error(
        code = 256,
        msg = "Invalid accounts address for {account_type} account"
    )]
    InvalidAccountAddress {
        expected_address: Pubkey,
        actual_address: Pubkey,
        account_type: AccountType,
    },
    #[error(code = 212, msg = "Candle with tag {tag} was not found")]
    CandleWasNotFound { tag: u32 },

    #[error(
        code = 250,
        msg = "Invalid accounts size in {account_type} account, actual {actual} < expected {expected}"
    )]
    InvalidAccountSize {
        address: Pubkey,
        account_type: AccountType,
        expected: usize,
        actual: usize,
    },
    #[error(code = 246, msg = "Failed to upgrade access manager to writable")]
    FailedToUpgrade,

    #[error(
        code = 258,
        msg = "Invalid candles amount, expected candle on index {index} exist while length is {len}"
    )]
    InvalidCandlesAmount { index: usize, len: usize },

    #[error(code = 110, msg = "Invalid wallet address")]
    #[serde(rename = "110")]
    InvalidWalletAddress {
        address: Pubkey,
        wallet_address: Pubkey,
        actual_address: Pubkey,
    },
    #[error(code = 224, msg = "Invalid candles context")]
    InvalidCandlesContext,

    #[error(code = 212, msg = "Unsupported account tag {tag}")]
    UnsupportedAccountTag { tag: u32 },

    #[error(
        code = 239,
        msg = "Self-referral is not allowed, client can not be referral for himself"
    )]
    SelfRefNotAllowed { client_primary_addr: Pubkey },

    #[error(code = 250, msg = "Offset {offset} is out of bound for data len {len}")]
    OffsetOutOfBounds {
        address: Pubkey,
        offset: usize,
        len: usize,
    },

    #[error(code = 284, msg = "Invalid token owner for {token_id} token")]
    InvalidTokenOwner {
        token_id: u32,
        address: Pubkey,
        expected_address: Pubkey,
        actual_address: Pubkey,
    },

    #[error(code = 280, msg = "Different token ids, {id_left} != {id_right}")]
    DifferentTokenIds { id_left: u32, id_right: u32 },

    #[error(code = 281, msg = "Perp clients count must not be 0")]
    InvalidPerpClientsCount,

    #[error(
        code = 282,
        msg = "Invalid supply {supply}, supply difference to MAX SUPPLY: {supply_difference}"
    )]
    InvalidSupply { supply: u32, supply_difference: u32 },

    #[error(code = 283, msg = "Perp was already allocated")]
    PerpAlreadyAllocated,

    #[error(code = 284, msg = "Traded amount ({amount}) is too small")]
    TradeIsTooSmall { amount: u32 },

    #[error(code = 285, msg = "Client primary account must be some")]
    ClientPrimaryAccountMustBeSome,

    #[error(code = 286, msg = "Must be in Private Mode")]
    MustBeInPrivateMode,

    #[error(
        code = 287,
        msg = "Non admin attemp of pair with DRVS token creation from {client_address}"
    )]
    NonAdminDrvsInstr {
        client_address: Pubkey,
        asset_token_id: u32,
        crncy_token_id: u32,
    },

    #[error(code = 288, msg = "{client_address} is not in private queue")]
    ClientIsNotInPrivate { client_address: Pubkey },

    #[error(
        code = 289,
        msg = "Invalid tokens decimals amount: {decs_count}, expected to be in range {min}..={max}"
    )]
    InvalidDecsCount {
        decs_count: u32,
        min: u32,
        max: u32,
        token_address: Pubkey,
    },
    #[error(
        code = 290,
        msg = "Clients wallet is already in private queue on index {index}"
    )]
    WalletIsInPrivateQueue { index: u32, wallet_address: Pubkey },

    #[error(
        code = 291,
        msg = "Invalid new expiration time for {program_name}, new time {new_time} < old_time {old_time}"
    )]
    InvalidNewExpirationTime {
        program_name: String,
        new_time: u32,
        old_time: u32,
    },

    #[error(
        code = 292,
        msg = "Provided variance is invalid, variance {variance} must be in {min_variance}..{max_variance}"
    )]
    InvalidVariance {
        variance: f64,
        min_variance: f64,
        max_variance: f64,
    },

    #[error(code = 293, msg = "Airdrop amount must be > 0")]
    InivalidAirdrop { wallet_address: Pubkey, ratio: f64 },

    #[error(
        code = 294,
        msg = "Airdrop authority wasn't initialised correctly. Expected {expected_address} != Actual {actual_address}"
    )]
    InvalidAirdropAuthority {
        expected_address: Pubkey,
        actual_address: Pubkey,
    },

    #[error(
        code = 295,
        msg = "Invalid Spl Token Program Id, expected: {expected}, actual: {actual}"
    )]
    InvalidSplTokenProgramId { expected: Pubkey, actual: Pubkey },

    #[error(
        code = 295,
        msg = "Failed to swap with give price {price} on side {side}"
    )]
    FailedToSwap { price: i64, side: OrderSide },

    #[error(
        code = 297,
        msg = "Token with id {token_id} can't be a base crncy. Reason: {reason}"
    )]
    InvalidBaseCrncy { token_id: u32, reason: String },

    #[error(
        code = 298,
        msg = "Private mode authority wasn't initialised correctly. Expected {expected_address} != Actual {actual_address}"
    )]
    InvalidPrivateModeAuthority {
        expected_address: Pubkey,
        actual_address: Pubkey,
    },

    #[error {
        code = 299,
        msg = "Max clients orders limit reached. For current client on {side} side max orders amount is {max_clients_orders}"
    }]
    MaxClientsOrderLimitReached {
        side: OrderSide,
        max_clients_orders: u32,
    },

    #[error(
        code = 300,
        msg = "Slippage bounds exceeded. Price: {price}, Bounds type: {bound_price}"
    )]
    SlippageExceeded { price: i64, bound_price: i64 },

    #[error(
        code = 301,
        msg = "Insuffecient LP tokens balance. Tokens after operation {final_tokens} < min amount {min_amount}"
    )]
    InsuffecientLpTokensBalance {
        lp_tokens: i64,
        operation_amount: i64,
        final_tokens: i64,
        min_amount: i64,
    },

    #[error(
        code = 302,
        msg = "Insuffecient pool supply. Pool supply after operation {final_ps} < min amount {min_amount}"
    )]
    InsuffecientPoolSupply {
        ps: i64,
        operation_amount: i64,
        final_ps: i64,
        min_amount: i64,
    },

    #[error(
        code = 303,
        msg = "System fault. In case of empty pool users lp balance must be 0"
    )]
    SystemPoolFault { ps: i64, lp_tokens: i64 },

    #[error(
        code = 304,
        msg = "Max instruments limit on the platform reached: {max_instr_amount}"
    )]
    MaxInstrumentLimitReached { max_instr_amount: u32 },

    #[error(code = 305, msg = "Invalid order type {order_type}")]
    InvalidOrderType {
        order_type: OrderType,
        order_type_raw: u8,
    },

    #[error(code = 306, msg = "Invalid denominator {denominator}, must be > 0")]
    InvalidDenominator { denominator: f64 },

    #[error(
        code = 307,
        msg = "Invalid base crncy id {base_crncy_id} was not found"
    )]
    InvalidBaseCrncyId { base_crncy_id: u32 },

    #[error(code = 308, msg = "Invalid fee rate, {fee_rate} < 0")]
    InvalidFeeValue { fee_rate: f64 },

    #[error(code = 309, msg = "No vote was found to change")]
    NoVoteToChange { voting_counter: u32 },

    #[error(code = 310, msg = "Invalid voting choice {choice}")]
    InvalidVotingChoice { voting_counter: u32, choice: u8 },

    #[error(code = 311, msg = "Referral id already assigned {ref_id}")]
    ReferralIdAlreadyAssigned { ref_id: u32 },

    #[error(code = 312, msg = "Invalid ref id value: {ref_id}")]
    InvalidRefIdValue { ref_id: u32 },

    #[error(code = 313, msg = "Attempted to create a referral on itself")]
    SelfRefLink,

    #[error(
        code = 314,
        msg = "Client {wallet_address} already has a registered account"
    )]
    AttemptedToAddExistingClient { wallet_address: Pubkey },

    #[error(code = 315, msg = "Invalid edge price {price}")]
    InvalidEdgePrice { price: i64 },

    #[error(
        code = 316,
        msg = "Incorrect Vm Mode status flag {flag:?}. Expected to be {expected}, actual {actual}"
    )]
    IncorrectVmModeStatus {
        flag: VmFlag,
        expected: bool,
        actual: bool,
    },

    #[error(code = 317, msg = "Invalid vm wallet address")]
    InvalidVmWalletAddress {
        address: Pubkey,
        actual_address: Pubkey,
        vm_wallet_address: Pubkey,
    },

    #[error(
        code = 318,
        msg = "No permission for instrument {instr_id} and trading section {trading_section}"
    )]
    InstrumentPermissionDenied {
        instr_id: u32,
        trading_section: TradingSection,
    },

    #[error(
        code = 319,
        msg = "Instruction {name}:{instruction_number} is restricted during vm mode"
    )]
    RestrictedInstructionDuringVmMode {
        name: String,
        instruction_number: u32,
    },
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "on-chain")]
    use solana_program::pubkey::Pubkey;
    #[cfg(feature = "off-chain")]
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_wallet_address() {
        let error = DeriverseErrorKind::InvalidWalletAddress {
            address: Pubkey::new_unique(),
            wallet_address: Pubkey::new_unique(),
            actual_address: Pubkey::new_unique(),
        };

        println!("{}", error.to_json());
    }

    #[test]
    fn test_error_codes_and_display() {
        // Test simple error
        let simple_err = DeriverseErrorKind::OperationRejected;
        assert_eq!(simple_err.code(), 243);
        assert_eq!(simple_err.to_string(), "Operation Rejected");

        // Test error with parameters
        let complex_err = DeriverseErrorKind::InvalidAccountsNumber {
            expected: 5,
            actual: 3,
        };
        assert_eq!(complex_err.code(), 101);
        assert_eq!(
            complex_err.to_string(),
            "Invalid provided accounts number: expected 5, got 3"
        );

        // Test Debug implementation
        let debug_str = format!("{:?}", complex_err);
        assert!(debug_str.contains("InvalidAccountsNumber"));
    }

    #[cfg(feature = "off-chain")]
    #[test]
    fn test_solana_integration_off_chain() {
        let err = DeriverseErrorKind::InvalidDataLength {
            expected: 100,
            actual: 50,
        };
        let code: u32 = err.into();
        assert_eq!(code, 102);
    }

    #[cfg(feature = "on-chain")]
    #[test]
    fn test_solana_integration_on_chain() {
        use solana_program::program_error::ProgramError;

        let err = DeriverseErrorKind::InvalidOperatorAccount {
            address: Pubkey::new_unique(),
        };

        let program_err: ProgramError = err.into();
        match program_err {
            ProgramError::Custom(code) => assert_eq!(code, 150),
            _ => panic!("Expected Custom program error"),
        }
    }

    #[test]
    fn test_json_serialization() {
        // Test custom to_json format
        let err = DeriverseErrorKind::InvalidDataLength {
            expected: 256,
            actual: 128,
        };
        let json = err.to_json();
        assert_eq!(json["code"], 102);
        assert_eq!(
            json["msg"],
            "Invalid instructions data length, expected 256 < actual 128"
        );
        assert_eq!(json["expected"], 256);

        // Test Pubkey in JSON
        let address = Pubkey::new_unique();
        let pubkey_err = DeriverseErrorKind::InvalidOperatorAccount { address };
        let pubkey_json = pubkey_err.to_json();
        assert_eq!(pubkey_json["address"], address.to_string());
    }

    #[test]
    fn test_serde_roundtrip() {
        // Test simple error
        let simple_err = DeriverseErrorKind::OperationRejected;
        let json_str = serde_json::to_string(&simple_err).unwrap();
        let deserialized: DeriverseErrorKind = serde_json::from_str(&json_str).unwrap();
        assert_eq!(simple_err, deserialized);

        // Test error with Pubkey
        let address = Pubkey::new_unique();
        let original = DeriverseErrorKind::InvalidOperatorAccount { address };
        let json_str = serde_json::to_string(&original).unwrap();
        let deserialized: DeriverseErrorKind = serde_json::from_str(&json_str).unwrap();
        assert_eq!(original.code(), deserialized.code());
    }

    #[test]
    fn test_context_pattern_and_equality() {
        // Test context pattern
        fn failing_op() -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "test"))
        }

        let result = failing_op().context(DeriverseErrorKind::OperationRejected);
        assert!(matches!(result, Err(DeriverseErrorKind::OperationRejected)));

        // Test equality
        let address = Pubkey::new_unique();
        let err1 = DeriverseErrorKind::InvalidOperatorAccount { address };
        let err2 = DeriverseErrorKind::InvalidOperatorAccount { address };
        assert_eq!(err1, err2);
    }
}
