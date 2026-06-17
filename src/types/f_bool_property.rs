use binrw::{BinRead, BinWrite};

use crate::types::{CollectionProperties, PropertyType};

#[derive(Debug)]
pub struct FBoolProperty(pub bool);

impl BinRead for FBoolProperty {
    type Args<'a> = (&'a PropertyType,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        _reader: &mut R,
        _endian: binrw::Endian,
        (t,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        match t {
            PropertyType::Incomplete {
                extra: CollectionProperties::Bool { value },
                ..
            } => Ok(Self(*value != 0)),
            PropertyType::Complete { flags, .. } => Ok(Self(flags.bool_true())),
            _ => Err(binrw::Error::AssertFail {
                pos: 0,
                message: "BoolProperty type not found".to_string(),
            }),
        }
    }
}

impl BinWrite for FBoolProperty {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        u8::write_options(&if self.0 { 1 } else { 0 }, writer, endian, ())
    }
}
