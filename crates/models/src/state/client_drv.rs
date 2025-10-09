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
