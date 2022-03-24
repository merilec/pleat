use serde::{Deserialize, Serialize};

use crate::error::*;
use crate::rom::*;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct EncounterEntry {
    pub min_level: u8,
    pub max_level: u8,
    pub species: u16,
}

impl EncounterEntry {
    pub fn read(address: usize, rom: &mut Rom) -> Result<EncounterEntry> {
        rom.seek_to(address)?;
        Ok(EncounterEntry {
            min_level: rom.read_u8()?,
            max_level: rom.read_u8()?,
            species: rom.read_u16()?,
        })
    }
    pub fn write(&self, address: usize, rom: &mut Rom) -> Result<()> {
        rom.seek_to(address)?;
        rom.write_u8(self.min_level)?;
        rom.write_u8(self.max_level)?;
        rom.write_u16(self.species)
    }
}
