use binrw::{BinRead, binwrite};

use crate::{
    options::ParsingOptions,
    properties::*,
    types::{EPropertyTagFlags, FGuid, FPropertyTypeName, PropertyType},
};

#[binwrite]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum Property {
    Array(#[bw(args(options))] ArrayProperty),
    Bool(BoolProperty),
    Delegate(DelegateProperty),
    Double(DoubleProperty),
    Enum(EnumProperty),
    Float(FloatProperty),
    Int(IntProperty),
    Int16(Int16Property),
    Int64(Int64Property),
    Int8(Int8Property),
    Map(#[bw(args(options))] MapProperty),
    MulticastInlineDelegate(MulticastInlineDelegateProperty),
    MulticastSparseDelegate(MulticastSparseDelegateProperty),
    Name(NameProperty),
    Object(ObjectProperty),
    Set(#[bw(args(options))] SetProperty),
    SoftObject(SoftObjectProperty),
    Str(StrProperty),
    Struct(#[bw(args(options))] StructProperty),
    Text(#[bw(args(options))] TextProperty),
    UInt16(UInt16Property),
    UInt32(UInt32Property),
    UInt64(UInt64Property),
    Unknown(PropertyType, Vec<u8>),
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
            Property::Map(..) => NAME_MAP_PROPERTY,
            Property::MulticastInlineDelegate(..) => NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
            Property::MulticastSparseDelegate(..) => NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
            Property::Name(..) => NAME_NAME_PROPERTY,
            Property::Object(..) => NAME_OBJECT_PROPERTY,
            Property::Set(..) => NAME_SET_PROPERTY,
            Property::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
            Property::Str(..) => NAME_STR_PROPERTY,
            Property::Struct(..) => NAME_STRUCT_PROPERTY,
            Property::Text(..) => NAME_TEXT_PROPERTY,
            Property::UInt16(..) => NAME_UINT16_PROPERTY,
            Property::UInt32(..) => NAME_UINT32_PROPERTY,
            Property::UInt64(..) => NAME_UINT64_PROPERTY,
            Property::Unknown(t, ..) => match t {
                PropertyType::Incomplete {
                    property_type: FString(Some(name)),
                    ..
                } => name,
                PropertyType::Complete {
                    property_type:
                        FPropertyTypeName {
                            name: FString(Some(name)),
                            ..
                        },
                    ..
                } => name,
                _ => todo!("{t:?}"),
            },
        }
    }

    fn container_inner_type_name(&self) -> Option<&str> {
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
                ArrayProperty::TaggedStruct { .. } => NAME_STRUCT_PROPERTY,
                ArrayProperty::Text(..) => NAME_TEXT_PROPERTY,
                _ => todo!("{array_property:?}"),
            }),
            // Property::Map(map_property) => todo!("{map_property:?}"),
            // Property::Option(option_property) => todo!("{option_property:?}"),
            // Property::Set(set_property) => todo!("{set_property:?}"),
            _ => None,
        }
    }

    pub fn property_type(&self, options: ParsingOptions, size: u32) -> PropertyType {
        let property_type_name = self.property_type_name();
        let property_type = FString::from(property_type_name);
        let array_index = 0;
        let guid = FGuid::invalid();
        let inner_type = FString::from(self.container_inner_type_name());
        match options.property_tag_complete_type_name {
            false => PropertyType::Incomplete {
                property_type,
                size,
                array_index,
                extra: match &self {
                    Property::Array(..) => CollectionProperties::Array { inner_type },
                    Property::Bool(BoolProperty(value)) => CollectionProperties::Bool {
                        value: *value as u8,
                    },
                    // Property::Byte(..) => Collection::Byte {},
                    // Property::Enum(..) => Collection::Enum {},
                    Property::Map(map_property) => {
                        let (key_type, value_type) = match map_property {
                            MapProperty::Known {
                                key_type,
                                value_type,
                                ..
                            }
                            | MapProperty::Unknown {
                                key_type,
                                value_type,
                                ..
                            } => (key_type, value_type),
                        };

                        let PropertyType::Incomplete {
                            property_type: key_type,
                            ..
                        } = key_type
                        else {
                            todo!()
                        };
                        let PropertyType::Incomplete {
                            property_type: value_type,
                            ..
                        } = value_type
                        else {
                            todo!()
                        };
                        CollectionProperties::Map {
                            inner_type: key_type.clone(),
                            value_type: value_type.clone(),
                        }
                    }
                    // Property::Option(option_property) => CollectionProperties::Option { inner_type },
                    Property::Set(..) => CollectionProperties::Set { inner_type },
                    Property::Delegate(..)
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
                    | Property::UInt64(..)
                    | Property::Unknown(..) => CollectionProperties::None,
                    // _ => todo!("{self:?}"),
                },
            },
            true => {
                let mut flags = EPropertyTagFlags::new();
                flags.set_has_property_guid(guid.is_valid());
                flags.set_has_array_index(array_index != 0);
                PropertyType::Complete {
                    property_type: FPropertyTypeName {
                        name: property_type,
                        children: match &self {
                            Property::Array(_) => TArray(Vec::from([FPropertyTypeName {
                                name: inner_type,
                                children: TArray(Vec::new()),
                            }])),
                            Property::Bool(BoolProperty(value)) => {
                                flags.set_bool_true(*value);
                                TArray(Vec::new())
                            }
                            // Property::Delegate(_) => todo!(),
                            Property::Double(_) |
                            // Property::Enum(_) => todo!(),
                            // Property::Float(_) => todo!(),
                            Property::Int(_) |
                            // Property::Int16(_) => todo!(),
                            // Property::Int64(_) => todo!(),
                            // Property::Int8(_) => todo!(),
                            // Property::MulticastInlineDelegate(_) => todo!(),
                            // Property::MulticastSparseDelegate(_) => todo!(),
                            Property::Name(_) => TArray(Vec::from([])),
                            // Property::Object(_) => todo!(),
                            // Property::SoftObject(_) => todo!(),
                            // Property::Str(_) => todo!(),
                            Property::Struct(_) => todo!(),
                            // Property::Text(_) => todo!(),
                            // Property::UInt16(_) => todo!(),
                            // Property::UInt32(_) => todo!(),
                            // Property::UInt64(_) => todo!(),
                            _ => todo!("{self:?}"),
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

        let property_type = t.property_type().expect("property_type");

        #[rustfmt::skip] // Disable wrapping on this block
        let result = match property_type {
            NAME_ARRAY_PROPERTY  => {
                let inner_type = t.array_inner_type().expect("inner_type");
                let array_property = ArrayProperty::read_options(reader, endian, (options, t, inner_type))?;
                Property::Array(array_property)
            },
            NAME_BOOL_PROPERTY   => Property::Bool  (  BoolProperty::read_options(reader, endian, (t,))?),
            // NAME_BYTE_PROPERTY   => Property::Byte  (  ByteProperty::read_options(reader, endian, ())?),
            NAME_DELEGATE_PROPERTY => Property::Delegate(DelegateProperty::read_options(reader, endian, ())?),
            NAME_DOUBLE_PROPERTY => Property::Double(DoubleProperty::read_options(reader, endian, ())?),
            NAME_ENUM_PROPERTY   => Property::Enum  (  EnumProperty::read_options(reader, endian, ())?),
            NAME_FLOAT_PROPERTY  => Property::Float ( FloatProperty::read_options(reader, endian, ())?),
            NAME_INT16_PROPERTY  => Property::Int16 ( Int16Property::read_options(reader, endian, ())?),
            NAME_INT64_PROPERTY  => Property::Int64 ( Int64Property::read_options(reader, endian, ())?),
            NAME_INT8_PROPERTY   => Property::Int8  (  Int8Property::read_options(reader, endian, ())?),
            NAME_INT_PROPERTY    => Property::Int   (   IntProperty::read_options(reader, endian, ())?),
            NAME_MAP_PROPERTY    => Property::Map   (   MapProperty::read_options(reader, endian, (options, t))?),
            NAME_MULTICAST_INLINE_DELGATE_PROPERTY => Property::MulticastInlineDelegate(MulticastInlineDelegateProperty::read_options(reader, endian, ())?),
            NAME_MULTICAST_SPARSE_DELGATE_PROPERTY => Property::MulticastSparseDelegate(MulticastSparseDelegateProperty::read_options(reader, endian, ())?),
            NAME_NAME_PROPERTY   => Property::Name  (  NameProperty::read_options(reader, endian, ())?),
            NAME_OBJECT_PROPERTY => Property::Object(ObjectProperty::read_options(reader, endian, ())?),
            // NAME_OPTION_PROPERTY => Property::Option(OptionProperty::read_options(reader, endian, ())?),
            NAME_SET_PROPERTY    => Property::Set   (   SetProperty::read_options(reader, endian, (options, t))?),
            NAME_SOFT_OBJECT_PROPERTY => Property::SoftObject(SoftObjectProperty::read_options(reader, endian, (options,))?),
            NAME_STRUCT_PROPERTY => {
                let type_name = t.struct_type_name().unwrap_or_default();
                let struct_property = StructProperty::read_options(reader, endian, (options, t, type_name))?;
                Property::Struct(struct_property)
            },
            NAME_STR_PROPERTY    => Property::Str   (   StrProperty::read_options(reader, endian, ())?),
            NAME_TEXT_PROPERTY   => Property::Text  (  TextProperty::read_options(reader, endian, (options,))?),
            NAME_UINT16_PROPERTY => Property::UInt16(UInt16Property::read_options(reader, endian, ())?),
            NAME_UINT32_PROPERTY => Property::UInt32(UInt32Property::read_options(reader, endian, ())?),
            NAME_UINT64_PROPERTY => Property::UInt64(UInt64Property::read_options(reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {property_type}");
                let mut buf = vec![0u8; size as usize];
                reader.read_exact(&mut buf)?;
                let result = Property::Unknown(t.clone(), buf);
                return Ok(result);
            }
        };

        // Check bytes read compared to size
        let start = start as i64;
        let size = size as i64;
        let pos = reader.stream_position()? as i64;
        let bytes_read = pos - start;
        if size != 0 && bytes_read != size {
            let remaining = size - bytes_read;
            reader.seek_relative(-bytes_read)?;
            println!(
                "Warning: Reader position 0x{pos:04X} does not match size 0x{size:04X} for {t:?}: 0x{remaining:04X} remaining",
            );
            let mut buf = vec![0u8; size as usize];
            reader.read_exact(&mut buf)?;
            let result = Property::Unknown(t.clone(), buf);
            return Ok(result);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    // TODO: Write tests
}
