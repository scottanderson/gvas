use std::io::{Read, Seek, Write};

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
};

/// Compression enum used in the SerializeCompressedNew format
/// Ref /Engine/Source/Runtime/Core/Private/Compression/CompressionUtil.cpp
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i8)]
pub enum ArchiveCompressionType {
    /// Named compression algorithm.
    Named = 0,
    /// No compression.
    None = 1,
    /// Oodle compression.
    Oodle = 2,
    /// Zlib compression.
    Zlib = 3,
    /// Gzip compression.
    Gzip = 4,
    /// LZ4 compression.
    LZ4 = 5,
}

/// Archive compression struct
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FArchiveCompression {
    /// No compression.
    None,
    /// Oodle compression.
    Oodle,
    /// Zlib compression.
    Zlib,
    /// Gzip compression.
    Gzip,
    /// LZ4 compression.
    LZ4,
}

impl FArchiveCompression {
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let compression_type = cursor.read_enum::<ArchiveCompressionType>()?;
        match compression_type {
            // ArchiveCompressionType::Named => Ok(FArchiveCompression::Named(cursor.read_string()?)),
            // ArchiveCompressionType::None => Ok(FArchiveCompression::None),
            // ArchiveCompressionType::Oodle => Ok(FArchiveCompression::Oodle),
            ArchiveCompressionType::Zlib => Ok(FArchiveCompression::Zlib),
            // ArchiveCompressionType::Gzip => Ok(FArchiveCompression::Gzip),
            // ArchiveCompressionType::LZ4 => Ok(FArchiveCompression::LZ4),
            _ => Err(DeserializeError::InvalidHeader(
                format!("Unsupported compression type {compression_type:?}").into_boxed_str(),
            ))?,
        }
    }

    pub(crate) fn write<W: Write>(&self, cursor: &mut W) -> Result<usize, Error> {
        match self {
            // FArchiveCompression::Named(compressor) => {
            //     cursor.write_enum(ArchiveCompressionType::Named)?;
            //     let size = cursor.write_string(compressor)?;
            //     Ok(size + 1)
            // }
            FArchiveCompression::None => {
                cursor.write_enum(ArchiveCompressionType::None)?;
                Ok(1)
            }
            FArchiveCompression::Oodle => {
                cursor.write_enum(ArchiveCompressionType::Oodle)?;
                Ok(1)
            }
            FArchiveCompression::Zlib => {
                cursor.write_enum(ArchiveCompressionType::Zlib)?;
                Ok(1)
            }
            FArchiveCompression::Gzip => {
                cursor.write_enum(ArchiveCompressionType::Gzip)?;
                Ok(1)
            }
            FArchiveCompression::LZ4 => {
                cursor.write_enum(ArchiveCompressionType::LZ4)?;
                Ok(1)
            }
        }
    }
}
