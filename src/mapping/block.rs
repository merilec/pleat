use std::fmt;

use crate::encounter::*;
use crate::error::*;
use crate::mapping::*;
use crate::rom::*;

#[derive(Clone, Copy, Debug)]
pub enum Background {
    Normal = 0,  // middle + top
    Covered = 2, // bottom + middle (Block is covered by Hero)
    Triple = 3,  // https://www.pokecommunity.com/showthread.php?t=352725
    Split = 4,   // bottom + top
}

#[derive(Clone, Copy, Debug)]
pub enum Terrain {
    Normal = 0,
    Grass = 1,     // cuttable-grass
    Water = 2,     // allowed to use rod, surf
    Waterfall = 3, // unused in game code, but used by some blocks
}

#[derive(Debug)]
pub enum InvalidBlock {
    InvalidTerrain(u32),
    InvalidBackground(u32),
    InvalidEncounter(u32),
}
impl std::error::Error for InvalidBlock {}
impl fmt::Display for InvalidBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidBlock::InvalidTerrain(value) => {
                write!(
                    f,
                    "Unknown terrain ({:#x}) (expected 0, 1, 2, or 3)!",
                    value
                )
            }
            InvalidBlock::InvalidBackground(value) => {
                write!(
                    f,
                    "Unknown background ({:#x}) (expected 0, 2, 3, or 4)!",
                    value
                )
            }
            InvalidBlock::InvalidEncounter(value) => {
                write!(
                    f,
                    "Unknown encounter ({:#x}) (expected 0, 1, or 2)!",
                    value
                )
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub tiles: [Tile; 8],
    // this structure is for FRLG, entirely different in RSE
    // behavior    0x00 - 0x08
    // terrain     0x09 - 0x0D
    // <unused>    0x0E - 0x17
    // encounter   0x18 - 0x1A
    // <unused>       0x1B
    // background  0x1C - 0x1E
    // <unused>       0x1F
    pub behavior: u16,
    pub terrain: Terrain,
    pub encounter: EncounterType,
    pub background: Background,
}

impl Block {
    pub fn read(
        block_address: usize,
        block_attr_address: usize,
        rom: &mut Rom,
    ) -> Result<Block> {
        rom.seek_to(block_address)?;
        let mut read_tile =
            |tile_num: usize| Tile::read(block_address + tile_num * 2, rom);
        let tiles: [Tile; 8] = [
            read_tile(0)?,
            read_tile(1)?,
            read_tile(2)?,
            read_tile(3)?,
            read_tile(4)?,
            read_tile(5)?,
            read_tile(6)?,
            read_tile(7)?,
        ];

        rom.seek_to(block_attr_address)?;
        let value = rom.read_u32()?;
        let behavior = (value & 0x1FF) as u16;
        let tr = (value & 0x3E00) >> 9;
        let terrain = match tr {
            0 => Terrain::Normal,
            1 => Terrain::Grass,
            2 => Terrain::Water,
            3 => Terrain::Waterfall,
            _ => Err(InvalidBlock::InvalidTerrain(tr))?,
        };
        let en = (value & 0x7000000) >> 24;
        let encounter = match en {
            0 => EncounterType::None,
            1 => EncounterType::Grass,
            2 => EncounterType::Surf,
            _ => return Err(InvalidBlock::InvalidEncounter(en))?,
        };
        let bg = (value & 0x70000000) >> 28;
        let background = match bg {
            0 => Background::Normal,
            2 => Background::Covered,
            3 => Background::Triple,
            4 => Background::Split,
            _ => return Err(InvalidBlock::InvalidBackground(bg))?,
        };

        Ok(Block {
            tiles,
            behavior,
            terrain,
            encounter,
            background,
        })
    }
}
