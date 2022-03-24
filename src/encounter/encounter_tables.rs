use serde::{Deserialize, Serialize};

use crate::encounter::*;
use crate::error::*;
use crate::rom::*;

const ENCOUNTER_TABLES_ADDRESS: usize = 0x3C9CB8;

#[derive(Debug, Deserialize, Serialize)]
pub struct EncounterTables {
    pub bank_num: usize,
    pub map_num: usize,
    #[serde(skip_serializing, skip_deserializing)]
    _padding: u16,
    pub grass_address: usize,
    pub surf_address: usize,
    pub rock_smash_address: usize,
    pub fishing_address: usize,
}

impl EncounterTables {
    pub fn get(
        rom: &mut Rom,
        bank_num: usize,
        map_num: usize,
    ) -> Result<Option<EncounterTables>> {
        let mut address = ENCOUNTER_TABLES_ADDRESS;
        loop {
            let tables = EncounterTables::read(address, rom)?;
            match (tables.bank_num, tables.map_num) {
                (0xFF, 0xFF) => {
                    return Ok(None);
                }
                (b, m) if (b, m) == (bank_num, map_num) => {
                    return Ok(Some(tables));
                }
                _ => {}
            }
            address += 20;
        }
    }
    pub fn read(address: usize, rom: &mut Rom) -> Result<EncounterTables> {
        rom.seek_to(address)?;
        Ok(EncounterTables {
            bank_num: rom.read_u8()? as usize,
            map_num: rom.read_u8()? as usize,
            _padding: rom.read_u16()?,
            grass_address: rom.read_address()?,
            surf_address: rom.read_address()?,
            rock_smash_address: rom.read_address()?,
            fishing_address: rom.read_address()?,
        })
    }
    pub fn get_encounter_table(
        &self,
        rom: &mut Rom,
        encounter_type: EncounterType,
    ) -> Result<Option<EncounterTable>> {
        if matches!(encounter_type, EncounterType::None) {
            return Ok(None);
        }
        let address = match encounter_type {
            EncounterType::None => unreachable!(),
            EncounterType::Grass => self.grass_address,
            EncounterType::Surf => self.surf_address,
            EncounterType::RockSmash => self.rock_smash_address,
            EncounterType::OldRod => self.fishing_address,
            EncounterType::GoodRod => self.fishing_address,
            EncounterType::SuperRod => self.fishing_address,
        };
        EncounterTable::read(address, rom, encounter_type)
    }
    pub fn get_grass_encounter_table(
        &self,
        rom: &mut Rom,
    ) -> Result<Option<EncounterTable>> {
        EncounterTable::read(self.grass_address, rom, EncounterType::Grass)
    }
    pub fn get_surf_encounter_table(
        &self,
        rom: &mut Rom,
    ) -> Result<Option<EncounterTable>> {
        EncounterTable::read(self.surf_address, rom, EncounterType::Surf)
    }
    pub fn get_rock_smash_encounter_table(
        &self,
        rom: &mut Rom,
    ) -> Result<Option<EncounterTable>> {
        EncounterTable::read(
            self.rock_smash_address,
            rom,
            EncounterType::RockSmash,
        )
    }
    pub fn get_old_rod_encounter_table(
        &self,
        rom: &mut Rom,
    ) -> Result<Option<EncounterTable>> {
        EncounterTable::read(self.fishing_address, rom, EncounterType::OldRod)
    }
    pub fn get_good_rod_encounter_table(
        &self,
        rom: &mut Rom,
    ) -> Result<Option<EncounterTable>> {
        EncounterTable::read(self.fishing_address, rom, EncounterType::GoodRod)
    }
    pub fn get_super_rod_encounter_table(
        &self,
        rom: &mut Rom,
    ) -> Result<Option<EncounterTable>> {
        EncounterTable::read(self.fishing_address, rom, EncounterType::SuperRod)
    }
}
