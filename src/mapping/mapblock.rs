use serde::{Deserialize, Serialize};

use crate::error::*;
use crate::rom::*;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MapBlock {
    pub block_id: u16,  // 0 - 9
    pub permission: u8, // A - B: collision; C - F: elevation
}

impl MapBlock {
    pub fn read(address: usize, rom: &mut Rom) -> Result<Self> {
        rom.seek_to(address)?;
        let value = rom.read_u16()?;
        Ok(Self {
            block_id: value & 0x3FF,
            permission: (value >> 0xA) as u8,
        })
    }
    pub fn write(&self, address: usize, rom: &mut Rom) -> Result<()> {
        rom.seek_to(address)?;
        let value = self.block_id | (self.permission as u16) << 0xA;
        rom.write_u16(value)
    }
}
