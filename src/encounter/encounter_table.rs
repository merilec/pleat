use serde::{Deserialize, Serialize};

use crate::encounter::{constants::*, *};
use crate::error::*;
use crate::rom::*;

#[derive(Clone, Copy, Debug)]
pub enum EncounterType {
    None = 0,
    Grass,
    Surf,
    RockSmash,
    OldRod,
    GoodRod,
    SuperRod,
}

impl Default for EncounterType {
    fn default() -> EncounterType { EncounterType::None }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncounterTable {
    #[serde(skip_serializing, skip_deserializing)]
    pub encounter_type: EncounterType,
    pub encounter_rate: u32,
    pub entries_address: usize,
}

impl EncounterTable {
    pub fn read(
        address: usize,
        rom: &mut Rom,
        encounter_type: EncounterType,
    ) -> Result<Option<EncounterTable>> {
        if address == 0 || matches!(encounter_type, EncounterType::None) {
            return Ok(None);
        }
        rom.seek_to(address)?;
        Ok(Some(EncounterTable {
            encounter_type,
            encounter_rate: rom.read_u32()?,
            entries_address: rom.read_address()?,
        }))
    }
    pub fn get_entries(&self, rom: &mut Rom) -> Result<Vec<EncounterEntry>> {
        let encounter_table_size = match self.encounter_type {
            EncounterType::None => unreachable!(),
            EncounterType::Grass => ENCOUNTER_TABLE_SIZE_GRASS,
            EncounterType::Surf => ENCOUNTER_TABLE_SIZE_SURF,
            EncounterType::RockSmash => ENCOUNTER_TABLE_SIZE_ROCK_SMASH,
            EncounterType::OldRod => ENCOUNTER_TABLE_SIZE_OLD_ROD,
            EncounterType::GoodRod => ENCOUNTER_TABLE_SIZE_GOOD_ROD,
            EncounterType::SuperRod => ENCOUNTER_TABLE_SIZE_SUPER_ROD,
        };
        let offset = match self.encounter_type {
            EncounterType::OldRod => 0,
            EncounterType::GoodRod => ENCOUNTER_TABLE_SIZE_OLD_ROD,
            EncounterType::SuperRod => {
                ENCOUNTER_TABLE_SIZE_OLD_ROD + ENCOUNTER_TABLE_SIZE_GOOD_ROD
            }
            _ => 0,
        };
        let mut read_entry = |entry_num: usize| {
            EncounterEntry::read(self.entries_address + entry_num * 4, rom)
        };
        let mut entries = vec![];
        for i in offset..(offset + encounter_table_size) {
            entries.push(read_entry(i)?);
        }
        Ok(entries)
    }
}
