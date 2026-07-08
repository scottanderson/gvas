#![cfg(feature = "palworld")]

//! Palworld save wrapper support.
//!
//! Palworld save files contain a 12-byte header followed by a [`USaveGame`]
//! which may be Zlib-deflated.

use std::io::{Cursor, Read, Seek, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian, binrw};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};

use crate::{error::binrw_custom, types::USaveGame};

#[binrw]
#[derive(Debug, PartialEq)]
struct PlZHeader {
    uncompressed_size: u32,
    compressed_size: u32,
    compression: PalworldCompression,
}

/// A Palworld save file containing a compressed [`USaveGame`].
///
/// Palworld wraps the Unreal save payload in a small `PlZ#` header that
/// records the compression method and payload sizes. The header sizes are
/// recalculated when writing.
#[derive(Debug, PartialEq)]
pub struct PalworldSaveGame {
    pub compression: PalworldCompression,
    pub content: USaveGame,
}

/// Compression methods used by [`PalworldSaveGame`].
#[binrw]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PalworldCompression {
    /// Uncompressed payload, identified by the `PlZ0` magic.
    #[brw(magic = b"PlZ0")]
    None,

    /// Payload compressed once with zlib, identified by the `PlZ1` magic.
    #[brw(magic = b"PlZ1")]
    Zlib,

    /// Payload compressed twice with zlib, identified by the `PlZ2` magic.
    #[brw(magic = b"PlZ2")]
    ZlibTwice,
}

impl BinRead for PalworldSaveGame {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (): Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        let header = PlZHeader::read_options(reader, endian, ())?;
        let uncompressed_size = header
            .uncompressed_size
            .try_into()
            .map_err(binrw_custom(pos))?;

        let compression = header.compression;
        let uncompressed = match compression {
            PalworldCompression::None => {
                let mut data = vec![0u8; uncompressed_size];
                reader.read_exact(&mut data)?;
                data
            }

            PalworldCompression::Zlib => {
                let mut reader = ZlibDecoder::new(reader);
                let mut data = vec![0u8; uncompressed_size];
                reader.read_exact(&mut data)?;
                data
            }

            PalworldCompression::ZlibTwice => {
                let mut reader = ZlibDecoder::new(ZlibDecoder::new(reader));
                let mut data = Vec::with_capacity(uncompressed_size);
                let _len = reader.read_to_end(&mut data)?;
                data
            }
        };

        let reader = &mut Cursor::new(uncompressed);
        let content = USaveGame::read_options(reader, endian, ())?;

        let result = Self {
            compression,
            content,
        };
        Ok(result)
    }
}

impl BinWrite for PalworldSaveGame {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        // Buffer uncompressed data
        let mut uncompressed = Cursor::new(Vec::new());
        self.content.write_options(&mut uncompressed, endian, ())?;
        let pos = writer.stream_position()?;
        let uncompressed = uncompressed.into_inner();
        let uncompressed_size = uncompressed.len().try_into().map_err(binrw_custom(pos))?;

        // Buffer compressed data
        let level = Compression::default();
        let compression = self.compression;
        let compressed = match compression {
            PalworldCompression::None => uncompressed,
            PalworldCompression::Zlib => {
                let tmp = Vec::new();
                let mut tmp = ZlibEncoder::new(tmp, level);
                tmp.write_all(uncompressed.as_slice())?;
                tmp.finish()?
            }
            PalworldCompression::ZlibTwice => {
                let tmp = Vec::new();
                let tmp = ZlibEncoder::new(tmp, level);
                let mut tmp = ZlibEncoder::new(tmp, level);
                tmp.write_all(uncompressed.as_slice())?;
                tmp.finish()?.finish()?
            }
        };
        let compressed_size = u32::try_from(compressed.len()).map_err(binrw_custom(pos))?;

        // Create a new header
        let header = PlZHeader {
            uncompressed_size,
            compressed_size,
            compression,
        };

        // Write
        header.write_options(writer, endian, ())?;
        writer.write_all(compressed.as_slice())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{
        assert_matches,
        fs::File,
        io::{Cursor, Read},
    };

    use binrw::{BinRead, BinWrite};

    use crate::{
        error::Result,
        palworld::{PalworldCompression, PalworldSaveGame},
        test::common::{PALWORLD_ZLIB_PATH, PALWORLD_ZLIB_TWICE_PATH},
        types::USaveGame,
    };

    #[test]
    fn zlib() -> Result<()> {
        // Open
        let mut file = File::open(PALWORLD_ZLIB_PATH)?;

        // Read
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let result = PalworldSaveGame::read_le(&mut cursor)?;

        // Compare
        assert_matches!(
            result,
            PalworldSaveGame {
                compression: PalworldCompression::Zlib,
                content: USaveGame { .. }
            }
        );

        // Write
        let buf2 = vec![0u8; len];
        let mut cursor2 = Cursor::new(buf2);
        PalworldSaveGame::write_le(&result, &mut cursor2)?;

        // Read again
        let buf2 = cursor2.into_inner();
        let mut cursor2 = Cursor::new(buf2);
        let result2 = PalworldSaveGame::read_le(&mut cursor2)?;

        // Compare
        assert!(result == result2);

        // Success
        Ok(())
    }

    #[test]
    fn zlib_twice() -> Result<()> {
        // Open
        let mut file = File::open(PALWORLD_ZLIB_TWICE_PATH)?;

        // Read
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let result = PalworldSaveGame::read_le(&mut cursor)?;

        // Compare
        assert_matches!(
            result,
            PalworldSaveGame {
                compression: PalworldCompression::ZlibTwice,
                content: USaveGame { .. },
            }
        );

        // Write
        let buf2 = vec![0u8; len];
        let mut cursor2 = Cursor::new(buf2);
        PalworldSaveGame::write_le(&result, &mut cursor2)?;

        // Read again
        let buf2 = cursor2.into_inner();
        let mut cursor2 = Cursor::new(buf2);
        let result2 = PalworldSaveGame::read_le(&mut cursor2)?;

        // Compare
        assert!(result == result2);

        // Success
        Ok(())
    }
}
