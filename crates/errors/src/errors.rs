use drv_errors_derive::DrvError;
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

#[derive(Debug, DrvError, Serialize, Deserialize, Eq, PartialEq)]
pub enum DeriverseError {
    // Errors with parameters for demonstration
    #[error(
        code = 101,
        msg = "Invalid accounts number: expected {expected}, got {actual}"
    )]
    InvalidAccountsNumber { expected: usize, actual: usize },

    #[error(
        code = 102,
        msg = "Invalid data length: expected {expected}, got {actual}"
    )]
    InvalidDataLength { expected: usize, actual: usize },

    #[error(code = 103, msg = "Invalid root account {account}")]
    InvalidRootAccount { account: Pubkey },

    #[error(
        code = 104,
        msg = "Invalid signer account {account}, expected {expected_signer}"
    )]
    InvalidSignerAccount {
        account: Pubkey,
        expected_signer: Pubkey,
    },

    // Simple errors without parameters
    #[error(code = 133, msg = "Invalid Client Primary Account")]
    InvalidClientPrimaryAccount,

    #[error(code = 134, msg = "Invalid Client Derivatives Account")]
    InvalidClientDerivativesAccount,

    #[error(code = 105, msg = "Invalid Instrument Static Account")]
    InvalidInstrStaticAccount,

    #[error(code = 106, msg = "Invalid Instrument Trace Account")]
    InvalidInstrTraceAccount,

    #[error(code = 107, msg = "Invalid Instrument Account")]
    InvalidInstrAccount,

    #[error(code = 108, msg = "Invalid Spot Bids Tree Account")]
    InvalidSpotBidsTreeAccount,

    #[error(code = 109, msg = "Invalid Spot Asks Tree Account")]
    InvalidSpotAsksTreeAccount,

    #[error(code = 110, msg = "Invalid Spot Bid Orders Account")]
    InvalidSpotBidOrdersAccount,

    #[error(code = 111, msg = "Invalid Spot Ask Orders Account")]
    InvalidSpotAskOrdersAccount,

    #[error(code = 112, msg = "Invalid Spot Lines Account")]
    InvalidSpotLinesAccount,

    #[error(code = 113, msg = "Invalid Spot Maps Account")]
    InvalidSpotMapsAccount,

    #[error(code = 114, msg = "Invalid Spot Client Infos Account")]
    InvalidSpotClientInfosAccount,

    #[error(code = 115, msg = "Invalid Spot Client Infos2 Account")]
    InvalidSpotClientInfos2Account,

    #[error(code = 116, msg = "Invalid Spot Client Accounts Account")]
    InvalidSpotClientAccountsAccount,

    #[error(code = 117, msg = "Invalid Candles Account")]
    InvalidCandlesAccount,

    #[error(code = 120, msg = "Invalid Tokens Account")]
    InvalidTokensAccount,

    #[error(code = 121, msg = "Invalid Base Tokens Account")]
    InvalidBaseTokensAccount,

    #[error(code = 142, msg = "Invalid PDF Account")]
    InvalidPdfAccount,

    #[error(code = 122, msg = "Invalid Token Program ID")]
    InvalidTokenProgramId,

    #[error(code = 123, msg = "Invalid Token 2022 Program ID")]
    InvalidToken2022ProgramId,

    #[error(code = 124, msg = "Invalid Mint Address")]
    InvalidMintAccount,

    #[error(code = 125, msg = "Invalid Token Address")]
    InvalidTokenAddress,

    #[error(code = 126, msg = "Invalid Token Program Address")]
    InvalidTokenProgramAddress,

    #[error(code = 127, msg = "Invalid LUT Program ID")]
    InvalidLutProgramId,

    #[error(code = 128, msg = "Invalid LUT Account")]
    InvalidLutAccount,

    #[error(code = 129, msg = "Invalid System Program ID")]
    InvalidSystemProgramId,

    #[error(code = 130, msg = "Invalid Quantity")]
    InvalidQuantity,

    #[error(code = 131, msg = "Invalid Price")]
    InvalidPrice,

    #[error(code = 132, msg = "Insufficient Funds")]
    InsufficientFunds,

    #[error(code = 135, msg = "Instrument Is Not Active")]
    InstrIsNotActive,

    #[error(code = 136, msg = "Too Much Lines")]
    TooMuchLines,

    #[error(code = 137, msg = "Allocator Failed")]
    AllocatorFailed,

    #[error(code = 138, msg = "You Try To Trade With Yourself")]
    CrossOrder,

    #[error(code = 139, msg = "Matching Engine Failed")]
    MatchingEngineFailed,

    #[error(code = 140, msg = "Pool Trade Failed")]
    PoolTradeFailed,

    #[error(code = 141, msg = "Invalid PDA")]
    InvalidPDA,

    #[error(code = 143, msg = "Invalid New Program Account")]
    InvalidNewProgramAccount,

    #[error(code = 144, msg = "Invalid New Account")]
    InvalidNewAccount,

    #[error(code = 145, msg = "Invalid Holder Account")]
    InvalidHolderAccount,

    #[error(code = 146, msg = "Invalid Holder Admin Account")]
    InvalidHolderAdminAccount,

    #[error(code = 147, msg = "Invalid Admin Account")]
    InvalidAdminAccount,

    #[error(code = 148, msg = "Invalid New Operator Account")]
    InvalidNewOperatorAccount,

    #[error(code = 149, msg = "Invalid New Account PDA")]
    InvalidNewAccountPDA,

    #[error(code = 150, msg = "Invalid Operator Account")]
    InvalidOperatorAccount,

    #[error(code = 151, msg = "Invalid Token ID")]
    InvalidTokenId,

    #[error(code = 152, msg = "Instrument Is Active")]
    InstrIsActive,

    #[error(code = 153, msg = "Mint Is Not Initialized")]
    MintIsNotInitialized,

    #[error(code = 154, msg = "SPL Token Account Is Not Initialized")]
    SplTokenAccountIsNotInitialized,

    #[error(code = 155, msg = "Invalid Token Account")]
    InvalidTokenAccount,

    #[error(code = 156, msg = "Pool Already Exists")]
    PoolAlreadyExists,

    #[error(code = 157, msg = "Invalid Futures Bids Tree Account")]
    InvalidFuturesBidsTreeAccount,

    #[error(code = 158, msg = "Invalid Futures Asks Tree Account")]
    InvalidFuturesAsksTreeAccount,

    #[error(code = 159, msg = "Invalid Futures Bid Orders Account")]
    InvalidFuturesBidOrdersAccount,

    #[error(code = 160, msg = "Invalid Futures Ask Orders Account")]
    InvalidFuturesAskOrdersAccount,

    #[error(code = 161, msg = "Invalid Futures Lines Account")]
    InvalidFuturesLinesAccount,

    #[error(code = 162, msg = "Invalid Futures Maps Account")]
    InvalidFuturesMapsAccount,

    #[error(code = 163, msg = "Invalid Futures Client Infos Account")]
    InvalidFuturesClientInfosAccount,

    #[error(code = 164, msg = "Invalid Futures Client Infos2 Account")]
    InvalidFuturesClientInfos2Account,

    #[error(code = 165, msg = "Invalid Futures Client Accounts Account")]
    InvalidFuturesClientAccountsAccount,

    #[error(code = 166, msg = "Invalid Instr ID")]
    InvalidInstrId,

    #[error(code = 167, msg = "Invalid Task ID")]
    InvalidTaskId,

    #[error(code = 168, msg = "Invalid Pool Instr ID")]
    InvalidPoolInstrId,

    #[error(code = 169, msg = "Max Number Of Tasks Exceeded")]
    MaxNumberOfTasksExceeded,

    #[error(code = 170, msg = "Task Is Not Started")]
    TaskIsNotStarted,

    #[error(code = 171, msg = "Task Is Already Started")]
    InvalidTaskIsAlreadyStarted,

    #[error(code = 172, msg = "Invalid Tokens Amount")]
    InvalidTokensAmount,

    #[error(code = 173, msg = "Insufficient Program Funds")]
    InsufficientProgramFunds,

    #[error(code = 174, msg = "Spot Pool Is Empty")]
    SpotPoolIsEmpty,

    #[error(code = 175, msg = "Order Not Found")]
    OrderNotFound,

    #[error(code = 176, msg = "Invalid Vanilla Trades Count")]
    InvalidVanillaTradesCount,

    #[error(code = 177, msg = "Invalid Options Amount")]
    InvalidOptionsAmount,

    #[error(code = 178, msg = "Invalid Strike ID")]
    InvalidStrikeId,

    #[error(code = 179, msg = "Max Cost Difference Exceeded")]
    MaxCostDiffExceeded,

    #[error(code = 180, msg = "Insufficient Pool Funds")]
    InsufficientPoolFunds,

    #[error(code = 181, msg = "Trading Is Not Available")]
    TradingIsNotAvailable,

    #[error(code = 182, msg = "Trading For This Strike Is Not Available")]
    TradingForThisStrikeIsNotAvailable,

    #[error(code = 183, msg = "Client Data Destruction")]
    ClientDataDestruction,

    #[error(code = 184, msg = "Invalid Options Pool Mint Supply")]
    InvalidOptionsPoolMintSupply,

    #[error(code = 185, msg = "Invalid Base Currency")]
    InvalidCrncy,

    #[error(code = 186, msg = "Invalid Task")]
    InvalidTask,

    #[error(code = 187, msg = "Invalid Time")]
    InvalidTime,

    #[error(code = 188, msg = "Invalid Sigma")]
    InvalidSigma,

    #[error(code = 189, msg = "Impossible To Upgrade")]
    ImpossibleToUpgrade,

    #[error(code = 190, msg = "Invalid Bid Orders Count")]
    InvalidBidOrdersCount,

    #[error(code = 191, msg = "Invalid Ask Orders Count")]
    InvalidAskOrdersCount,

    #[error(code = 192, msg = "Invalid Bid Lines Count")]
    InvalidBidLinesCount,

    #[error(code = 193, msg = "Invalid Ask Lines Count")]
    InvalidAskLinesCount,

    #[error(code = 194, msg = "Impossible To Payoff")]
    ImpossibleToPayoff,

    #[error(code = 195, msg = "Too Small Amount To Withdraw")]
    TooSmallAmountToWithdraw,

    #[error(code = 196, msg = "Invalid Associated Token Address")]
    InvalidAssociatedTokenAddress,

    #[error(code = 197, msg = "Invalid Instance ID")]
    InvalidInstanceId,

    #[error(code = 198, msg = "Collateral Reduction Unavailable")]
    CollateralReductionUnavailable,

    #[error(code = 200, msg = "Base Currency Token Not Found")]
    BaseCrncyNotFound,

    #[error(code = 201, msg = "Too Early To Distrib Funds")]
    TooEarlyToDistribFunds,

    #[error(code = 202, msg = "Insufficient Deriverse Tokens")]
    InsufficientDeriverseTokens,

    #[error(code = 203, msg = "Insufficient Deriverse Tokens Supply")]
    InsufficientDeriverseTokensSupply,

    #[error(code = 204, msg = "Invalid Client Community Account")]
    InvalidClientCommunityAccount,

    #[error(code = 205, msg = "Invalid Voting Counter")]
    InvalidVotingCounter,

    #[error(code = 206, msg = "Already Voted")]
    AlreadyVoted,

    #[error(code = 207, msg = "Airdrop Failed")]
    AirdropFailed,

    #[error(code = 208, msg = "Invalid Deriverse Authority Account")]
    InvalidDrvsAuthAccount,

    #[error(code = 209, msg = "Invalid Community Account")]
    InvalidCommunityAccount,

    #[error(code = 210, msg = "No Trade (IOC)")]
    NoTradeIOC,

    #[error(code = 211, msg = "Invalid Asset Type")]
    InvalidAssetType,

    #[error(code = 212, msg = "Asset Not Found")]
    AssetNotFound,

    #[error(code = 213, msg = "Invalid Spot Account")]
    InvalidSpotAccount,

    #[error(code = 214, msg = "Null Pointer")]
    NullPointer,

    #[error(code = 215, msg = "Invalid Client Bids Count")]
    InvalidClientBidsCount,

    #[error(code = 216, msg = "Invalid Client Asks Count")]
    InvalidClientAsksCount,

    #[error(code = 217, msg = "Community Account Has To Be Read Only")]
    CommunityAccountHasToBeReadOnly,

    #[error(code = 218, msg = "Invalid Token Type")]
    InvalidTokenType,

    #[error(code = 219, msg = "Null Index")]
    NullIndex,

    #[error(code = 220, msg = "Invalid Futures Account")]
    InvalidFuturesAccount,

    #[error(code = 221, msg = "Client Derivative Not Found")]
    ClientDerivativeNotFound,

    #[error(code = 222, msg = "Debug Breaking Point")]
    DebugBreakingPoint,

    #[error(code = 223, msg = "Arithmetic Overflow")]
    ArithmeticOverflow,

    #[error(code = 224, msg = "Invalid Data Format")]
    InvalidDataFormat,

    #[error(code = 225, msg = "Invalid Order ID")]
    InvalidOrderId,

    #[error(code = 226, msg = "Perp Is Not Available")]
    PerpIsNotAvailable,

    #[error(code = 227, msg = "Invalid Perp Account")]
    InvalidPerpAccount,

    #[error(code = 228, msg = "Invalid Perp Maps Account")]
    InvalidPerpMapsAccount,

    #[error(code = 229, msg = "Impossible To Withdraw Funds During Margin Call")]
    ImpossibleToWithdrawFundsDuringMarginCall,

    #[error(code = 230, msg = "Invalid Perp Clients Count")]
    InvalidPerpClientsCount,

    #[error(code = 231, msg = "Max Perp Clients Count Reached")]
    MaxPerpClientsCountReached,

    #[error(code = 232, msg = "Invalid Leverage")]
    InvalidLeverage,

    #[error(code = 233, msg = "Invalid Socialized Loss Open Interest")]
    InvalidSocializedLossOpenInterest,

    #[error(code = 234, msg = "Impossible To Close Perp Position")]
    ImpossibleToClosePerpPosition,

    #[error(code = 235, msg = "Too Early To Withdraw Fees")]
    TooEarlyToWithdrawFees,

    #[error(code = 236, msg = "Fees Withdrawal Is Too Large")]
    FeesWithdrawalIsTooLarge,

    #[error(code = 237, msg = "Invalid Oracle Feed")]
    InvalidOracleFeed,

    #[error(code = 238, msg = "Invalid Ref Program Parameters")]
    InvalidRefProgramParameters,

    #[error(code = 239, msg = "Ref Program Inactive")]
    RefProgramInactive,

    #[error(code = 240, msg = "Invalid Ref Link ID")]
    InvalidRefLinkId,

    #[error(code = 241, msg = "Ref Link Expired")]
    RefLinkExpired,

    #[error(code = 242, msg = "Invalid Ref Address")]
    InvalidRefAddress,

    #[error(code = 243, msg = "Operation Rejected")]
    OperationRejected,

    #[error(code = 244, msg = "Memory map creation or general error")]
    MemoryMapFailed,

    #[error(code = 245, msg = "Memory map deallocation error")]
    MemoryMapFreeFailed,

    #[error(code = 246, msg = "Invalid Write Permission")]
    InvalidWritePermission,

    #[error(code = 247, msg = "Invalid Account Tag")]
    InvalidAccountTag,

    #[error(code = 248, msg = "Invalid Account Owner")]
    InvalidAccountOwner,

    #[error(code = 249, msg = "Incompatible version in the RootState")]
    InvalidRootAccountVersion,

    #[error(code = 250, msg = "Invalid data alignment")]
    InvalidDataAlignment,

    #[error(code = 251, msg = "Invalid amount of provided accounts")]
    InvalidAccountsAmount,

    #[error(
        code = 252,
        msg = "wSOL minting at legacy solana_native address is not supported"
    )]
    LegacyNativeMintNotSupported,

    #[error(code = 253, msg = "Identical tokens not allowed in trading pair")]
    IdenticalTokensInPair,

    #[error(code = 254, msg = "Failed to call invoke or invoke signed")]
    InvokeFailed,

    #[error(
        code = 255,
        msg = "wSOL minting at solana_native address is not supported"
    )]
    Token2022NativeMintNotSupported,

    #[error(code = 256, msg = "Invalid Maps account address")]
    InvalidMapsAccountAddress,

    #[error(code = 257, msg = "Trade is too small")]
    TradeIsTooSmall,

    #[error(code = 258, msg = "Perp was already allocated")]
    PerpAlreadyAllocated,

    #[error(code = 259, msg = "Invalid Supply")]
    InvalidSupply,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "on-chain")]
    use solana_program::pubkey::Pubkey;
    #[cfg(feature = "off-chain")]
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_error_codes_and_display() {
        // Test simple error
        let simple_err = DeriverseError::OperationRejected;
        assert_eq!(simple_err.code(), 243);
        assert_eq!(simple_err.to_string(), "Operation Rejected");

        // Test error with parameters
        let complex_err = DeriverseError::InvalidAccountsNumber {
            expected: 5,
            actual: 3,
        };
        assert_eq!(complex_err.code(), 101);
        assert_eq!(complex_err.to_string(), "Invalid accounts number: expected 5, got 3");

        // Test Debug implementation
        let debug_str = format!("{:?}", complex_err);
        assert!(debug_str.contains("InvalidAccountsNumber"));
    }

    #[cfg(feature = "off-chain")]
    #[test]
    fn test_solana_integration_off_chain() {
        let err = DeriverseError::InvalidDataLength { expected: 100, actual: 50 };
        let code: u32 = err.into();
        assert_eq!(code, 102);
    }

    #[cfg(feature = "on-chain")]
    #[test]
    fn test_solana_integration_on_chain() {
        use solana_program::program_error::ProgramError;

        let err = DeriverseError::InvalidRootAccount { account: Pubkey::new_unique() };
        let program_err: ProgramError = err.into();
        match program_err {
            ProgramError::Custom(code) => assert_eq!(code, 103),
            _ => panic!("Expected Custom program error"),
        }
    }

    #[test]
    fn test_json_serialization() {
        // Test custom to_json format
        let err = DeriverseError::InvalidDataLength { expected: 256, actual: 128 };
        let json = err.to_json();
        assert_eq!(json["code"], 102);
        assert_eq!(json["msg"], "Invalid data length: expected 256, got 128");
        assert_eq!(json["expected"], 256);

        // Test Pubkey in JSON
        let account = Pubkey::new_unique();
        let pubkey_err = DeriverseError::InvalidRootAccount { account };
        let pubkey_json = pubkey_err.to_json();
        assert_eq!(pubkey_json["account"], account.to_string());
    }

    #[test]
    fn test_serde_roundtrip() {
        // Test simple error
        let simple_err = DeriverseError::OperationRejected;
        let json_str = serde_json::to_string(&simple_err).unwrap();
        let deserialized: DeriverseError = serde_json::from_str(&json_str).unwrap();
        assert_eq!(simple_err, deserialized);

        // Test error with Pubkey
        let account = Pubkey::new_unique();
        let original = DeriverseError::InvalidRootAccount { account };
        let json_str = serde_json::to_string(&original).unwrap();
        let deserialized: DeriverseError = serde_json::from_str(&json_str).unwrap();
        assert_eq!(original.code(), deserialized.code());
    }

    #[test]
    fn test_context_pattern_and_equality() {
        // Test context pattern
        fn failing_op() -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "test"))
        }

        let result = failing_op().context(DeriverseError::OperationRejected);
        assert!(matches!(result, Err(DeriverseError::OperationRejected)));

        // Test equality
        let account = Pubkey::new_unique();
        let err1 = DeriverseError::InvalidRootAccount { account };
        let err2 = DeriverseError::InvalidRootAccount { account };
        assert_eq!(err1, err2);
    }
}