use std::io::{Cursor, Read, Seek};

use binrw::{BinRead, binwrite};

use crate::{
    options::ParsingOptions,
    properties::*,
    types::{PropertyTagFlags, PropertyType, TypeTree},
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
    MulticastInlineDelegate(MulticastInlineDelegateProperty),
    MulticastSparseDelegate(MulticastSparseDelegateProperty),
    Name(NameProperty),
    Object(ObjectProperty),
    SoftObject(SoftObjectProperty),
    Str(StrProperty),
    Struct(StructProperty),
    Text(TextProperty),
    UInt16(UInt16Property),
    UInt32(UInt32Property),
    UInt64(UInt64Property),
    Unknown(Vec<u8>),
}

impl Property {
    fn property_type_name(&self) -> &str {
        match &self {
            Property::Array(..) => NAME_ARRAY_PROPERTY,
            Property::Bool(..) => NAME_BOOL_PROPERTY,
            Property::Delegate(..) => NAME_DELEGATE_PROPERTY,
            Property::Double(..) => NAME_DOUBLE_PROPERTY,
            Property::Enum(..) => NAME_ENUM_PROPERTY,
            Property::Float(..) => NAME_FLOAT_PROPERTY,
            Property::Int(..) => NAME_INT_PROPERTY,
            Property::Int16(..) => NAME_INT16_PROPERTY,
            Property::Int64(..) => NAME_INT64_PROPERTY,
            Property::Int8(..) => NAME_INT8_PROPERTY,
            Property::MulticastInlineDelegate(..) => NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
            Property::MulticastSparseDelegate(..) => NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
            Property::Name(..) => NAME_NAME_PROPERTY,
            Property::Object(..) => NAME_OBJECT_PROPERTY,
            Property::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
            Property::Str(..) => NAME_STR_PROPERTY,
            Property::Struct(..) => NAME_STRUCT_PROPERTY,
            Property::Text(..) => NAME_TEXT_PROPERTY,
            Property::UInt16(..) => NAME_UINT16_PROPERTY,
            Property::UInt32(..) => NAME_UINT32_PROPERTY,
            Property::UInt64(..) => NAME_UINT64_PROPERTY,
            _ => todo!(),
        }
    }

    fn inner_type_name(&self) -> Option<&str> {
        match &self {
            Property::Array(array_property) => Some(match array_property {
                ArrayProperty::Bool(..) => NAME_BOOL_PROPERTY,
                ArrayProperty::Byte(..) => NAME_BYTE_PROPERTY,
                ArrayProperty::Enum(..) => NAME_ENUM_PROPERTY,
                ArrayProperty::Float(..) => NAME_FLOAT_PROPERTY,
                ArrayProperty::Int(..) => NAME_INT_PROPERTY,
                ArrayProperty::Name(..) => NAME_NAME_PROPERTY,
                ArrayProperty::Object(..) => NAME_OBJECT_PROPERTY,
                ArrayProperty::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
                ArrayProperty::Str(..) => NAME_STR_PROPERTY,
                ArrayProperty::Struct { .. } => NAME_STRUCT_PROPERTY,
                ArrayProperty::Text(..) => NAME_TEXT_PROPERTY,
                _ => todo!(),
            }),
            // Property::Map(map_property) = match map_property { ... },
            // Property::Option(option_property) = match option_property { ... },
            // Property::Set(set_property) => match set_property { ... },
            Property::Bool(..)
            | Property::Delegate(..)
            | Property::Double(..)
            | Property::Enum(..)
            | Property::Float(..)
            | Property::Int(..)
            | Property::Int16(..)
            | Property::Int64(..)
            | Property::Int8(..)
            | Property::MulticastInlineDelegate(..)
            | Property::MulticastSparseDelegate(..)
            | Property::Name(..)
            | Property::Object(..)
            | Property::SoftObject(..)
            | Property::Str(..)
            | Property::Struct(..)
            | Property::Text(..)
            | Property::UInt16(..)
            | Property::UInt32(..)
            | Property::UInt64(..) => None,
            _ => todo!(),
        }
    }

    pub fn tag(&self, options: ParsingOptions, size: u32) -> PropertyType {
        let property_type_name = self.property_type_name();
        let property_type = FString(Some(property_type_name.to_string()));
        let array_index = 0;
        let guid = 0;
        let inner_type = FString(self.inner_type_name().map(|s| s.to_string()));
        match options.property_tag_complete_type_name {
            false => PropertyType::Incomplete {
                property_type,
                size,
                array_index,
                extra: match &self {
                    Property::Array(..) => CollectionProperties::Array { inner_type },
                    // Property::Map(map_property) = match map_property { ... },
                    // Property::Option(option_property) = match option_property { ... },
                    // Property::Set(set_property) => match set_property { ... },
                    Property::Bool(..)
                    | Property::Delegate(..)
                    | Property::Double(..)
                    | Property::Enum(..)
                    | Property::Float(..)
                    | Property::Int(..)
                    | Property::Int16(..)
                    | Property::Int64(..)
                    | Property::Int8(..)
                    | Property::MulticastInlineDelegate(..)
                    | Property::MulticastSparseDelegate(..)
                    | Property::Name(..)
                    | Property::Object(..)
                    | Property::SoftObject(..)
                    | Property::Str(..)
                    | Property::Struct(..)
                    | Property::Text(..)
                    | Property::UInt16(..)
                    | Property::UInt32(..)
                    | Property::UInt64(..) => CollectionProperties::None,
                    _ => todo!(),
                },
            },
            true => {
                let mut flags = PropertyTagFlags::new();
                flags.set_has_property_guid(guid != 0);
                flags.set_has_array_index(array_index != 0);
                PropertyType::Complete {
                    property_type: TypeTree {
                        name: property_type,
                        children: match &self {
                            Property::Array(array_property) => StaticArray(Vec::from([TypeTree {
                                name: inner_type,

                                children: StaticArray(Vec::new()),
                            }])),
                            Property::Bool(bool_property) => todo!(),
                            Property::Delegate(delegate_property) => todo!(),
                            Property::Double(double_property) => todo!(),
                            Property::Enum(enum_property) => todo!(),
                            Property::Float(float_property) => todo!(),
                            Property::Int(int_property) => todo!(),
                            Property::Int16(int16_property) => todo!(),
                            Property::Int64(int64_property) => todo!(),
                            Property::Int8(int8_property) => todo!(),
                            Property::MulticastInlineDelegate(
                                multicast_inline_delegate_property,
                            ) => {
                                todo!()
                            }
                            Property::MulticastSparseDelegate(
                                multicast_sparse_delegate_property,
                            ) => {
                                todo!()
                            }
                            Property::Name(name_property) => todo!(),
                            Property::Object(object_property) => todo!(),
                            Property::SoftObject(soft_object_property) => todo!(),
                            Property::Str(str_property) => todo!(),
                            Property::Struct(struct_property) => todo!(),
                            Property::Text(text_property) => todo!(),
                            Property::UInt16(uint16_property) => todo!(),
                            Property::UInt32(uint32_property) => todo!(),
                            Property::UInt64(uint64_property) => todo!(),
                            _ => todo!(),
                        },
                    },
                    size,
                    flags,
                    array_index,
                    guid,
                }
            }
        }
    }
}

impl BinRead for Property {
    type Args<'a> = (ParsingOptions, &'a PropertyType);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (options, t): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let size = t.size();
        let start = reader.stream_position()?;

        // TODO: Improve handling of None results
        let property_type = t.property_type().unwrap_or_default();
        let inner_type = t.inner_type().unwrap_or_default();
        let type_name = t.type_name().unwrap_or_default();

        #[rustfmt::skip] // Disable wrapping on this block
        let result = match property_type {
            NAME_ARRAY_PROPERTY  => Property::Array ( ArrayProperty::read_options(reader, endian, (options, t, inner_type))?),
            NAME_BOOL_PROPERTY   => Property::Bool  (  BoolProperty::read_options(reader, endian, (t))?),
            // NAME_BYTE_PROPERTY   => Property::Byte  (  ByteProperty::read_options(reader, endian, ())?),
            NAME_DELEGATE_PROPERTY => Property::Delegate(DelegateProperty::read_options(reader, endian, ())?),
            NAME_DOUBLE_PROPERTY => Property::Double(DoubleProperty::read_options(reader, endian, ())?),
            NAME_ENUM_PROPERTY   => Property::Enum  (  EnumProperty::read_options(reader, endian, ())?),
            NAME_FLOAT_PROPERTY  => Property::Float ( FloatProperty::read_options(reader, endian, ())?),
            NAME_INT16_PROPERTY  => Property::Int16 ( Int16Property::read_options(reader, endian, ())?),
            NAME_INT64_PROPERTY  => Property::Int64 ( Int64Property::read_options(reader, endian, ())?),
            NAME_INT8_PROPERTY   => Property::Int8  (  Int8Property::read_options(reader, endian, ())?),
            NAME_INT_PROPERTY    => Property::Int   (   IntProperty::read_options(reader, endian, ())?),
            // NAME_MAP_PROPERTY    => Property::Map   (   MapProperty::read_options(reader, endian, (options, t))?),
            NAME_MULTICAST_INLINE_DELGATE_PROPERTY => Property::MulticastInlineDelegate(MulticastInlineDelegateProperty::read_options(reader, endian, ())?),
            NAME_MULTICAST_SPARSE_DELGATE_PROPERTY => Property::MulticastSparseDelegate(MulticastSparseDelegateProperty::read_options(reader, endian, ())?),
            NAME_NAME_PROPERTY   => Property::Name  (  NameProperty::read_options(reader, endian, ())?),
            NAME_OBJECT_PROPERTY => Property::Object(ObjectProperty::read_options(reader, endian, ())?),
            // NAME_OPTION_PROPERTY => Property::Option(OptionProperty::read_options(reader, endian, ())?),
            // NAME_SET_PROPERTY    => Property::Set   (   SetProperty::read_options(reader, endian, (options, t))?),
            NAME_SOFT_OBJECT_PROPERTY => Property::SoftObject(SoftObjectProperty::read_options(reader, endian, (options,))?),
            NAME_STRUCT_PROPERTY => Property::Struct(StructProperty::read_options(reader, endian, (options, type_name))?),
            NAME_STR_PROPERTY    => Property::Str   (   StrProperty::read_options(reader, endian, ())?),
            NAME_TEXT_PROPERTY   => Property::Text  (  TextProperty::read_options(reader, endian, (options,))?),
            NAME_UINT16_PROPERTY => Property::UInt16(UInt16Property::read_options(reader, endian, ())?),
            NAME_UINT32_PROPERTY => Property::UInt32(UInt32Property::read_options(reader, endian, ())?),
            NAME_UINT64_PROPERTY => Property::UInt64(UInt64Property::read_options(reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {:?}", property_type);
                let mut buf = vec![0u8; size as usize];
                reader.read_exact(&mut buf)?;
                return Ok(Property::Unknown(buf));
            }
        };

        // Check bytes read compared to size
        let start = start as i64;
        let size = size as i64;
        let pos = reader.stream_position()? as i64;
        let bytes_read = pos - start;
        if bytes_read != size {
            let remaining = size - bytes_read;
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
