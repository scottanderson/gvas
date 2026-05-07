use std::io::Cursor;

use binrw::{BinRead, binwrite};

use crate::{options::ParsingOptions, properties::StrProperty, types::PropertyType};

#[binwrite]
#[derive(Debug)]
pub enum Property {
    Str(StrProperty),
    Unknown(Vec<u8>),
}

impl BinRead for Property {
    type Args<'a> = (ParsingOptions, &'a PropertyType);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (_options, t): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let size = match t {
            PropertyType::Incomplete { size, .. } => *size,
            PropertyType::Complete { size, .. } => *size,
        };
        let mut buf = vec![0u8; size as usize];
        reader.read_exact(&mut buf)?;

        let mut reader = Cursor::new(&buf);
        // read_typed_property(options, t, buf);

        let prop_type = match t {
            PropertyType::Incomplete { prop_type, .. } => prop_type,
            PropertyType::Complete { prop_type, .. } => &prop_type.name,
        };

        let result = match &prop_type.0 {
            Some(s) if s == "StrProprty" => {
                let value = StrProperty::read_options(&mut reader, endian, ())?;
                Property::Str(value)
            }
            _ => Property::Unknown(buf),
        };

        // TODO: Check bytes read compared to size

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    // TODO: Write tests
}
