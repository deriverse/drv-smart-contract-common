use bytemuck::{Pod, Zeroable};

#[cfg(test)]
mod tests {
    use bytemuck::{Pod, Zeroable};
    use drv_macros::new_type;

    #[repr(transparent)]
    #[new_type]
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable, Pod, Default)]
    pub struct Wrapper(pub u32);

    const VAL: u32 = 10;

    #[test]
    fn test_new_macro() {
        let wrapper = Wrapper(VAL);

        assert_eq!(wrapper, VAL);
        assert!(wrapper > VAL - 1);
        let val: u32 = *wrapper;

        assert_eq!(val, VAL as u32);
    }
}

pub mod version {
    use super::{Pod, Zeroable};
    use drv_macros::new_type;

    #[repr(transparent)]
    #[new_type]
    #[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod, Default)]
    /// A type-safe wrapper around 'u32' that represents the version of a
    /// contract Ensures that version are handled correctly and prevents
    /// accidental misuse of raw integer values.
    pub struct Version(pub u32);
}

pub mod tag {
    use super::{Pod, Zeroable};
    use drv_macros::new_type;

    #[repr(transparent)]
    #[new_type]
    #[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod, Default)]
    /// A type-safe wrapper around 'u32' that represents the Tag of an account
    /// Ensures that Tags are handled correctly and prevents
    /// accidental misuse of raw integer values.
    pub struct Tag(pub u32);
}

pub mod instrument {
    use super::{Pod, Zeroable};
    use drv_macros::new_type;

    #[repr(transparent)]
    #[new_type]
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable, Pod, Default)]
    /// A type-sfe wrapper around 'u32' that represents an id of an instrument
    /// Ensures that ids are handled correctly and prevents accidental
    /// misuse of raw integer values.
    pub struct InstrId(pub u32);
}

pub mod client {
    use std::fmt::Display;

    use super::{Pod, Zeroable};
    use drv_macros::new_type;

    #[repr(transparent)]
    #[new_type]
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable, Pod, Default)]
    /// A type-safe wrapper around 'u32' that represents an id of a client
    ///
    /// Ensures that ids are handled correctly and prevents accidental
    /// misuse of raw integer values.
    pub struct ClientId(pub u32);

    impl Display for ClientId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Client {}", self.0)
        }
    }
}
