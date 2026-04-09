pub mod instr_mask {
    use bytemuck::{Pod, Zeroable};
    use serde::{Deserialize, Serialize};

    #[repr(u32)]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    pub enum InstrFlag {
        PerpActive = 0x40000000,
        ReadyToPerpUpgrade = 0x01000000,
        ZeroFees = 0x1,
        FixedFees = 0x2,
        SimilarAssets = 0x4,
        UsdStablecoin = 0x8,
        Forex = 0x10,
        Suspended = 0x20,
        LongMarginCall = 0x40,
        ShortMarginCall = 0x80,
        ExpandableCandles = 0x100,
    }

    impl std::fmt::Display for InstrFlag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub trait SimpleInstrMask {
        fn get_flag(&self, flag: InstrFlag) -> bool;
        fn set_flag(&mut self, flag: InstrFlag);
        fn clear_flag(&mut self, flag: InstrFlag);
    }

    #[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct InstrMask(pub u32);

    impl InstrMask {
        pub fn merge(&mut self, input: InstrInputMask) {
            self.0 |= (input.0 as u32) & InstrInputMask::ALLOWED_FLAGS;
        }
    }

    impl SimpleInstrMask for InstrMask {
        fn get_flag(&self, flag: InstrFlag) -> bool {
            self.0 & flag as u32 != 0
        }
        fn set_flag(&mut self, flag: InstrFlag) {
            self.0 |= flag as u32
        }
        fn clear_flag(&mut self, flag: InstrFlag) {
            self.0 &= !(flag as u32)
        }
    }

    #[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct InstrInputMask(u8);

    impl InstrInputMask {
        pub const ALLOWED_FLAGS: u32 = InstrFlag::ZeroFees as u32
            | InstrFlag::FixedFees as u32
            | InstrFlag::SimilarAssets as u32
            | InstrFlag::UsdStablecoin as u32;
    }

    impl SimpleInstrMask for InstrInputMask {
        fn get_flag(&self, flag: InstrFlag) -> bool {
            if flag as u32 > u8::MAX as u32 {
                return false;
            }
            self.0 & flag as u8 != 0
        }
        fn set_flag(&mut self, flag: InstrFlag) {
            if flag as u32 > u8::MAX as u32 {
                return;
            }
            self.0 |= flag as u8
        }
        fn clear_flag(&mut self, flag: InstrFlag) {
            if flag as u32 > u8::MAX as u32 {
                return;
            }
            self.0 &= !(flag as u8)
        }
    }

    #[test]
    fn merge_test() {
        let mut instr_mask = InstrMask(0);
        instr_mask.set_flag(InstrFlag::ReadyToPerpUpgrade);

        let mut input_instr_mask = InstrInputMask(0);

        input_instr_mask.set_flag(InstrFlag::SimilarAssets);
        input_instr_mask.set_flag(InstrFlag::Forex);

        instr_mask.merge(input_instr_mask);

        assert!(instr_mask.get_flag(InstrFlag::Forex));
        assert!(instr_mask.get_flag(InstrFlag::ReadyToPerpUpgrade));
        assert!(instr_mask.get_flag(InstrFlag::SimilarAssets));
    }
}

pub mod token_mask {
    use bytemuck::{Pod, Zeroable};
    use serde::{Deserialize, Serialize};

    #[repr(u32)]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    pub enum TokenFlag {
        Token2022 = 0x80000000,
        BaseCrncy = 0x40000000,
        WrappedToken = 0x20000000,
        SAMCrncy = 0x10000000,
    }

    impl std::fmt::Display for TokenFlag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TokenMask(pub u32);

    impl TokenMask {
        const DECIMALS_MASK: u32 = 0xFF;
        const BUMP_MASK: u32 = 0xFF00;
        const BUMP_SHIFT: u32 = 8;

        pub fn get_flag(&self, flag: TokenFlag) -> bool {
            self.0 & flag as u32 != 0
        }

        pub fn set_flag(&mut self, flag: TokenFlag) {
            self.0 |= flag as u32;
        }

        pub fn clear_flag(&mut self, flag: TokenFlag) {
            self.0 &= !(flag as u32);
        }

        pub fn decimals(&self) -> u8 {
            (self.0 & Self::DECIMALS_MASK) as u8
        }

        pub fn set_decimals(&mut self, decimals: u8) {
            self.0 = (self.0 & !Self::DECIMALS_MASK) | (decimals as u32);
        }

        pub fn bump_seed(&self) -> u8 {
            ((self.0 & Self::BUMP_MASK) >> Self::BUMP_SHIFT) as u8
        }

        pub fn set_bump_seed(&mut self, bump_seed: u8) {
            self.0 = (self.0 & !Self::BUMP_MASK) | ((bump_seed as u32) << Self::BUMP_SHIFT);
        }
    }

    #[test]
    fn test_token_mask() {
        let mut mask = TokenMask(0);

        mask.set_decimals(9);
        assert_eq!(mask.decimals(), 9);

        mask.set_bump_seed(255);
        assert_eq!(mask.bump_seed(), 255);

        mask.set_flag(TokenFlag::BaseCrncy);
        mask.set_flag(TokenFlag::SAMCrncy);

        assert!(mask.get_flag(TokenFlag::BaseCrncy));
        assert!(mask.get_flag(TokenFlag::SAMCrncy));
        assert!(!mask.get_flag(TokenFlag::WrappedToken));

        assert_eq!(mask.decimals(), 9);
        assert_eq!(mask.bump_seed(), 255);

        mask.clear_flag(TokenFlag::BaseCrncy);
        assert!(!mask.get_flag(TokenFlag::BaseCrncy));

        assert_eq!(mask.decimals(), 9);
        assert_eq!(mask.bump_seed(), 255);
        assert!(mask.get_flag(TokenFlag::SAMCrncy));
    }
}
