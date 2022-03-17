use std::fmt;

use crate::error::*;
use crate::mapping::*;
use crate::rom::*;

pub struct MapHeader {
    pub bank_num: usize,
    pub map_num: usize,
    pub layout_addr: usize,
    pub events_addr: usize,
    // and many more other fields irrelevant for now
}

impl fmt::Display for MapHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MapHeader ({}.{})", self.bank_num, self.map_num)
    }
}

impl MapHeader {
    pub fn read(
        address: usize,
        rom: &mut Rom,
        bank_num: usize,
        map_num: usize,
    ) -> Result<MapHeader> {
        rom.seek_to(address)?;
        Ok(MapHeader {
            layout_addr: rom.read_address()?,
            events_addr: rom.read_address()?,
            bank_num,
            map_num,
        })
    }
    pub fn get_map_layout(&self, rom: &mut Rom) -> Result<MapLayout> {
        MapLayout::read(self.layout_addr, rom)
    }
    pub fn get_map_name(&self, _rom: &mut Rom) -> String {
        unimplemented!();
    }
}
