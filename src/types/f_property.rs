use binrw::{BinRead, binwrite};

use crate::{
    format::SerializationFormat,
    types::{
        CollectionProperties, EPropertyTagFlags, FArrayProperty, FBoolProperty, FDelegateProperty,
        FDoubleProperty, FEnumProperty, FFloatProperty, FGuid, FInt8Property, FInt16Property,
        FInt64Property, FIntProperty, FMapProperty, FMulticastInlineDelegateProperty,
        FMulticastSparseDelegateProperty, FNameProperty, FObjectProperty, FPropertyTypeName,
        FSetProperty, FSoftObjectProperty, FStrProperty, FString, FStructProperty, FTextProperty,
        FUInt16Property, FUInt32Property, FUInt64Property, NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY,
        NAME_BYTE_PROPERTY, NAME_DELEGATE_PROPERTY, NAME_DOUBLE_PROPERTY, NAME_ENUM_PROPERTY,
        NAME_FLOAT_PROPERTY, NAME_INT_PROPERTY, NAME_INT8_PROPERTY, NAME_INT16_PROPERTY,
        NAME_INT64_PROPERTY, NAME_MAP_PROPERTY, NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
        NAME_MULTICAST_SPARSE_DELGATE_PROPERTY, NAME_NAME_PROPERTY, NAME_OBJECT_PROPERTY,
        NAME_SET_PROPERTY, NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY, NAME_STRUCT_PROPERTY,
        NAME_TEXT_PROPERTY, NAME_UINT16_PROPERTY, NAME_UINT32_PROPERTY, NAME_UINT64_PROPERTY,
        PropertyType, TArray,
    },
};

#[binwrite]
#[bw(import(format: SerializationFormat))]
#[derive(Debug)]
pub enum FProperty {
    Array(#[bw(args(format))] FArrayProperty),
    Bool(FBoolProperty),
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
            FProperty::Array(..) => NAME_ARRAY_PROPERTY,
            FProperty::Bool(..) => NAME_BOOL_PROPERTY,
            FProperty::Delegate(..) => NAME_DELEGATE_PROPERTY,
            FProperty::Double(..) => NAME_DOUBLE_PROPERTY,
            FProperty::Enum(..) => NAME_ENUM_PROPERTY,
            FProperty::Float(..) => NAME_FLOAT_PROPERTY,
            FProperty::Int(..) => NAME_INT_PROPERTY,
            FProperty::Int16(..) => NAME_INT16_PROPERTY,
            FProperty::Int64(..) => NAME_INT64_PROPERTY,
            FProperty::Int8(..) => NAME_INT8_PROPERTY,
            FProperty::Map(..) => NAME_MAP_PROPERTY,
            FProperty::MulticastInlineDelegate(..) => NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
            FProperty::MulticastSparseDelegate(..) => NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
            FProperty::Name(..) => NAME_NAME_PROPERTY,
            FProperty::Object(..) => NAME_OBJECT_PROPERTY,
            FProperty::Set(..) => NAME_SET_PROPERTY,
            FProperty::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
            FProperty::Str(..) => NAME_STR_PROPERTY,
            FProperty::Struct(..) => NAME_STRUCT_PROPERTY,
            FProperty::Text(..) => NAME_TEXT_PROPERTY,
            FProperty::UInt16(..) => NAME_UINT16_PROPERTY,
            FProperty::UInt32(..) => NAME_UINT32_PROPERTY,
            FProperty::UInt64(..) => NAME_UINT64_PROPERTY,
            FProperty::Unknown(t, ..) => match t {
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
            FProperty::Array(array_property) => Some(match array_property {
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
    ) -> PropertyType {
        let property_type_name = self.property_type_name();
        let property_type = FString::from(property_type_name);
        let inner_type = FString::from(self.container_inner_type_name());
        match format.property_tag_complete_type_name {
            false => PropertyType::Incomplete {
                property_type,
                size,
                array_index,
                extra: match &self {
                    FProperty::Array(..) => CollectionProperties::Array { inner_type },
                    FProperty::Bool(FBoolProperty(value)) => CollectionProperties::Bool {
                        value: *value as u8,
                    },
                    // Property::Byte(..) => Collection::Byte {},
                    // Property::Enum(..) => Collection::Enum {},
                    FProperty::Map(map_property) => {
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
                    FProperty::Set(..) => CollectionProperties::Set { inner_type },
                    FProperty::Struct(p) => CollectionProperties::Struct {
                        type_name: p.struct_type(),
                        guid,
                    },
                    FProperty::Delegate(..)
                    | FProperty::Double(..)
                    | FProperty::Enum(..)
                    | FProperty::Float(..)
                    | FProperty::Int(..)
                    | FProperty::Int16(..)
                    | FProperty::Int64(..)
                    | FProperty::Int8(..)
                    | FProperty::MulticastInlineDelegate(..)
                    | FProperty::MulticastSparseDelegate(..)
                    | FProperty::Name(..)
                    | FProperty::Object(..)
                    | FProperty::SoftObject(..)
                    | FProperty::Str(..)
                    | FProperty::Text(..)
                    | FProperty::UInt16(..)
                    | FProperty::UInt32(..)
                    | FProperty::UInt64(..)
                    | FProperty::Unknown(..) => CollectionProperties::None,
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
                            FProperty::Array(_) => TArray::from([FPropertyTypeName::from(inner_type)]),
                            FProperty::Bool(FBoolProperty(value)) => {
                                flags.set_bool_true(*value);
                                TArray::empty()
                            }
                            // Property::Delegate(_) => todo!(),
                            FProperty::Double(_) |
                            // Property::Enum(_) => todo!(),
                            // Property::Float(_) => todo!(),
                            FProperty::Int(_) => TArray::empty(),
                            // Property::Int16(_) => todo!(),
                            // Property::Int64(_) => todo!(),
                            // Property::Int8(_) => todo!(),
                            FProperty::Map(map_property) => {
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
                            FProperty::Name(_) |
                            FProperty::Object(_) |
                            FProperty::SoftObject(_) |
                            FProperty::Str(_) => TArray::empty(),
                            FProperty::Struct(struct_property) => {
                                let struct_type = struct_property.struct_type();
                                let struct_class = struct_property.struct_class();
                                let struct_guid = struct_property.struct_guid();
                                let cap = if struct_guid.is_valid() { 2 } else { 1 };
                                let mut children = Vec::with_capacity(cap);
                                children.push(FPropertyTypeName::with_children(
                                    struct_type,
                                    if struct_class.0.is_none() {
                                        // Vec::new()
                                        todo!("{self:?}")
                                    } else {
                                        Vec::from([FPropertyTypeName::from(struct_class)])
                                    }
                                ));
                                if struct_guid.is_valid() {
                                    let struct_guid = struct_guid.to_string();
                                    let struct_guid = FString::from(struct_guid);
                                    let struct_guid = FPropertyTypeName::from(struct_guid);
                                    children.push(struct_guid);
                                }
                                TArray(children)
                            },
                            FProperty::Text(_) => TArray::empty(),
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
                FProperty::Array(array_property)
            },
            NAME_BOOL_PROPERTY   => FProperty::Bool  (  FBoolProperty::read_options(reader, endian, (t,))?),
            // NAME_BYTE_PROPERTY   => Property::Byte  (  ByteProperty::read_options(reader, endian, ())?),
            NAME_DELEGATE_PROPERTY => FProperty::Delegate(FDelegateProperty::read_options(reader, endian, ())?),
            NAME_DOUBLE_PROPERTY => FProperty::Double(FDoubleProperty::read_options(reader, endian, ())?),
            NAME_ENUM_PROPERTY   => FProperty::Enum  (  FEnumProperty::read_options(reader, endian, ())?),
            NAME_FLOAT_PROPERTY  => FProperty::Float ( FFloatProperty::read_options(reader, endian, ())?),
            NAME_INT16_PROPERTY  => FProperty::Int16 ( FInt16Property::read_options(reader, endian, ())?),
            NAME_INT64_PROPERTY  => FProperty::Int64 ( FInt64Property::read_options(reader, endian, ())?),
            NAME_INT8_PROPERTY   => FProperty::Int8  (  FInt8Property::read_options(reader, endian, ())?),
            NAME_INT_PROPERTY    => FProperty::Int   (   FIntProperty::read_options(reader, endian, ())?),
            NAME_MAP_PROPERTY    => FProperty::Map   (   FMapProperty::read_options(reader, endian, (format, t))?),
            NAME_MULTICAST_INLINE_DELGATE_PROPERTY => FProperty::MulticastInlineDelegate(FMulticastInlineDelegateProperty::read_options(reader, endian, ())?),
            NAME_MULTICAST_SPARSE_DELGATE_PROPERTY => FProperty::MulticastSparseDelegate(FMulticastSparseDelegateProperty::read_options(reader, endian, ())?),
            NAME_NAME_PROPERTY   => FProperty::Name  (  FNameProperty::read_options(reader, endian, ())?),
            NAME_OBJECT_PROPERTY => FProperty::Object(FObjectProperty::read_options(reader, endian, ())?),
            // NAME_OPTIONAL_PROPERTY => Property::Optional(OptionalProperty::read_options(reader, endian, ())?),
            NAME_SET_PROPERTY    => FProperty::Set   (   FSetProperty::read_options(reader, endian, (format, t))?),
            NAME_SOFT_OBJECT_PROPERTY => FProperty::SoftObject(FSoftObjectProperty::read_options(reader, endian, (format,))?),
            NAME_STRUCT_PROPERTY => {
                let type_name = t.struct_type_name().unwrap_or_default();
                let class_name = t.struct_class_name();
                let guid = t.struct_guid();
                let struct_property = FStructProperty::read_options(reader, endian, (format, t, type_name, class_name, guid ))?;
                FProperty::Struct(struct_property)
            },
            NAME_STR_PROPERTY    => FProperty::Str   (   FStrProperty::read_options(reader, endian, ())?),
            NAME_TEXT_PROPERTY   => FProperty::Text  (  FTextProperty::read_options(reader, endian, (format,))?),
            NAME_UINT16_PROPERTY => FProperty::UInt16(FUInt16Property::read_options(reader, endian, ())?),
            NAME_UINT32_PROPERTY => FProperty::UInt32(FUInt32Property::read_options(reader, endian, ())?),
            NAME_UINT64_PROPERTY => FProperty::UInt64(FUInt64Property::read_options(reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {property_type}");
                let mut buf = vec![0u8; size as usize];
                reader.read_exact(&mut buf)?;
                let result = FProperty::Unknown(t.clone(), buf);
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
            let result = FProperty::Unknown(t.clone(), buf);
            return Ok(result);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    // TODO: Write tests
}
