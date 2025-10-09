use bytemuck::Zeroable;

use crate::new_types::version::Version;

#[repr(C)]
#[derive(Zeroable)]
/// New path - src/state/pdf.rs
pub struct PdfAccountHeader {
    pub tag: u32,
    pub version: Version,
}

pub const PDF_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<PdfAccountHeader>();
