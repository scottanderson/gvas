use std::io::{Cursor, Seek};

use binrw::{BinRead, binwrite};

use crate::{
    options::ParsingOptions,
    properties::*,
    types::{PropertyTagFlags, PropertyType},
};

#[binwrite]
#[derive(Debug)]
pub enum Property {
    Array(ArrayProperty),
    Bool(BoolProperty),
    Delegate(DelegateProperty),
    Double(DoubleProperty),
    Enum(EnumProperty),
    Float(FloatProperty),
    Int(IntProperty),
    Int16(Int16Property),
    Int64(Int64Property),
    Int8(Int8Property),
    Name(NameProperty),
    Object(ObjectProperty),
    Str(StrProperty),
    Struct(StructProperty),
    UInt16(UInt16Property),
    UInt32(UInt32Property),
    UInt64(UInt64Property),
    Unknown(Vec<u8>),
}

impl BinRead for Property {
    type Args<'a> = (ParsingOptions, &'a PropertyType);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (options, t): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let size = t.size();
        let mut buf = vec![0u8; size as usize];
        reader.read_exact(&mut buf)?;

        let mut reader = Cursor::new(&buf);

        // TODO: Improve handling of None results
        let property_type = t.property_type().unwrap_or_default();
        let inner_type = t.inner_type().unwrap_or_default();
        let type_name = t.type_name().unwrap_or_default();

        #[rustfmt::skip] // Disable wrapping on this block
        let result = match property_type {
            NAME_ARRAY_PROPERTY  => Property::Array ( ArrayProperty::read_options(&mut reader, endian, (options, t, inner_type))?),
            NAME_BOOL_PROPERTY   => Property::Bool  (  BoolProperty::read_options(&mut reader, endian, (t))?),
            // NAME_BYTE_PROPERTY   => Property::Byte  (  ByteProperty::read_options(&mut reader, endian, ())?),
            NAME_DELEGATE_PROPERTY => Property::Delegate(DelegateProperty::read_options(&mut reader, endian, ())?),
            NAME_DOUBLE_PROPERTY => Property::Double(DoubleProperty::read_options(&mut reader, endian, ())?),
            NAME_ENUM_PROPERTY   => Property::Enum  (  EnumProperty::read_options(&mut reader, endian, ())?),
            NAME_FLOAT_PROPERTY  => Property::Float ( FloatProperty::read_options(&mut reader, endian, ())?),
            NAME_INT16_PROPERTY  => Property::Int16 ( Int16Property::read_options(&mut reader, endian, ())?),
            NAME_INT64_PROPERTY  => Property::Int64 ( Int64Property::read_options(&mut reader, endian, ())?),
            NAME_INT8_PROPERTY   => Property::Int8  (  Int8Property::read_options(&mut reader, endian, ())?),
            NAME_INT_PROPERTY    => Property::Int   (   IntProperty::read_options(&mut reader, endian, ())?),
            // NAME_MAP_PROPERTY    => Property::Map   (   MapProperty::read_options(&mut reader, endian, (options, t))?),
            // NAME_MULTICAST_INLINE_DELGATE_PROPERTY => Property::MulticastInlineDelegate(MulticastInlineDelegateProperty::read_options(&mut reader, endian, ())?),
            // NAME_MULTICAST_SPARSE_DELGATE_PROPERTY => Property::MulticastSparseDelegate(MulticastSparseDelegateProperty::read_options(&mut reader, endian, ())?),
            NAME_NAME_PROPERTY   => Property::Name  (  NameProperty::read_options(&mut reader, endian, ())?),
            NAME_OBJECT_PROPERTY => Property::Object(ObjectProperty::read_options(&mut reader, endian, ())?),
            // NAME_OPTION_PROPERTY => Property::Option(OptionProperty::read_options(&mut reader, endian, ())?),
            // NAME_SET_PROPERTY    => Property::Set   (   SetProperty::read_options(&mut reader, endian, (options, t))?),
            // NAME_SOFT_OBJECT_PROPERTY => Property::SoftObject(SoftObjectProperty::read_options(&mut reader, endian, ())?),
            NAME_STRUCT_PROPERTY => Property::Struct(StructProperty::read_options(&mut reader, endian, (options, type_name))?),
            NAME_STR_PROPERTY    => Property::Str   (   StrProperty::read_options(&mut reader, endian, ())?),
            // NAME_TEXT_PROPERTY   => Property::Text  (  TextProperty::read_options(&mut reader, endian, (options))?),
            NAME_UINT16_PROPERTY => Property::UInt16(UInt16Property::read_options(&mut reader, endian, ())?),
            NAME_UINT32_PROPERTY => Property::UInt32(UInt32Property::read_options(&mut reader, endian, ())?),
            NAME_UINT64_PROPERTY => Property::UInt64(UInt64Property::read_options(&mut reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {:?}", property_type);
                let mut buf = vec![0u8; size as usize];
                std::io::Read::read_exact(&mut reader, &mut buf)?;

                let mut reader = Cursor::new(&buf);
                return Ok(Property::Unknown(buf));
            }
        };

        // Check bytes read compared to size
        let pos = reader.stream_position()?;
        if pos != size as u64 {
            let remaining = (size as i64) - (pos as i64);
            reader.seek_relative(remaining)?;
            println!(
                "Warning: Reader position does not match size: 0x{pos:04X} 0x{size:04X} ({remaining}) {property_type:?} {inner_type:?} {type_name:?}",
            );
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    // TODO: Write tests
}
