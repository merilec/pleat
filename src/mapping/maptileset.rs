use crate::constants::*;
use crate::error::*;
use crate::graphics::*;
use crate::lz77::*;
use crate::mapping::*;
use crate::rom::*;

pub struct MapTileset {
    pub is_compressed: bool,
    pub use_sec_palettes: bool,
    pub padding: u16,
    pub tilemap_addr: usize,
    pub palettes_addr: usize,
    pub blockmap_addr: usize,
    pub tileset_callback_addr: usize,
    pub block_attributes_addr: usize,
    pub max_block_count: usize,
}

impl MapTileset {
    pub fn read(
        address: usize,
        rom: &mut Rom,
        max_block_count: usize,
    ) -> Result<MapTileset> {
        rom.seek_to(address)?;
        Ok(MapTileset {
            is_compressed: rom.read_u8()? != 0,
            use_sec_palettes: rom.read_u8()? != 0,
            padding: rom.read_u16()?,
            tilemap_addr: rom.read_address()?,
            palettes_addr: rom.read_address()?,
            blockmap_addr: rom.read_address()?,
            tileset_callback_addr: rom.read_address()?,
            block_attributes_addr: rom.read_address()?,
            max_block_count,
        })
    }
    pub fn get_tiles_data(&self, rom: &mut Rom) -> Result<Vec<Vec<u8>>> {
        let data = match self.is_compressed {
            true => lz77_decompress(self.tilemap_addr, rom)?,
            false => {
                rom.seek_to(self.tilemap_addr)?;
                rom.read_data(self.max_block_count * 0x20)
            }
        };
        let tile_length = SIZE_TILE * SIZE_TILE / 2;
        Ok(data.chunks(tile_length).map(|x| x.to_vec()).collect())
    }
    pub fn get_palettes(
        &self,
        rom: &mut Rom,
    ) -> Result<[[Color; NUM_COLORS_IN_PALETTE]; NUM_PALETTES_IN_TILESET]> {
        rom.seek_to(self.palettes_addr)?;
        let mut palettes: Vec<[Color; NUM_COLORS_IN_PALETTE]> = vec![];
        for _ in 0..NUM_PALETTES_IN_TILESET {
            let mut palette: Vec<Color> = vec![];
            for __ in 0..NUM_COLORS_IN_PALETTE {
                palette.push(Color::new(rom.read_u16()?));
            }
            palettes.push(palette.try_into().unwrap());
        }
        Ok(palettes.try_into().unwrap())
    }
    pub fn get_blocks(&self, rom: &mut Rom) -> Vec<Block> {
        // because there is no way of knowing exactly how many blocks a tileset
        // has, just keep trying to read blocks until:
        // - the block's address comes after the offset for block attributes (no
        //   longer reading blocks but block attributes instead), or
        // - the block read is invalid, or
        // - `max_block_count` is reached
        let mut blocks = vec![];
        for i in 0..self.max_block_count {
            let block_addr = self.blockmap_addr + (i * 16);
            let block_attr_addr = self.block_attributes_addr + (i * 4);
            if block_addr >= self.block_attributes_addr {
                break;
            }
            match Block::read(block_addr, block_attr_addr, rom) {
                Ok(block) => {
                    blocks.push(block);
                }
                Err(_) => {
                    break;
                }
            };
        }
        blocks
    }
    pub fn get_tilesheet_as_png(
        &self,
        rom: &mut Rom,
        palette_id: usize,
    ) -> Result<String> {
        const NUM_TILES_ACROSS: usize = 16;
        let tiles_data = self.get_tiles_data(rom)?;
        let num_tiles = tiles_data.len();
        let width = NUM_TILES_ACROSS;
        let height = (num_tiles + width - 1) / width;

        let mut data =
            vec![0u8; (width * height * SIZE_TILE * SIZE_TILE) as usize];
        for (tile_id, tile_data) in tiles_data.iter().enumerate() {
            let tile_data = tile_data
                .iter()
                .flat_map(|byte| [byte & 0xF, byte >> 4])
                .collect::<Vec<u8>>();
            let dx = (tile_id % NUM_TILES_ACROSS) * SIZE_TILE;
            let dy = (tile_id / NUM_TILES_ACROSS) * SIZE_TILE;
            for (i, pixel) in tile_data.iter().enumerate() {
                let dxx = i % SIZE_TILE;
                let dyy = i / SIZE_TILE;
                let index = (dy + dyy) * width * SIZE_TILE + (dx + dxx);
                data[index] = *pixel;
            }
        }

        let palette = self.get_palettes(rom)?[palette_id]
            .iter()
            .flat_map(|c| c.to_rgb())
            .collect::<Vec<u8>>();

        let mut png = vec![];
        {
            let image_width = (width * SIZE_TILE) as u32;
            let image_height = (height * SIZE_TILE) as u32;
            let mut encoder = png::Encoder::new(
                std::io::Cursor::new(&mut png),
                image_width,
                image_height,
            );
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
