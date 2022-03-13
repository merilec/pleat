use std::fmt;

use crate::mapping::*;

const MAP_BANK_TABLE_POINTER: usize = 0x05524C;

const BANK_SIZES_FR: [u8; 43] = [
    5, 123, 60, 66, 4, 6, 8, 10, 6, 8, 20, 10, 8, 2, 10, 4, 2, 2, 2, 1, 1, 2,
    2, 3, 2, 3, 2, 1, 1, 1, 1, 7, 5, 5, 8, 8, 5, 5, 1, 1, 1, 2, 1,
];

#[derive(Debug)]
pub enum RomError {
    SeekToNullError,
    OutOfBoundsError(usize),
    InvalidAddress(usize, u32),
}
impl std::error::Error for RomError {}
impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RomError::SeekToNullError => {
                write!(f, "Cannot seek to null address!")
            }
            RomError::OutOfBoundsError(address) => {
                write!(
                    f,
                    "Cannot access address at {:#08x} (out of bounds)!",
                    address
                )
            }
            RomError::InvalidAddress(address, value) => {
                let bytes = value.to_le_bytes();
                write!(
                    f,
                    "Cannot read \"{:02x} {:02x} {:02x} {:02x}\" at {:#08x} as address!",
                    bytes[0], bytes[1], bytes[2], bytes[3], address
                )
            }
        }
    }
}

pub struct Rom {
    data: Vec<u8>,
    pos: usize,
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Rom {
        Rom { data, pos: 0 }
    }
    pub fn get_data(&self) -> Vec<u8> {
        self.data.to_vec()
    }
    pub fn seek_to(&mut self, address: usize) -> Result<(), RomError> {
        return match address {
            0 => Err(RomError::SeekToNullError),
            0x8000000..=0xA000000 => Ok(self.pos = address - 0x8000000),
            _ => {
                return if address >= self.data.len() {
                    Err(RomError::OutOfBoundsError(address))
                } else {
                    Ok(self.pos = address)
                };
            }
        };
    }
    pub fn seek_to_address_read(&mut self) -> Result<(), RomError> {
        let address = self.read_address()?;
        self.seek_to(address)
    }
    pub fn read_u8(&mut self) -> Result<u8, RomError> {
        if self.pos > self.data.len() {
            return Err(RomError::OutOfBoundsError(self.pos));
        }
        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }
    pub fn read_u16(&mut self) -> Result<u16, RomError> {
        if (self.pos + 1) > self.data.len() {
            return Err(RomError::OutOfBoundsError(self.pos + 1));
        }
        let value = self.data[self.pos] as u16
            + (self.data[self.pos + 1] as u16) * (1 << 8);
        self.pos += 2;
        Ok(value)
    }
    pub fn read_u32(&mut self) -> Result<u32, RomError> {
        if (self.pos + 3) > self.data.len() {
            return Err(RomError::OutOfBoundsError(self.pos + 3));
        }
        let value = self.data[self.pos] as u32
            + (self.data[self.pos + 1] as u32) * (1 << 8)
            + (self.data[self.pos + 2] as u32) * (1 << 16)
            + (self.data[self.pos + 3] as u32) * (1 << 24);
        self.pos += 4;
        Ok(value)
    }
    pub fn read_address(&mut self) -> Result<usize, RomError> {
        let address = self.read_u32()?;
        match address {
            0 => Ok(0),
            0x8000000..=0xA000000 => Ok((address - 0x8000000) as usize),
            _ => {
                self.pos -= 4;
                Err(RomError::InvalidAddress(self.pos, address))
            }
        }
    }
    pub fn read_data(&mut self, length: usize) -> Vec<u8> {
        let data = (&self.data[self.pos..self.pos + length]).to_vec();
        self.pos += length;
        data
    }
    pub fn write_u8(&mut self, value: u8) -> Result<(), RomError> {
        if self.pos > self.data.len() {
            return Err(RomError::OutOfBoundsError(self.pos));
        }
        self.data[self.pos] = value;
        Ok(self.pos += 1)
    }
    pub fn write_u16(&mut self, value: u16) -> Result<(), RomError> {
        if (self.pos + 1) > self.data.len() {
            return Err(RomError::OutOfBoundsError(self.pos + 1));
        }
        self.data[self.pos + 0] = (value & 0x00FF) as u8;
        self.data[self.pos + 1] = ((value & 0xFF00) >> 8) as u8;
        Ok(self.pos += 2)
    }
    pub fn write_u32(&mut self, value: u32) -> Result<(), RomError> {
        if (self.pos + 3) > self.data.len() {
            return Err(RomError::OutOfBoundsError(self.pos + 3));
        }
        self.data[self.pos + 0] = (value & 0x000000FF) as u8;
        self.data[self.pos + 1] = ((value & 0x0000FF00) >> 8) as u8;
        self.data[self.pos + 2] = ((value & 0x00FF0000) >> 16) as u8;
        self.data[self.pos + 3] = ((value & 0xFF000000) >> 24) as u8;
        Ok(self.pos += 4)
    }
    pub fn get_map_banks_names(
        &mut self,
    ) -> Result<Vec<Vec<String>>, RomError> {
        let mut banks = vec![];
        //TODO: iterate through banks without hard-coded bank sizes
        for (bank_num, bank_size) in BANK_SIZES_FR.iter().enumerate() {
            let mut bank = vec![];
            for map_num in 0..*bank_size {
                // bank.push(self.get_map_header(bank_num, map_num as usize).get_map_name(self));
                self.seek_to_map(bank_num, map_num as usize)?;
                bank.push(format!("{:#x}", self.pos));
            }
            banks.push(bank.try_into().unwrap());
        }
        Ok(banks)
    }
    pub fn get_map_header(
        &mut self,
        bank_num: usize,
        map_num: usize,
    ) -> Result<MapHeader, RomError> {
        self.seek_to_map(bank_num, map_num)?;
        MapHeader::read(self.pos, self, bank_num, map_num)
    }

    fn seek_to_map(
        &mut self,
        bank_num: usize,
        map_num: usize,
    ) -> Result<(), RomError> {
        self.seek_to(MAP_BANK_TABLE_POINTER)?;
        let bank_addr_pointer = self.read_address()? + bank_num * 4;
        self.seek_to(bank_addr_pointer)?;
        let map_addr_pointer = self.read_address()? + map_num * 4;
        self.seek_to(map_addr_pointer)?;
        self.seek_to_address_read()
    }
}
