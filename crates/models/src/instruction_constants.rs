pub trait DrvInstruction {
    const INSTRUCTION_NUMBER: u8;
    const MIN_ACCOUNTS: usize;
}

pub struct NewHolderInstruction;
impl DrvInstruction for NewHolderInstruction {
    const INSTRUCTION_NUMBER: u8 = 0;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct NewOperatorInstruction;
impl DrvInstruction for NewOperatorInstruction {
    const INSTRUCTION_NUMBER: u8 = 1;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct NewRootAccountInstruction;
impl DrvInstruction for NewRootAccountInstruction {
    const INSTRUCTION_NUMBER: u8 = 2;
    const MIN_ACCOUNTS: usize = 12;
}

pub struct PerpWithdrawInstruction;
impl DrvInstruction for PerpWithdrawInstruction {
    const INSTRUCTION_NUMBER: u8 = 3;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct NewBaseCrncyInstruction;
impl DrvInstruction for NewBaseCrncyInstruction {
    const INSTRUCTION_NUMBER: u8 = 4;
    const MIN_ACCOUNTS: usize = 9;
}

pub struct FeesDepositInstruction;
impl DrvInstruction for FeesDepositInstruction {
    const INSTRUCTION_NUMBER: u8 = 5;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct DepositInstruction;
impl DrvInstruction for DepositInstruction {
    const INSTRUCTION_NUMBER: u8 = 7;
    const MIN_ACCOUNTS: usize = 9;
}

pub struct WithdrawInstruction;
impl DrvInstruction for WithdrawInstruction {
    const INSTRUCTION_NUMBER: u8 = 8;
    const MIN_ACCOUNTS: usize = 11;
}

pub struct NewInstrumentInstruction;
impl DrvInstruction for NewInstrumentInstruction {
    const INSTRUCTION_NUMBER: u8 = 9;
    const MIN_ACCOUNTS: usize = 23;
}

pub struct UpgradeToPerpInstruction;
impl DrvInstruction for UpgradeToPerpInstruction {
    const INSTRUCTION_NUMBER: u8 = 10;
    const MIN_ACCOUNTS: usize = 21;
}

pub struct PerpDepositInstruction;
impl DrvInstruction for PerpDepositInstruction {
    const INSTRUCTION_NUMBER: u8 = 11;
    const MIN_ACCOUNTS: usize = 19;
}

pub struct NewSpotOrderInstruction;
impl DrvInstruction for NewSpotOrderInstruction {
    const INSTRUCTION_NUMBER: u8 = 12;
    const MIN_ACCOUNTS: usize = 18;
}

pub struct SpotOrderCancelInstruction;
impl DrvInstruction for SpotOrderCancelInstruction {
    const INSTRUCTION_NUMBER: u8 = 13;
    const MIN_ACCOUNTS: usize = 14;
}

pub struct SpotLpInstruction;
impl DrvInstruction for SpotLpInstruction {
    const INSTRUCTION_NUMBER: u8 = 14;
    const MIN_ACCOUNTS: usize = 5;
}

pub struct SpotMassCancelInstruction;
impl DrvInstruction for SpotMassCancelInstruction {
    const INSTRUCTION_NUMBER: u8 = 15;
    const MIN_ACCOUNTS: usize = 14;
}

pub struct NextVotingInstruction;
impl DrvInstruction for NextVotingInstruction {
    const INSTRUCTION_NUMBER: u8 = 16;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct NewPerpOrderInstruction;
impl DrvInstruction for NewPerpOrderInstruction {
    const INSTRUCTION_NUMBER: u8 = 19;
    const MIN_ACCOUNTS: usize = 21;
}

pub struct DividendsAllocationInstruction;
impl DrvInstruction for DividendsAllocationInstruction {
    const INSTRUCTION_NUMBER: u8 = 25;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct SwapInstruction;
impl DrvInstruction for SwapInstruction {
    const INSTRUCTION_NUMBER: u8 = 26;
    const MIN_ACCOUNTS: usize = 28;
}

pub struct AirdropInstruction;
impl DrvInstruction for AirdropInstruction {
    const INSTRUCTION_NUMBER: u8 = 27;
    const MIN_ACCOUNTS: usize = 12;
}

pub struct DividendsClaimInstruction;
impl DrvInstruction for DividendsClaimInstruction {
    const INSTRUCTION_NUMBER: u8 = 28;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct PerpOrderCancelInstruction;
impl DrvInstruction for PerpOrderCancelInstruction {
    const INSTRUCTION_NUMBER: u8 = 30;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct VotingInstruction;
impl DrvInstruction for VotingInstruction {
    const INSTRUCTION_NUMBER: u8 = 32;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct SpotQuotesReplaceInstruction;
impl DrvInstruction for SpotQuotesReplaceInstruction {
    const INSTRUCTION_NUMBER: u8 = 34;
    const MIN_ACCOUNTS: usize = 18;
}

pub struct PerpMassCancelInstruction;
impl DrvInstruction for PerpMassCancelInstruction {
    const INSTRUCTION_NUMBER: u8 = 36;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct PerpChangeLeverageInstruction;
impl DrvInstruction for PerpChangeLeverageInstruction {
    const INSTRUCTION_NUMBER: u8 = 37;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct FeesWithdrawInstruction;
impl DrvInstruction for FeesWithdrawInstruction {
    const INSTRUCTION_NUMBER: u8 = 39;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct SetInstrReadyForPerpUpgradeInstruction;
impl DrvInstruction for SetInstrReadyForPerpUpgradeInstruction {
    const INSTRUCTION_NUMBER: u8 = 41;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct PerpQuotesReplaceInstruction;
impl DrvInstruction for PerpQuotesReplaceInstruction {
    const INSTRUCTION_NUMBER: u8 = 42;
    const MIN_ACCOUNTS: usize = 21;
}

pub struct MoveSpotAvailFundsInstruction;
impl DrvInstruction for MoveSpotAvailFundsInstruction {
    const INSTRUCTION_NUMBER: u8 = 43;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct ChangeRefProgramInstruction;
impl DrvInstruction for ChangeRefProgramInstruction {
    const INSTRUCTION_NUMBER: u8 = 44;
    const MIN_ACCOUNTS: usize = 2;
}

pub struct NewRefLinkInstruction;
impl DrvInstruction for NewRefLinkInstruction {
    const INSTRUCTION_NUMBER: u8 = 45;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct PerpStatisticsResetInstruction;
impl DrvInstruction for PerpStatisticsResetInstruction {
    const INSTRUCTION_NUMBER: u8 = 46;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct BuyMarketSeatInstruction;
impl DrvInstruction for BuyMarketSeatInstruction {
    const INSTRUCTION_NUMBER: u8 = 47;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct SellMarketSeatInstruction;
impl DrvInstruction for SellMarketSeatInstruction {
    const INSTRUCTION_NUMBER: u8 = 48;
    const MIN_ACCOUNTS: usize = 20;
}

pub struct NewPrivateClientInstruction;
impl DrvInstruction for NewPrivateClientInstruction {
    const INSTRUCTION_NUMBER: u8 = 49;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct TerminatePrivateModeInstruction;
impl DrvInstruction for TerminatePrivateModeInstruction {
    const INSTRUCTION_NUMBER: u8 = 50;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct ChangePointsProgramExpirationInstruction;
impl DrvInstruction for ChangePointsProgramExpirationInstruction {
    const INSTRUCTION_NUMBER: u8 = 51;
    const MIN_ACCOUNTS: usize = 2;
}

pub struct ChangeAirdropAuthority;
impl DrvInstruction for ChangeAirdropAuthority {
    const INSTRUCTION_NUMBER: u8 = 52;

    const MIN_ACCOUNTS: usize = 3;
}

pub struct ChangePrivateModeAuthority;
impl DrvInstruction for ChangePrivateModeAuthority {
    const INSTRUCTION_NUMBER: u8 = 53;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct SetVariance;
impl DrvInstruction for SetVariance {
    const INSTRUCTION_NUMBER: u8 = 54;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct VotingReset;
impl DrvInstruction for VotingReset {
    const INSTRUCTION_NUMBER: u8 = 55;
    const MIN_ACCOUNTS: usize = 3;
}

pub struct ChangeDenominator;
impl DrvInstruction for ChangeDenominator {
    const INSTRUCTION_NUMBER: u8 = 56;
    const MIN_ACCOUNTS: usize = 3;
}
pub struct PerpClientsProcessingInstruction;
impl DrvInstruction for PerpClientsProcessingInstruction {
    const INSTRUCTION_NUMBER: u8 = 57;

    const MIN_ACCOUNTS: usize = 19;
}

pub struct SetSeatPurchasingFee;
impl DrvInstruction for SetSeatPurchasingFee {
    const INSTRUCTION_NUMBER: u8 = 58;
    const MIN_ACCOUNTS: usize = 2;
}

pub struct ChangeVotingInstruction;
impl DrvInstruction for ChangeVotingInstruction {
    const INSTRUCTION_NUMBER: u8 = 59;
    const MIN_ACCOUNTS: usize = 6;
}

pub struct GarbageCollectorInstruction;
impl DrvInstruction for GarbageCollectorInstruction {
    const INSTRUCTION_NUMBER: u8 = 60;

    const MIN_ACCOUNTS: usize = 6;
}

pub struct ActivateClientRefProgram;
impl DrvInstruction for ActivateClientRefProgram {
    const INSTRUCTION_NUMBER: u8 = 61;

    const MIN_ACCOUNTS: usize = 4;
}

pub struct CleanCandlesInstruction;
impl DrvInstruction for CleanCandlesInstruction {
    const INSTRUCTION_NUMBER: u8 = 62;

    const MIN_ACCOUNTS: usize = 6;
}

pub struct VmInitActivate;
impl DrvInstruction for VmInitActivate {
    const INSTRUCTION_NUMBER: u8 = 63;
    const MIN_ACCOUNTS: usize = 5;
}

pub struct VmInitActivateCancel;
impl DrvInstruction for VmInitActivateCancel {
    const INSTRUCTION_NUMBER: u8 = 64;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmFinalizeActivate;
impl DrvInstruction for VmFinalizeActivate {
    const INSTRUCTION_NUMBER: u8 = 65;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmInitDeactivate;
impl DrvInstruction for VmInitDeactivate {
    const INSTRUCTION_NUMBER: u8 = 66;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmInitDeactivateCancel;
impl DrvInstruction for VmInitDeactivateCancel {
    const INSTRUCTION_NUMBER: u8 = 67;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmFinalizeDeactivate;
impl DrvInstruction for VmFinalizeDeactivate {
    const INSTRUCTION_NUMBER: u8 = 68;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmInitWithdraw;
impl DrvInstruction for VmInitWithdraw {
    const INSTRUCTION_NUMBER: u8 = 69;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmInitWithdrawCancel;
impl DrvInstruction for VmInitWithdrawCancel {
    const INSTRUCTION_NUMBER: u8 = 70;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmInitWithdrawFinalize;
impl DrvInstruction for VmInitWithdrawFinalize {
    const INSTRUCTION_NUMBER: u8 = 71;
    const MIN_ACCOUNTS: usize = 4;
}

pub struct VmChangeWhitelist;
impl DrvInstruction for VmChangeWhitelist {
    const INSTRUCTION_NUMBER: u8 = 72;
    const MIN_ACCOUNTS: usize = 4;
}
