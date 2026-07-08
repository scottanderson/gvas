use binrw::{BinRead, BinWrite};

use crate::error::binrw_custom;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct FString(pub Option<String>);

impl std::fmt::Debug for FString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => f.write_str("FString(None)"),
            Some(s) => write!(f, "FString::from({s:?})"),
        }
    }
}

impl std::fmt::Display for FString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_deref() {
            None => f.write_str("null"),
            Some(s) => std::fmt::Display::fmt(s, f),
        }
    }
}

impl BinRead for FString {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let length = i32::read_options(reader, endian, ())?;
        if !(-0x20000..0x20000).contains(&length) {
            Err(binrw::Error::AssertFail {
                pos,
                message: format!("Invalid FString length 0x{length:x}"),
            })?;
        }
        let value = if length == 0 {
            None
        } else if length > 0 {
            let count = usize::try_from(length - 1).map_err(binrw_custom(pos))?;
            let mut buf = vec![0u8; count];
            reader.read_exact(&mut buf)?;

            let terminator = u8::read_options(reader, endian, ())?;
            if terminator != 0 {
                Err(binrw::Error::AssertFail {
                    pos,
                    message: format!("Invalid terminator value for string length {count}"),
                })?;
            }

            let str = String::from_utf8(buf).map_err(binrw_custom(pos))?;
            Some(str)
        } else {
            let count = usize::try_from(-length - 1).map_err(binrw_custom(pos))?;
            let mut bytes = vec![0u8; count * 2];
            reader.read_exact(&mut bytes)?;

            let terminator = u16::read_options(reader, endian, ())?;
            if terminator != 0 {
                Err(binrw::Error::AssertFail {
                    pos,
                    message: format!("Invalid terminator value for string length {count}"),
                })?;
            }

            let buf: Vec<u16> = bytes
                .as_chunks::<2>()
                .0
                .iter()
                .copied()
                .map(match endian {
                    binrw::Endian::Big => u16::from_be_bytes,
                    binrw::Endian::Little => u16::from_le_bytes,
                })
                .collect();
            let str = String::from_utf16(&buf).map_err(binrw_custom(pos))?;
            Some(str)
        };
        Ok(Self(value))
    }
}

impl BinWrite for FString {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match self.as_deref() {
            None => u32::write_options(&0, writer, endian, ())?,
            Some(str) => {
                let pos = writer.stream_position()?;
                let err_convert = binrw_custom(pos);
                if str.is_ascii() {
                    // ASCII strings do not require encoding
                    let len = i32::try_from(str.len() + 1).map_err(err_convert)?;
                    i32::write_options(&len, writer, endian, ())?;
                    let _ = writer.write(str.as_bytes())?;
                    let _ = writer.write(&[0u8; 1])?;
                } else {
                    // Perform UTF-16 encoding when non-ASCII characters are detected
                    let words: Vec<u16> = str.encode_utf16().collect();
                    let len = -i32::try_from(words.len() + 1).map_err(err_convert)?;
                    i32::write_options(&len, writer, endian, ())?;
                    for word in words {
                        u16::write_options(&word, writer, endian, ())?;
                    }
                    u16::write_options(&0, writer, endian, ())?;
                }
            }
        }
        Ok(())
    }
}

impl std::ops::Deref for FString {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for FString {
    #[inline]
    fn from(value: String) -> Self {
        Self(Some(value))
    }
}

impl From<Option<String>> for FString {
    #[inline]
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}

impl From<&str> for FString {
    #[inline]
    fn from(value: &str) -> Self {
        Self(Some(value.to_owned()))
    }
}

impl From<Option<&str>> for FString {
    #[inline]
    fn from(value: Option<&str>) -> Self {
        Self(value.map(ToOwned::to_owned))
    }
}

impl PartialEq<str> for FString {
    fn eq(&self, other: &str) -> bool {
        self.as_deref() == Some(other)
    }
}

impl PartialEq<&str> for FString {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::error::Result;

    use super::*;

    const BYTES_NULL: &[u8; 4] = b"\x00\x00\x00\x00";
    const BYTES_EMPTY: &[u8; 5] = b"\x01\x00\x00\x00\x00";
    const BYTES_PROPERTY: &[u8; 16] = b"\x0c\x00\x00\x00StrProperty\x00";
    const BYTES_UTF16: &[u8; 8] = b"\xfe\xff\xff\xff\xa7\x00\x00\x00";

    const STR_EMPTY: &str = "";
    const STR_PROPERTY: &str = "StrProperty";
    const STR_UTF16: &str = "§";

    #[test]
    fn read_fstring_null() -> Result<()> {
        let mut cursor = Cursor::new(BYTES_NULL);
        let string = FString::read_le(&mut cursor)?;
        assert_eq!(string, FString(None));
        Ok(())
    }

    #[test]
    fn read_fstring_empty() -> Result<()> {
        let mut cursor = Cursor::new(BYTES_EMPTY);
        let string = FString::read_le(&mut cursor)?;
        assert_eq!(string, FString::from(STR_EMPTY));
        Ok(())
    }

    #[test]
    fn read_fstring_ascii() -> Result<()> {
        let mut cursor = Cursor::new(BYTES_PROPERTY);
        let string = FString::read_le(&mut cursor)?;
        assert_eq!(string, FString::from(STR_PROPERTY));
        Ok(())
    }

    #[test]
    fn read_fstring_utf16() -> Result<()> {
        let mut cursor = Cursor::new(BYTES_UTF16);
        let string = FString::read_le(&mut cursor)?;
        assert_eq!(string, FString::from(STR_UTF16));
        Ok(())
    }

    #[test]
    fn write_fstring_null() -> Result<()> {
        let mut cursor = Cursor::new(vec![]);
        let string = FString(None);
        string.write_le(&mut cursor)?;
        assert_eq!(cursor.into_inner().as_slice(), BYTES_NULL);
        Ok(())
    }

    #[test]
    fn write_fstring_empty() -> Result<()> {
        let mut cursor = Cursor::new(vec![]);
        let string = FString::from(STR_EMPTY);
        string.write_le(&mut cursor)?;
        assert_eq!(cursor.into_inner().as_slice(), BYTES_EMPTY);
        Ok(())
    }

    #[test]
    fn write_fstring_ascii() -> Result<()> {
        let mut cursor = Cursor::new(vec![]);
        let string = FString::from(STR_PROPERTY);
        string.write_le(&mut cursor)?;
        assert_eq!(cursor.into_inner().as_slice(), BYTES_PROPERTY);
        Ok(())
    }

    #[test]
    fn write_fstring_utf16() -> Result<()> {
        let mut cursor = Cursor::new(vec![]);
        let string = FString::from(STR_UTF16);
        string.write_le(&mut cursor)?;
        assert_eq!(cursor.into_inner().as_slice(), BYTES_UTF16);
        Ok(())
    }
}
