use crate::constants::*;
use crate::graphics::*;
use crate::lz77::*;
use crate::mapping::*;
use crate::rom::*;

pub struct MapLayout {
    pub width: u32,
    pub height: u32,
    pub border_blocks_addr: usize,
    pub map_blocks_addr: usize,
    pub pri_tileset_addr: usize,
    pub sec_tileset_addr: usize,
    pub border_width: u8,
    pub border_height: u8,
}

impl MapLayout {
    pub fn read(address: usize, rom: &mut Rom) -> Result<MapLayout, RomError> {
        rom.seek_to(address)?;
        Ok(MapLayout {
            width: rom.read_u32()?,
            height: rom.read_u32()?,
            border_blocks_addr: rom.read_address()?,
            map_blocks_addr: rom.read_address()?,
            pri_tileset_addr: rom.read_address()?,
            sec_tileset_addr: rom.read_address()?,
            border_width: rom.read_u8()?,
            border_height: rom.read_u8()?,
        })
    }
    pub fn get_map_blocks(
        &self,
        rom: &mut Rom,
    ) -> Result<Vec<MapBlock>, RomError> {
        let mut mapblocks: Vec<MapBlock> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                mapblocks
                    .push(self.get_map_block_at(rom, x as usize, y as usize)?);
            }
        }
        Ok(mapblocks)
    }
    pub fn get_map_block_at(
        &self,
        rom: &mut Rom,
        x: usize,
        y: usize,
    ) -> Result<MapBlock, RomError> {
        let map_block_addr =
            self.map_blocks_addr + (y * 2 * self.width as usize) + (x * 2);
        MapBlock::read(map_block_addr, rom)
    }
    pub fn set_map_block_at(
        &self,
        rom: &mut Rom,
        x: usize,
        y: usize,
        map_block: &MapBlock,
    ) -> Result<(), RomError> {
        let map_block_addr =
            self.map_blocks_addr + (y * 2 * self.width as usize) + (x * 2);
        map_block.write(map_block_addr, rom)
    }
    pub fn get_pri_tileset(
        &self,
        rom: &mut Rom,
    ) -> Result<MapTileset, RomError> {
        MapTileset::read(self.pri_tileset_addr, rom, MAX_NUM_PRIMARY_BLOCKS)
    }
    pub fn get_sec_tileset(
        &self,
        rom: &mut Rom,
    ) -> Result<MapTileset, RomError> {
        MapTileset::read(self.sec_tileset_addr, rom, MAX_NUM_SECONDARY_BLOCKS)
    }
    pub fn get_palettes(
        &self,
        rom: &mut Rom,
    ) -> Result<
        [[Color; NUM_COLORS_IN_PALETTE]; NUM_PALETTES_IN_TILESET],
        RomError,
    > {
        let pri_palettes = self.get_pri_tileset(rom)?.get_palettes(rom)?; // 0 - 6
        let sec_palettes = self.get_sec_tileset(rom)?.get_palettes(rom)?; // 7 - F
        Ok([&pri_palettes[0..=6], &sec_palettes[7..=0xF]]
            .concat()
            .try_into()
            .unwrap())
    }
    pub fn get_tiles_data(
        &self,
        rom: &mut Rom,
    ) -> Result<Vec<Vec<u8>>, LzError> {
        let pri_tiles_data = self.get_pri_tileset(rom)?.get_tiles_data(rom);
        let sec_tiles_data = self.get_sec_tileset(rom)?.get_tiles_data(rom);
        Ok([&pri_tiles_data?[..], &sec_tiles_data?[..]]
            .concat()
            .try_into()
            .unwrap())
    }
    pub fn get_blocks(&self, rom: &mut Rom) -> Result<Vec<Block>, RomError> {
        let pri_blocks = self.get_pri_tileset(rom)?.get_blocks(rom);
        let sec_blocks = self.get_sec_tileset(rom)?.get_blocks(rom);
        Ok([pri_blocks, sec_blocks].concat())
    }
    pub fn get_blocksheet_as_png(
        &self,
        rom: &mut Rom,
    ) -> Result<String, LzError> {
        let palette = self
            .get_palettes(rom)?
            .iter()
            .flat_map(|pal| *pal)
            .flat_map(|c| c.to_rgb())
            .collect::<Vec<u8>>();

        const NUM_BLOCKS_ACROSS: usize = 8;
        let blocks = self.get_blocks(rom)?;
        let num_blocks = blocks.len();
        let width = NUM_BLOCKS_ACROSS * SIZE_BLOCK;
        let height = ((num_blocks + NUM_BLOCKS_ACROSS - 1) / NUM_BLOCKS_ACROSS)
            * SIZE_BLOCK;

        let tiles_data = self.get_tiles_data(rom)?;

        let mut data = vec![0u8; (width * height) as usize];
        for (block_id, block) in blocks.iter().enumerate() {
            let dx = (block_id % NUM_BLOCKS_ACROSS) * SIZE_BLOCK;
            let dy = (block_id / NUM_BLOCKS_ACROSS) * SIZE_BLOCK;
            for (tile_num, tile) in block.tiles.iter().enumerate() {
                let tile_id = if (tile.tile_id as usize) < tiles_data.len() {
                    tile.tile_id as usize
                } else {
                    0
                };
                let tile_data = tiles_data[tile_id]
                    .iter()
                    .flat_map(|byte| [byte & 0xF, byte >> 4])
                    .collect::<Vec<u8>>();
                let dxx = (tile_num % 2) * SIZE_TILE;
                let dyy = ((tile_num % 4) / 2) * SIZE_TILE;
                for (i, pixel) in tile_data.iter().enumerate() {
                    if *pixel == 0 {
                        continue;
                    }
                    let pixel =
                        pixel + tile.palette_id * NUM_COLORS_IN_PALETTE as u8;
                    let dxxx = match tile.h_flip {
                        true => SIZE_TILE - (i % SIZE_TILE) - 1,
                        false => i % SIZE_TILE,
                    };
                    let dyyy = match tile.v_flip {
                        true => SIZE_TILE - (i / SIZE_TILE) - 1,
                        false => i / SIZE_TILE,
                    };
                    let index =
                        (dy + dyy + dyyy) * NUM_BLOCKS_ACROSS * SIZE_BLOCK
                            + (dx + dxx + dxxx);
                    data[index] = pixel;
                }
            }
        }

        let mut png = vec![];
        {
            let cursor = std::io::Cursor::new(&mut png);
            let mut encoder =
                png::Encoder::new(cursor, width as u32, height as u32);
            encoder.set_color(png::ColorType::Indexed);
            encoder.set_palette(std::borrow::Cow::Owned(palette));
            encoder.set_compression(png::Compression::Best);
            encoder
                .write_header()
                .unwrap()
                .write_image_data(&data)
                .unwrap();
        }
        Ok(base64::encode(png))
    }
}
