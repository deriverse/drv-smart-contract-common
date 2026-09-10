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

    impl InstrFlag {
        pub const ALL: [InstrFlag; 11] = [
            InstrFlag::PerpActive,
            InstrFlag::ReadyToPerpUpgrade,
            InstrFlag::ZeroFees,
            InstrFlag::FixedFees,
            InstrFlag::SimilarAssets,
            InstrFlag::UsdStablecoin,
            InstrFlag::Forex,
            InstrFlag::Suspended,
            InstrFlag::LongMarginCall,
            InstrFlag::ShortMarginCall,
            InstrFlag::ExpandableCandles,
        ];
    }

    impl std::fmt::Display for InstrFlag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    const fn bits(flags: &[InstrFlag]) -> u32 {
        let mut out = 0;
        let mut i = 0;
        while i < flags.len() {
            out |= flags[i] as u32;
            i += 1;
        }
        out
    }

    struct FlagRule {
        flag: InstrFlag,
        requires: &'static [u32],
        forbids: u32,
    }

    const RULES: &[FlagRule] = &[
        FlagRule {
            flag: InstrFlag::PerpActive,
            requires: &[],
            forbids: bits(&[InstrFlag::ReadyToPerpUpgrade, InstrFlag::SimilarAssets]),
        },
        FlagRule {
            flag: InstrFlag::ReadyToPerpUpgrade,
            requires: &[],
            forbids: bits(&[InstrFlag::PerpActive, InstrFlag::SimilarAssets]),
        },
        FlagRule {
            flag: InstrFlag::LongMarginCall,
            requires: &[InstrFlag::PerpActive as u32],
            forbids: 0,
        },
        FlagRule {
            flag: InstrFlag::ShortMarginCall,
            requires: &[InstrFlag::PerpActive as u32],
            forbids: 0,
        },
        FlagRule {
            flag: InstrFlag::SimilarAssets,
            requires: &[bits(&[InstrFlag::ZeroFees, InstrFlag::FixedFees])],
            forbids: bits(&[
                InstrFlag::Forex,
                InstrFlag::PerpActive,
                InstrFlag::ReadyToPerpUpgrade,
            ]),
        },
        FlagRule {
            flag: InstrFlag::ZeroFees,
            requires: &[InstrFlag::SimilarAssets as u32],
            forbids: InstrFlag::FixedFees as u32,
        },
        FlagRule {
            flag: InstrFlag::FixedFees,
            requires: &[InstrFlag::SimilarAssets as u32],
            forbids: InstrFlag::ZeroFees as u32,
        },
        FlagRule {
            flag: InstrFlag::UsdStablecoin,
            requires: &[InstrFlag::SimilarAssets as u32],
            forbids: 0,
        },
        FlagRule {
            flag: InstrFlag::Forex,
            requires: &[],
            forbids: InstrFlag::SimilarAssets as u32,
        },
    ];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum InstrMaskError {
        UnknownBits(u32),
        MissingRequired { flag: InstrFlag, required: u32 },
        Forbidden { flag: InstrFlag, forbidden: u32 },
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
        pub const KNOWN_FLAGS: u32 = bits(&InstrFlag::ALL);

        pub fn validate(&self) -> Result<(), InstrMaskError> {
            let raw_mask = self.0;

            let unknown = raw_mask & !Self::KNOWN_FLAGS;
            if unknown != 0 {
                return Err(InstrMaskError::UnknownBits(unknown));
            }

            for rule in RULES {
                if raw_mask & rule.flag as u32 == 0 {
                    continue;
                }
                for &clause in rule.requires {
                    if raw_mask & clause == 0 {
                        return Err(InstrMaskError::MissingRequired {
                            flag: rule.flag,
                            required: clause,
                        });
                    }
                }
                let forbidden = raw_mask & rule.forbids;
                if forbidden != 0 {
                    return Err(InstrMaskError::Forbidden {
                        flag: rule.flag,
                        forbidden,
                    });
                }
            }
            Ok(())
        }

        pub fn merge(&mut self, input: InstrInputMask) {
            self.0 |= (input.0 as u32) & InstrInputMask::ALLOWED_FLAGS;
        }

        pub fn try_merge(&mut self, input: InstrInputMask) -> Result<(), InstrMaskError> {
            let mut next = *self;
            next.merge(input);
            next.validate()?;
            *self = next;
            Ok(())
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
            | InstrFlag::UsdStablecoin as u32
            | InstrFlag::Forex as u32;
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

    #[cfg(test)]
    mod tests {
        use super::*;

        fn mask(flags: &[InstrFlag]) -> InstrMask {
            InstrMask(bits(flags))
        }

        fn input(flags: &[InstrFlag]) -> InstrInputMask {
            let mut m = InstrInputMask(0);
            for &f in flags {
                m.set_flag(f);
            }
            m
        }

        #[test]
        fn valid_masks() {
            for flags in [
                &[][..],
                &[InstrFlag::Suspended, InstrFlag::ExpandableCandles],
                &[InstrFlag::ReadyToPerpUpgrade],
                &[
                    InstrFlag::PerpActive,
                    InstrFlag::LongMarginCall,
                    InstrFlag::ShortMarginCall,
                ],
                &[InstrFlag::SimilarAssets, InstrFlag::ZeroFees],
                &[
                    InstrFlag::SimilarAssets,
                    InstrFlag::FixedFees,
                    InstrFlag::UsdStablecoin,
                ],
                &[InstrFlag::Forex, InstrFlag::PerpActive],
            ] {
                assert_eq!(mask(flags).validate(), Ok(()), "{flags:?}");
            }
        }

        #[test]
        fn invalid_masks() {
            for flags in [
                &[InstrFlag::PerpActive, InstrFlag::ReadyToPerpUpgrade][..],
                &[InstrFlag::LongMarginCall],
                &[InstrFlag::ReadyToPerpUpgrade, InstrFlag::ShortMarginCall],
                &[InstrFlag::SimilarAssets],
                &[
                    InstrFlag::SimilarAssets,
                    InstrFlag::ZeroFees,
                    InstrFlag::FixedFees,
                ],
                &[InstrFlag::ZeroFees],
                &[InstrFlag::UsdStablecoin],
                &[
                    InstrFlag::SimilarAssets,
                    InstrFlag::ZeroFees,
                    InstrFlag::Forex,
                ],
                &[
                    InstrFlag::SimilarAssets,
                    InstrFlag::ZeroFees,
                    InstrFlag::PerpActive,
                ],
                &[
                    InstrFlag::SimilarAssets,
                    InstrFlag::ZeroFees,
                    InstrFlag::ReadyToPerpUpgrade,
                ],
            ] {
                assert!(mask(flags).validate().is_err(), "{flags:?}");
            }
        }

        #[test]
        fn unknown_bits_rejected() {
            assert_eq!(
                InstrMask(0x8000_0000).validate(),
                Err(InstrMaskError::UnknownBits(0x8000_0000))
            );
        }

        #[test]
        fn merge_test() {
            let mut m = InstrMask(0);
            let err = m.try_merge(input(&[
                InstrFlag::SimilarAssets,
                InstrFlag::ZeroFees,
                InstrFlag::Forex,
            ]));
            assert!(err.is_err());
            assert_eq!(m, InstrMask(0));

            let mut m = mask(&[InstrFlag::SimilarAssets, InstrFlag::ZeroFees]);
            m.set_flag(InstrFlag::ReadyToPerpUpgrade);
            assert!(m.validate().is_err());

            let mut m = InstrMask(0);
            m.try_merge(input(&[
                InstrFlag::SimilarAssets,
                InstrFlag::FixedFees,
                InstrFlag::UsdStablecoin,
            ]))
            .unwrap();
            assert!(m.get_flag(InstrFlag::SimilarAssets));
            assert!(m.get_flag(InstrFlag::FixedFees));
            assert!(m.get_flag(InstrFlag::UsdStablecoin));

            let mut i = InstrInputMask(0);
            i.set_flag(InstrFlag::Suspended);
            let mut m = InstrMask(0);
            m.try_merge(i).unwrap();
            assert!(!m.get_flag(InstrFlag::Suspended));
        }
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
