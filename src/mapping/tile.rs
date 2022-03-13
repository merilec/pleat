use crate::rom::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tile_id: u16,   // 0 - 9
    pub h_flip: bool,   // A
    pub v_flip: bool,   // B
    pub palette_id: u8, // C - F
}

impl Tile {
    pub fn read(address: usize, rom: &mut Rom) -> Result<Tile, RomError> {
        rom.seek_to(address)?;
        let value = rom.read_u16()?;
        Ok(Tile {
            tile_id: value & 0x3FF,
            h_flip: ((value >> 0xA) & 1) != 0,
            v_flip: ((value >> 0xB) & 1) != 0,
            palette_id: (value >> 0xC) as u8,
        })
    }
}
