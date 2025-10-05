use crate::new_types::client::ClientId;
use bytemuck::{Pod, Zeroable};

use super::types::Discriminator;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ClientDrvAccountHeader {
    pub discriminator: Discriminator,
    pub id: ClientId,
    pub count: u32,
    pub slot: u32,
    pub reserved: u32,
}

pub const CLIENT_DRV_ACCOUNT_HEADER_SIZE: usize = std::mem::size_of::<ClientDrvAccountHeader>();
