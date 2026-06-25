use binrw::{BinRead, binwrite};

use crate::{
    format::SerializationFormat,
    types::{
        CollectionProperties, EPropertyTagFlags, FArrayProperty, FBoolProperty, FByteProperty,
        FDelegateProperty, FDoubleProperty, FEnumProperty, FFloatProperty, FGuid, FInt8Property,
        FInt16Property, FInt64Property, FIntProperty, FMapProperty,
        FMulticastInlineDelegateProperty, FMulticastSparseDelegateProperty, FNameProperty,
        FObjectProperty, FPropertyTypeName, FSetProperty, FSoftObjectProperty, FStrProperty,
        FString, FStructProperty, FTextProperty, FUInt16Property, FUInt32Property, FUInt64Property,
        NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_DELEGATE_PROPERTY,
        NAME_DOUBLE_PROPERTY, NAME_ENUM_PROPERTY, NAME_FLOAT_PROPERTY, NAME_INT_PROPERTY,
        NAME_INT8_PROPERTY, NAME_INT16_PROPERTY, NAME_INT64_PROPERTY, NAME_MAP_PROPERTY,
        NAME_MULTICAST_INLINE_DELGATE_PROPERTY, NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
        NAME_NAME_PROPERTY, NAME_NONE, NAME_OBJECT_PROPERTY, NAME_SET_PROPERTY,
        NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY, NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY,
        NAME_UINT16_PROPERTY, NAME_UINT32_PROPERTY, NAME_UINT64_PROPERTY, PropertyType, TArray,
    },
};

#[binwrite]
#[bw(import(format: SerializationFormat))]
#[derive(Debug)]
pub enum FProperty {
    Array(#[bw(args(format))] FArrayProperty),
    Bool(#[bw(ignore)] FBoolProperty),
    Byte(FByteProperty),
    Delegate(FDelegateProperty),
    Double(FDoubleProperty),
    Enum(FEnumProperty),
    Float(FFloatProperty),
    Int(FIntProperty),
    Int16(FInt16Property),
    Int64(FInt64Property),
    Int8(FInt8Property),
    Map(#[bw(args(format))] FMapProperty),
    MulticastInlineDelegate(FMulticastInlineDelegateProperty),
    MulticastSparseDelegate(FMulticastSparseDelegateProperty),
    Name(FNameProperty),
    Object(FObjectProperty),
    Set(#[bw(args(format))] FSetProperty),
    SoftObject(FSoftObjectProperty),
    Str(FStrProperty),
    Struct(#[bw(args(format))] FStructProperty),
    Text(#[bw(args(format))] FTextProperty),
    UInt16(FUInt16Property),
    UInt32(FUInt32Property),
    UInt64(FUInt64Property),
    Unknown(#[bw(ignore)] PropertyType, Vec<u8>),
}

impl FProperty {
    fn property_type_name(&self) -> &str {
        match &self {
            Self::Array(..) => NAME_ARRAY_PROPERTY,
            Self::Bool(..) => NAME_BOOL_PROPERTY,
            Self::Byte(..) => NAME_BYTE_PROPERTY,
            Self::Delegate(..) => NAME_DELEGATE_PROPERTY,
            Self::Double(..) => NAME_DOUBLE_PROPERTY,
            Self::Enum(..) => NAME_ENUM_PROPERTY,
            Self::Float(..) => NAME_FLOAT_PROPERTY,
            Self::Int(..) => NAME_INT_PROPERTY,
            Self::Int16(..) => NAME_INT16_PROPERTY,
            Self::Int64(..) => NAME_INT64_PROPERTY,
            Self::Int8(..) => NAME_INT8_PROPERTY,
            Self::Map(..) => NAME_MAP_PROPERTY,
            Self::MulticastInlineDelegate(..) => NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
            Self::MulticastSparseDelegate(..) => NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
            Self::Name(..) => NAME_NAME_PROPERTY,
            Self::Object(..) => NAME_OBJECT_PROPERTY,
            Self::Set(..) => NAME_SET_PROPERTY,
            Self::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
            Self::Str(..) => NAME_STR_PROPERTY,
            Self::Struct(..) => NAME_STRUCT_PROPERTY,
            Self::Text(..) => NAME_TEXT_PROPERTY,
            Self::UInt16(..) => NAME_UINT16_PROPERTY,
            Self::UInt32(..) => NAME_UINT32_PROPERTY,
            Self::UInt64(..) => NAME_UINT64_PROPERTY,
            Self::Unknown(t, ..) => match t {
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
            Self::Array(array_property) => Some(match array_property {
                FArrayProperty::Bool(..) => NAME_BOOL_PROPERTY,
                FArrayProperty::Byte(..) => NAME_BYTE_PROPERTY,
                FArrayProperty::Enum(..) => NAME_ENUM_PROPERTY,
                FArrayProperty::Float(..) => NAME_FLOAT_PROPERTY,
                FArrayProperty::Int(..) => NAME_INT_PROPERTY,
                FArrayProperty::Name(..) => NAME_NAME_PROPERTY,
                FArrayProperty::Object(..) => NAME_OBJECT_PROPERTY,
                FArrayProperty::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
                FArrayProperty::Str(..) => NAME_STR_PROPERTY,
                FArrayProperty::Struct { .. } => NAME_STRUCT_PROPERTY,
                FArrayProperty::TaggedStruct { .. } => NAME_STRUCT_PROPERTY,
                FArrayProperty::Text(..) => NAME_TEXT_PROPERTY,
                _ => todo!("{array_property:?}"),
            }),
            // Property::Map(p) => todo!("{p:?}"),
            // Property::Optional(p) => todo!("{p:?}"),
            // Property::Set(p) => todo!("{p:?}"),
            _ => None,
        }
    }

    pub fn property_type(
        &self,
        format: SerializationFormat,
        size: u32,
        array_index: u32,
        guid: FGuid,
        native: bool,
        extensions: bool,
    ) -> PropertyType {
        if let Self::Unknown(original_tag, _) = self {
            return original_tag.clone();
        }

        let property_type_name = self.property_type_name();
        let property_type = FString::from(property_type_name);
        let inner_type = FString::from(self.container_inner_type_name());
        match format.property_tag_complete_type_name {
            false => PropertyType::Incomplete {
                property_type,
                size,
                array_index,
                extra: match &self {
                    Self::Array(..) => CollectionProperties::Array { inner_type },
                    Self::Bool(FBoolProperty(value)) => CollectionProperties::Bool {
                        value: *value as u8,
                    },
                    Self::Byte(value) => CollectionProperties::Byte {
                        enum_name: match value {
                            FByteProperty::Enum(name, _) => name.clone(),
                            FByteProperty::Byte(_) => FString::from(NAME_NONE),
                        },
                    },
                    Self::Enum(FEnumProperty(enum_name, _)) => CollectionProperties::Enum {
                        enum_name: enum_name.clone(),
                    },
                    Self::Map(map_property) => {
                        let (key_type, value_type) = match map_property {
                            FMapProperty::Known {
                                key_type,
                                value_type,
                                ..
                            }
                            | FMapProperty::Unknown {
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
                    // Property::Optional(p) => CollectionProperties::Optional { inner_type },
                    Self::Set(..) => CollectionProperties::Set { inner_type },
                    Self::Struct(p) => CollectionProperties::Struct {
                        type_name: p.struct_type(),
                        guid,
                    },
                    Self::Delegate(..)
                    | Self::Double(..)
                    | Self::Float(..)
                    | Self::Int(..)
                    | Self::Int16(..)
                    | Self::Int64(..)
                    | Self::Int8(..)
                    | Self::MulticastInlineDelegate(..)
                    | Self::MulticastSparseDelegate(..)
                    | Self::Name(..)
                    | Self::Object(..)
                    | Self::SoftObject(..)
                    | Self::Str(..)
                    | Self::Text(..)
                    | Self::UInt16(..)
                    | Self::UInt32(..)
                    | Self::UInt64(..) => CollectionProperties::None,
                    Self::Unknown(..) => unimplemented!(),
                },
            },
            true => {
                let mut flags = EPropertyTagFlags::new();
                flags.set_has_array_index(array_index != 0);
                flags.set_has_property_guid(guid.is_valid());
                flags.set_has_binary_or_native_serialize(native);
                flags.set_has_property_extensions(extensions);
                PropertyType::Complete {
                    property_type: FPropertyTypeName {
                        name: property_type,
                        children: match self {
                            Self::Array(array_property) => {
                                match array_property {
                                    FArrayProperty::Struct { struct_type, struct_class, struct_guid, values:_ } => {
                                        TArray::from([
                                            FPropertyTypeName::for_struct(
                                                struct_type.to_owned(),
                                                struct_class.to_owned(),
                                                *struct_guid,
                                            ),
                                        ])
                                    },
                                    FArrayProperty::TaggedStruct {..} => todo!(),
                                    _ => TArray::from([FPropertyTypeName::from(inner_type)]),
                                }
                            },
                            Self::Bool(FBoolProperty(value)) => {
                                flags.set_bool_true(*value);
                                TArray::empty()
                            }
                            // Property::Delegate(_) => todo!(),
                            Self::Double(_) |
                            // Property::Enum(_) => todo!(),
                            // Property::Float(_) => todo!(),
                            Self::Int(_) => TArray::empty(),
                            // Property::Int16(_) => todo!(),
                            // Property::Int64(_) => todo!(),
                            // Property::Int8(_) => todo!(),
                            Self::Map(map_property) => {
                                let key_type = match map_property {
                                    FMapProperty::Known { key_type, .. } |
                                    FMapProperty::Unknown { key_type, .. } => match key_type {
                                        PropertyType::Incomplete {..} => todo!(),
                                        PropertyType::Complete { property_type, .. } => property_type.clone(),
                                    },
                                };
                                let value_type = match map_property {
                                    FMapProperty::Known { value_type, .. } |
                                    FMapProperty::Unknown { value_type, .. } => match value_type {
                                        PropertyType::Incomplete {..} => todo!(),
                                        PropertyType::Complete { property_type, .. } => property_type.clone(),
                                    },
                                };
                                TArray::from([key_type, value_type])
                            }
                            // Property::MulticastInlineDelegate(_) => todo!(),
                            // Property::MulticastSparseDelegate(_) => todo!(),
                            Self::Name(_) |
                            Self::Object(_) |
                            Self::SoftObject(_) |
                            Self::Str(_) => TArray::empty(),
                            Self::Struct(struct_property) => {
                                let struct_type = struct_property.struct_type();
                                let struct_class = struct_property.struct_class();
                                let struct_guid = struct_property.struct_guid();
                                FPropertyTypeName::for_struct(struct_type, struct_class, struct_guid).children
                            },
                            Self::Text(_) => TArray::empty(),
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

impl BinRead for FProperty {
    type Args<'a> = (SerializationFormat, &'a PropertyType);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, t): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let size = t.size();
        let start = reader.stream_position()?;

        let property_type = t.property_type().expect("property_type");

        #[rustfmt::skip] // Disable wrapping on this block
        let result = match property_type {
            NAME_ARRAY_PROPERTY  => {
                let inner_type = t.array_inner_type().expect("inner_type");
                let array_property = FArrayProperty::read_options(reader, endian, (format, t, inner_type))?;
                Self::Array(array_property)
            },
            NAME_BOOL_PROPERTY   => Self::Bool  (  FBoolProperty::read_options(reader, endian, (t,))?),
            NAME_BYTE_PROPERTY   => Self::Byte  (  FByteProperty::read_options(reader, endian, (t,))?),
            NAME_DELEGATE_PROPERTY => Self::Delegate(FDelegateProperty::read_options(reader, endian, ())?),
            NAME_DOUBLE_PROPERTY => Self::Double(FDoubleProperty::read_options(reader, endian, ())?),
            NAME_ENUM_PROPERTY   => Self::Enum  (  FEnumProperty::read_options(reader, endian, (t,))?),
            NAME_FLOAT_PROPERTY  => Self::Float ( FFloatProperty::read_options(reader, endian, ())?),
            NAME_INT16_PROPERTY  => Self::Int16 ( FInt16Property::read_options(reader, endian, ())?),
            NAME_INT64_PROPERTY  => Self::Int64 ( FInt64Property::read_options(reader, endian, ())?),
            NAME_INT8_PROPERTY   => Self::Int8  (  FInt8Property::read_options(reader, endian, ())?),
            NAME_INT_PROPERTY    => Self::Int   (   FIntProperty::read_options(reader, endian, ())?),
            NAME_MAP_PROPERTY    => Self::Map   (   FMapProperty::read_options(reader, endian, (format, t))?),
            NAME_MULTICAST_INLINE_DELGATE_PROPERTY => Self::MulticastInlineDelegate(FMulticastInlineDelegateProperty::read_options(reader, endian, ())?),
            NAME_MULTICAST_SPARSE_DELGATE_PROPERTY => Self::MulticastSparseDelegate(FMulticastSparseDelegateProperty::read_options(reader, endian, ())?),
            NAME_NAME_PROPERTY   => Self::Name  (  FNameProperty::read_options(reader, endian, ())?),
            NAME_OBJECT_PROPERTY => Self::Object(FObjectProperty::read_options(reader, endian, ())?),
            // NAME_OPTIONAL_PROPERTY => Property::Optional(OptionalProperty::read_options(reader, endian, ())?),
            NAME_SET_PROPERTY    => Self::Set   (   FSetProperty::read_options(reader, endian, (format, t))?),
            NAME_SOFT_OBJECT_PROPERTY => Self::SoftObject(FSoftObjectProperty::read_options(reader, endian, (format,))?),
            NAME_STRUCT_PROPERTY => {
                let type_name = t.struct_type_name().unwrap_or_default();
                let class_name = t.struct_class_name();
                let guid = t.struct_guid();
                let struct_property = FStructProperty::read_options(reader, endian, (format, t, type_name, class_name, guid ))?;
                Self::Struct(struct_property)
            },
            NAME_STR_PROPERTY    => Self::Str   (   FStrProperty::read_options(reader, endian, ())?),
            NAME_TEXT_PROPERTY   => Self::Text  (  FTextProperty::read_options(reader, endian, (format,))?),
            NAME_UINT16_PROPERTY => Self::UInt16(FUInt16Property::read_options(reader, endian, ())?),
            NAME_UINT32_PROPERTY => Self::UInt32(FUInt32Property::read_options(reader, endian, ())?),
            NAME_UINT64_PROPERTY => Self::UInt64(FUInt64Property::read_options(reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {property_type}");
                let mut buf = vec![0u8; size as usize];
                reader.read_exact(&mut buf)?;
                let result = Self::Unknown(t.clone(), buf);
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
            let result = Self::Unknown(t.clone(), buf);
            return Ok(result);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    // TODO: Write tests
}
