use binrw::{BinRead, BinWrite};

#[derive(Clone, Eq, PartialEq)]
pub struct FString(pub Option<String>);

impl std::fmt::Debug for FString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => f.write_str("null"),
            Some(s) => s.fmt(f),
        }
    }
}

impl BinRead for FString {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let length = i32::read_le(reader)?;
        let value = if length == 0 {
            None
        } else if length > 0 {
            let count = length as usize - 1;
            let mut buf = vec![0u8; count];
            reader.read_exact(&mut buf)?;

            let terminator = u8::read_le(reader)?;
            if terminator != 0 {
                Err(binrw::Error::AssertFail {
                    pos,
                    message: format!("Invalid terminator value for string length {length}"),
                })?
            }

            let str = String::from_utf8(buf).map_err(|_| -> binrw::Error {
                binrw::Error::AssertFail {
                    pos,
                    message: "FromUtf8Error".into(),
                }
            })?;
            Some(str)
        } else {
            let count = -length as usize - 1;
            let buf: Vec<u16> = (0..count)
                .map(|_| u16::read_options(reader, _endian, ()))
                .collect::<binrw::BinResult<_>>()?;

            let terminator = u16::read_le(reader)?;
            if terminator != 0 {
                Err(binrw::Error::AssertFail {
                    pos,
                    message: "Invalid terminator value".into(),
                })?
            }

            let str = String::from_utf16(&buf).map_err(|_| -> binrw::Error {
                binrw::Error::AssertFail {
                    pos,
                    message: "FromUtf16Error".into(),
                }
            })?;
            Some(str)
        };
        Ok(FString(value))
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
        match &self.0 {
            None => u32::write_options(&0, writer, endian, ())?,
            Some(str) => {
                if str.is_ascii() {
                    // ASCII strings do not require encoding
                    let len: i32 = (str.len() + 1) as i32;
                    i32::write_options(&len, writer, endian, ())?;
                    let _ = writer.write(str.as_bytes())?;
                    let _ = writer.write(&[0u8; 1])?;
                } else {
                    // Perform UTF-16 encoding when non-ASCII characters are detected
                    let words: Vec<u16> = str.encode_utf16().collect();
                    let len = -((words.len() + 1) as i32);
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

#[cfg(test)]
mod test {
    use super::FString;
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    const BYTES_NULL: &[u8; 4] = b"\x00\x00\x00\x00";
    const BYTES_EMPTY: &[u8; 5] = b"\x01\x00\x00\x00\x00";
    const BYTES_PROPERTY: &[u8; 16] = b"\x0c\x00\x00\x00StrProperty\x00";
    const BYTES_UTF16: &[u8; 8] = b"\xfe\xff\xff\xff\xa7\x00\x00\x00";

    const STR_EMPTY: &str = "";
    const STR_PROPERTY: &str = "StrProperty";
    const STR_UTF16: &str = "§";

    #[test]
    fn read_fstring_null() {
        let mut cursor = Cursor::new(BYTES_NULL);
        let string = FString::read_le(&mut cursor).unwrap();
        assert_eq!(string, FString(None));
    }

    #[test]
    fn read_fstring_empty() {
        let mut cursor = Cursor::new(BYTES_EMPTY);
        let string = FString::read_le(&mut cursor).expect("FString::read_le");
        assert_eq!(string, FString(Some(String::from(STR_EMPTY))));
    }

    #[test]
    fn read_fstring_ascii() {
        let mut cursor = Cursor::new(BYTES_PROPERTY);
        let string = FString::read_le(&mut cursor).expect("FString::read_le");
        assert_eq!(string, FString(Some(String::from(STR_PROPERTY))));
    }

    #[test]
    fn read_fstring_utf16() {
        let mut cursor = Cursor::new(BYTES_UTF16);
        let string = FString::read_le(&mut cursor).expect("FString::read_le");
        assert_eq!(string, FString(Some(String::from(STR_UTF16))));
    }

    #[test]
    fn write_fstring_null() {
        let mut cursor = Cursor::new(vec![]);
        let string = FString(None);
        string.write_le(&mut cursor).expect("FString::write_le");
        assert_eq!(cursor.into_inner().as_slice(), BYTES_NULL)
    }

    #[test]
    fn write_fstring_empty() {
        let mut cursor = Cursor::new(vec![]);
        let string = FString(Some(String::from("")));
        string.write_le(&mut cursor).expect("FString::write_le");
        assert_eq!(cursor.into_inner().as_slice(), BYTES_EMPTY)
    }

    #[test]
    fn write_fstring_ascii() {
        let mut cursor = Cursor::new(vec![]);
        let string = FString(Some(String::from("StrProperty")));
        string.write_le(&mut cursor).expect("FString::write_le");
        assert_eq!(cursor.into_inner().as_slice(), BYTES_PROPERTY)
    }

    #[test]
    fn write_fstring_utf16() {
        let mut cursor = Cursor::new(vec![]);
        let string = FString(Some(String::from("§")));
        string.write_le(&mut cursor).expect("FString::write_le");
        assert_eq!(cursor.into_inner().as_slice(), BYTES_UTF16)
    }
}
