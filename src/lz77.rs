use std::fmt;

use crate::rom::*;

#[derive(Debug)]
pub enum LzError {
    DecompressionError,
    RomError(RomError),
}
impl std::error::Error for LzError {}
impl fmt::Display for LzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LzError::DecompressionError => {
                write!(f, "Error performing LZ-decompression on data!")
            }
            LzError::RomError(err) => err.fmt(f),
        }
    }
}
impl From<RomError> for LzError {
    fn from(err: RomError) -> LzError {
        LzError::RomError(err)
    }
}

// https://github.com/haven1433/HexManiacAdvance/blob/a25a2e0ce6589883358af599939f2b224f29c7c8/src/HexManiac.Core/Models/Runs/Sprites/LZRun.cs#L80
pub fn lz77_decompress(
    address: usize,
    rom: &mut Rom,
) -> Result<Vec<u8>, LzError> {
    rom.seek_to(address)?;
    let length = get_uncompressed_length(rom)?;
    let mut output: Vec<u8> = vec![0; length];
    let mut index = 0;
    while index < output.len() {
        let mut bitfield = rom.read_u8()?;
        for _ in 0..8 {
            if index > output.len() {
                break;
            }
            if index == output.len() {
                return match bitfield {
                    0 => Ok(output),
                    _ => Err(LzError::DecompressionError),
                };
            }
            let is_compressed_token = (bitfield & 0x80) != 0;
            bitfield = (bitfield << 1) & 0xFF;
            if !is_compressed_token {
                output[index] = rom.read_u8()?;
                index += 1;
            } else {
                let (run_length, run_offset) = read_compressed_token(rom)?;
                if index < run_offset {
                    return Err(LzError::DecompressionError);
                }
                for j in 0..run_length {
                    output[index + j] = output[index + j - run_offset];
                }
                index += run_length;
            }
        }
        if bitfield != 0 {
            return Err(LzError::DecompressionError);
        }
    }
    if index != output.len() {
        return Err(LzError::DecompressionError);
    }
    Ok(output)
}

fn get_uncompressed_length(rom: &mut Rom) -> Result<usize, LzError> {
    match rom.read_u8()? {
        0x10 => Ok(rom.read_u8()? as usize
            + rom.read_u8()? as usize * (1 << 8)
            + rom.read_u8()? as usize * (1 << 16)),
        _ => Err(LzError::DecompressionError),
    }
}

fn read_compressed_token(rom: &mut Rom) -> Result<(usize, usize), LzError> {
    let byte1 = rom.read_u8()? as usize;
    let byte2 = rom.read_u8()? as usize;
    let run_length = (byte1 >> 4) + 3;
    let run_offset = (((byte1 & 0xF) << 8) | byte2) + 1;
    Ok((run_length, run_offset))
}
