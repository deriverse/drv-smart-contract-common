use std::mem::size_of;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Zeroable, Pod)]
pub struct HolderAccountHeader {
    pub tag: u32,
    pub operators_count: u32,
}

pub const HOLDER_ACCOUNT_HEADER_SIZE: usize = size_of::<HolderAccountHeader>();
