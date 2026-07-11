use binrw::{BinRead, BinResult, binwrite};

use crate::{
    error::binrw_custom,
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
        NAME_UINT16_PROPERTY, NAME_UINT32_PROPERTY, NAME_UINT64_PROPERTY, PropertyTag,
        PropertyTagIncompleteGuid,
    },
};

#[binwrite]
#[bw(import(format: SerializationFormat))]
#[derive(Debug, PartialEq)]
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
    Unknown(#[bw(ignore)] PropertyTag, Vec<u8>),
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
                PropertyTag::Incomplete {
                    property_type: FString(Some(name)),
                    ..
                } => name,
                PropertyTag::Complete { property_type, .. } => property_type.name(),
                PropertyTag::Incomplete { .. } => todo!("{t:?}"),
            },
        }
    }

    pub(crate) fn generate_tag(
        &self,
        format: SerializationFormat,
        size: u32,
        array_index: u32,
        has_binary_or_native_serialize: bool,
        has_property_extensions: bool,
        property_guid: FGuid,
    ) -> BinResult<PropertyTag> {
        if let Self::Unknown(original_tag, _) = self {
            return Ok(original_tag.clone());
        }

        if format.property_tag_complete_type_name {
            let mut flags = EPropertyTagFlags::new();
            flags.set_has_array_index(array_index != 0);
            flags.set_has_property_guid(property_guid.is_valid());
            flags.set_has_binary_or_native_serialize(has_binary_or_native_serialize);
            flags.set_has_property_extensions(has_property_extensions);
            if let Self::Bool(FBoolProperty(value)) = self {
                flags.set_bool_true(*value);
            }
            let property_type = self.generate_complete_property_type()?;
            Ok(PropertyTag::Complete {
                property_type,
                size,
                flags,
                array_index,
                property_guid,
            })
        } else {
            let property_type = FString::from(self.property_type_name());
            let extra = self.generate_incomplete_property_extra();
            let maybe_property_guid = PropertyTagIncompleteGuid::from(property_guid);
            Ok(PropertyTag::Incomplete {
                property_type,
                size,
                array_index,
                extra,
                maybe_property_guid,
            })
        }
    }

    fn generate_incomplete_property_extra(&self) -> CollectionProperties {
        match &self {
            Self::Array(p) => CollectionProperties::Array {
                inner_type: p.element_property_type_name().into(),
            },
            Self::Bool(FBoolProperty(value)) => CollectionProperties::Bool {
                value: u8::from(*value),
            },
            Self::Byte(FByteProperty::Byte(_)) => CollectionProperties::Byte {
                enum_name: NAME_NONE.into(),
            },
            Self::Byte(FByteProperty::Enum(FEnumProperty(enum_type, _)))
            | Self::Enum(FEnumProperty(enum_type, _)) => CollectionProperties::Enum {
                enum_name: enum_type.enum_class_name().clone(),
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

                let PropertyTag::Incomplete {
                    property_type: key_type,
                    ..
                } = key_type
                else {
                    todo!()
                };
                let PropertyTag::Incomplete {
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
            Self::Set(p) => CollectionProperties::Set {
                inner_type: p.element_property_type_name().into(),
            },
            Self::Struct(p) => CollectionProperties::Struct {
                type_name: p.struct_type(),
                struct_guid: p.struct_guid(),
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
        }
    }

    fn generate_complete_property_type(&self) -> BinResult<FPropertyTypeName> {
        Ok(match self {
            Self::Array(array_property) => {
                let x = match array_property {
                    FArrayProperty::Bool(_) => FPropertyTypeName::Bool,
                    FArrayProperty::Byte(_) => FPropertyTypeName::Byte(None),
                    FArrayProperty::Enum(enum_type, _) => match enum_type {
                        FPropertyTypeName::Enum { .. } => enum_type.clone(),
                        _ => unimplemented!("{enum_type:?}"),
                    },
                    FArrayProperty::Float(_) => FPropertyTypeName::Float,
                    FArrayProperty::Int(_) => FPropertyTypeName::Int,
                    FArrayProperty::Name(_) => FPropertyTypeName::Name,
                    FArrayProperty::Object(_) => FPropertyTypeName::Object,
                    FArrayProperty::SoftObject(_) => FPropertyTypeName::SoftObject,
                    FArrayProperty::Str(_) => FPropertyTypeName::Str,
                    FArrayProperty::Struct {
                        type_name,
                        class_name,
                        struct_guid,
                        values: _,
                    } => FPropertyTypeName::Struct {
                        type_name: type_name.clone(),
                        class_name: class_name.clone(),
                        struct_guid: *struct_guid,
                    },
                    FArrayProperty::TaggedStruct { .. } => unimplemented!(),
                    FArrayProperty::Text(_) => FPropertyTypeName::Text,
                };
                FPropertyTypeName::Array(Box::new(x))
            }
            Self::Bool(_) => FPropertyTypeName::Bool,
            // Property::Delegate(_) => todo!(),
            Self::Double(_) => FPropertyTypeName::Double,
            // Property::Enum(_) => todo!(),
            // Property::Float(_) => todo!(),
            Self::Int(_) => FPropertyTypeName::Int,
            // Property::Int16(_) => todo!(),
            // Property::Int64(_) => todo!(),
            // Property::Int8(_) => todo!(),
            Self::Map(map_property) => FPropertyTypeName::Map {
                key: Box::new(map_property.key_type().property_type_field()?),
                value: Box::new(map_property.value_type().property_type_field()?),
            },
            // Property::MulticastInlineDelegate(_) => todo!(),
            // Property::MulticastSparseDelegate(_) => todo!(),
            Self::Name(_) => FPropertyTypeName::Name,
            Self::Object(_) => FPropertyTypeName::Object,
            Self::SoftObject(_) => FPropertyTypeName::SoftObject,
            Self::Str(_) => FPropertyTypeName::Str,
            Self::Struct(struct_property) => FPropertyTypeName::Struct {
                type_name: struct_property.struct_type(),
                class_name: struct_property.struct_class(),
                struct_guid: struct_property.struct_guid(),
            },
            Self::Text(_) => FPropertyTypeName::Text,
            // Property::UInt16(_) => todo!(),
            // Property::UInt32(_) => todo!(),
            // Property::UInt64(_) => todo!(),
            _ => todo!("{self:?}"),
        })
    }
}

impl BinRead for FProperty {
    type Args<'a> = (SerializationFormat, &'a PropertyTag);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, t): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let size = t.size();
        let start = reader.stream_position()?;
        let err_convert = &binrw_custom(start);

        let property_type = t.property_type()?;

        #[rustfmt::skip] // Disable wrapping on this block
        let result = match property_type {
            NAME_ARRAY_PROPERTY  => {
                let inner_type = t.array_inner_type()?;
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
                let guid = t.struct_guid().map_err(binrw_custom(start))?;
                                    Self::Struct(FStructProperty::read_options(reader, endian, (format, t, type_name, class_name, guid))?)
            },
            NAME_STR_PROPERTY    => Self::Str   (   FStrProperty::read_options(reader, endian, ())?),
            NAME_TEXT_PROPERTY   => Self::Text  (  FTextProperty::read_options(reader, endian, (format,))?),
            NAME_UINT16_PROPERTY => Self::UInt16(FUInt16Property::read_options(reader, endian, ())?),
            NAME_UINT32_PROPERTY => Self::UInt32(FUInt32Property::read_options(reader, endian, ())?),
            NAME_UINT64_PROPERTY => Self::UInt64(FUInt64Property::read_options(reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {property_type}");
                let size = usize::try_from(size).map_err(err_convert)?;
                let mut buf = vec![0u8; size];
                reader.read_exact(&mut buf)?;
                let result = Self::Unknown(t.clone(), buf);
                return Ok(result);
            }
        };

        // Check bytes read compared to size
        let size = i64::from(size);
        let start = i64::try_from(start).map_err(err_convert)?;
        let pos = reader.stream_position()?;
        let pos = i64::try_from(pos).map_err(err_convert)?;
        let bytes_read = pos - start;
        if size != 0 && bytes_read != size {
            let remaining = size - bytes_read;
            reader.seek_relative(-bytes_read)?;
            let kind = if remaining < 0 {
                "overflow"
            } else {
                "underflow"
            };
            let offset = remaining.abs();
            eprintln!(
                "Warning: Reader position {kind}: Bytes read 0x{bytes_read:04X} does not match size 0x{size:04X} for {t:#?}: 0x{offset:04X}",
            );
            let size = usize::try_from(size).map_err(err_convert)?;
            let mut buf = vec![0u8; size];
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
