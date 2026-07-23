use binrw::{BinRead, binwrite};

use crate::{
    error::{PropertyTagError, binrw_custom},
    format::SerializationFormat,
    types::{
        CollectionProperties, EPropertyTagFlags, FArrayProperty, FBoolProperty, FByteProperty,
        FDelegateProperty, FDoubleProperty, FEnumProperty, FFieldPathProperty, FFloatProperty,
        FGuid, FInt8Property, FInt16Property, FInt64Property, FIntProperty, FMapProperty,
        FMulticastInlineDelegateProperty, FMulticastSparseDelegateProperty, FNameProperty,
        FObjectProperty, FPropertyTypeName, FSetProperty, FSoftObjectProperty, FStrProperty,
        FString, FStructProperty, FTextProperty, FUInt16Property, FUInt32Property, FUInt64Property,
        NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_DELEGATE_PROPERTY,
        NAME_DOUBLE_PROPERTY, NAME_ENUM_PROPERTY, NAME_FIELD_PATH_PROPERTY, NAME_FLOAT_PROPERTY,
        NAME_INT_PROPERTY, NAME_INT8_PROPERTY, NAME_INT16_PROPERTY, NAME_INT64_PROPERTY,
        NAME_MAP_PROPERTY, NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
        NAME_MULTICAST_SPARSE_DELGATE_PROPERTY, NAME_NAME_PROPERTY, NAME_NONE,
        NAME_OBJECT_PROPERTY, NAME_SET_PROPERTY, NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY,
        NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY, NAME_UINT16_PROPERTY, NAME_UINT32_PROPERTY,
        NAME_UINT64_PROPERTY, PropertyTag, PropertyTagIncompleteGuid,
    },
};

#[binwrite]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FProperty {
    #[cfg_attr(feature = "serde", serde(rename = "ArrayProperty"))]
    Array(#[bw(args(format))] FArrayProperty),
    #[cfg_attr(feature = "serde", serde(rename = "BoolProperty"))]
    Bool {
        #[bw(ignore)]
        value: FBoolProperty,
    },
    #[cfg_attr(feature = "serde", serde(rename = "ByteProperty"))]
    Byte(FByteProperty),
    #[cfg_attr(feature = "serde", serde(rename = "DelegateProperty"))]
    Delegate { value: FDelegateProperty },
    #[cfg_attr(feature = "serde", serde(rename = "DoubleProperty"))]
    Double { value: FDoubleProperty },
    #[cfg_attr(feature = "serde", serde(rename = "EnumProperty"))]
    Enum { value: FEnumProperty },
    #[cfg_attr(feature = "serde", serde(rename = "FieldPathProperty"))]
    FieldPath { value: FFieldPathProperty },
    #[cfg_attr(feature = "serde", serde(rename = "FloatProperty"))]
    Float { value: FFloatProperty },
    #[cfg_attr(feature = "serde", serde(rename = "IntProperty"))]
    Int { value: FIntProperty },
    #[cfg_attr(feature = "serde", serde(rename = "Int16Property"))]
    Int16 { value: FInt16Property },
    #[cfg_attr(feature = "serde", serde(rename = "Int64Property"))]
    Int64 { value: FInt64Property },
    #[cfg_attr(feature = "serde", serde(rename = "Int8Property"))]
    Int8 { value: FInt8Property },
    #[cfg_attr(feature = "serde", serde(rename = "MapProperty"))]
    Map(#[bw(args(format))] FMapProperty),
    #[cfg_attr(feature = "serde", serde(rename = "MulticastInlineDelegateProperty"))]
    MulticastInlineDelegate {
        value: FMulticastInlineDelegateProperty,
    },
    #[cfg_attr(feature = "serde", serde(rename = "MultiCastSparseDelegateProperty"))]
    MulticastSparseDelegate {
        value: FMulticastSparseDelegateProperty,
    },
    #[cfg_attr(feature = "serde", serde(rename = "NameProperty"))]
    Name { value: FNameProperty },
    #[cfg_attr(feature = "serde", serde(rename = "ObjectProperty"))]
    Object { value: FObjectProperty },
    #[cfg_attr(feature = "serde", serde(rename = "SetProperty"))]
    Set(#[bw(args(format))] FSetProperty),
    #[cfg_attr(feature = "serde", serde(rename = "SoftObjectProperty"))]
    SoftObject(FSoftObjectProperty),
    #[cfg_attr(feature = "serde", serde(rename = "StrProperty"))]
    Str { value: FStrProperty },
    #[cfg_attr(feature = "serde", serde(rename = "StructProperty"))]
    Struct(#[bw(args(format))] FStructProperty),
    #[cfg_attr(feature = "serde", serde(rename = "TextProperty"))]
    Text {
        #[bw(args(format))]
        value: FTextProperty,
    },
    #[cfg_attr(feature = "serde", serde(rename = "UInt16Property"))]
    UInt16 { value: FUInt16Property },
    #[cfg_attr(feature = "serde", serde(rename = "UInt32Property"))]
    UInt32 { value: FUInt32Property },
    #[cfg_attr(feature = "serde", serde(rename = "UInt64Property"))]
    UInt64 { value: FUInt64Property },
    Unknown {
        #[bw(ignore)]
        tag: PropertyTag,
        value: Vec<u8>,
    },
}

impl FProperty {
    fn property_type_name(&self) -> &str {
        match &self {
            Self::Array(..) => NAME_ARRAY_PROPERTY,
            Self::Bool { .. } => NAME_BOOL_PROPERTY,
            Self::Byte { .. } => NAME_BYTE_PROPERTY,
            Self::Delegate { .. } => NAME_DELEGATE_PROPERTY,
            Self::Double { .. } => NAME_DOUBLE_PROPERTY,
            Self::Enum { .. } => NAME_ENUM_PROPERTY,
            Self::FieldPath { .. } => NAME_FIELD_PATH_PROPERTY,
            Self::Float { .. } => NAME_FLOAT_PROPERTY,
            Self::Int { .. } => NAME_INT_PROPERTY,
            Self::Int16 { .. } => NAME_INT16_PROPERTY,
            Self::Int64 { .. } => NAME_INT64_PROPERTY,
            Self::Int8 { .. } => NAME_INT8_PROPERTY,
            Self::Map(..) => NAME_MAP_PROPERTY,
            Self::MulticastInlineDelegate { .. } => NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
            Self::MulticastSparseDelegate { .. } => NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
            Self::Name { .. } => NAME_NAME_PROPERTY,
            Self::Object { .. } => NAME_OBJECT_PROPERTY,
            Self::Set(..) => NAME_SET_PROPERTY,
            Self::SoftObject { .. } => NAME_SOFT_OBJECT_PROPERTY,
            Self::Str { .. } => NAME_STR_PROPERTY,
            Self::Struct(..) => NAME_STRUCT_PROPERTY,
            Self::Text { .. } => NAME_TEXT_PROPERTY,
            Self::UInt16 { .. } => NAME_UINT16_PROPERTY,
            Self::UInt32 { .. } => NAME_UINT32_PROPERTY,
            Self::UInt64 { .. } => NAME_UINT64_PROPERTY,
            Self::Unknown { tag, .. } => match tag {
                PropertyTag::Incomplete {
                    property_type: FString(Some(name)),
                    ..
                } => name,
                PropertyTag::Complete { property_type, .. } => property_type.name(),
                PropertyTag::Incomplete { .. } => todo!("{tag:?}"),
            },
        }
    }

    pub(crate) fn generate_tag(
        &self,
        format: &SerializationFormat,
        size: u32,
        array_index: u32,
        has_binary_or_native_serialize: bool,
        has_property_extensions: bool,
        property_guid: FGuid,
    ) -> Result<PropertyTag, PropertyTagError> {
        if let Self::Unknown { tag, .. } = self {
            return Ok(tag.clone());
        }

        if format.property_tag_complete_type_name() {
            let mut flags = EPropertyTagFlags::new();
            flags.set_has_array_index(array_index != 0);
            flags.set_has_property_guid(property_guid.is_valid());
            flags.set_has_binary_or_native_serialize(has_binary_or_native_serialize);
            flags.set_has_property_extensions(has_property_extensions);
            if let Self::Bool {
                value: FBoolProperty(value),
            } = self
            {
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
            Self::Bool {
                value: FBoolProperty(value),
            } => CollectionProperties::Bool {
                value: u8::from(*value),
            },
            Self::Byte(FByteProperty::Byte(_)) => CollectionProperties::Byte {
                enum_name: NAME_NONE.into(),
            },
            Self::Byte(FByteProperty::Enum(FEnumProperty(enum_type, _)))
            | Self::Enum {
                value: FEnumProperty(enum_type, _),
            } => CollectionProperties::Enum {
                enum_name: enum_type.enum_class_name().clone(),
            },
            Self::Map(map_property) => CollectionProperties::Map {
                inner_type: map_property.key_type().name().into(),
                value_type: map_property.value_type().name().into(),
            },
            // Property::Optional(p) => CollectionProperties::Optional { inner_type },
            Self::Set(p) => CollectionProperties::Set {
                inner_type: p.element_property_type_name().into(),
            },
            Self::Struct(p) => CollectionProperties::Struct {
                type_name: p.struct_type(),
                struct_guid: p.struct_guid(),
            },
            Self::Delegate { .. }
            | Self::Double { .. }
            | Self::FieldPath { .. }
            | Self::Float { .. }
            | Self::Int { .. }
            | Self::Int16 { .. }
            | Self::Int64 { .. }
            | Self::Int8 { .. }
            | Self::MulticastInlineDelegate { .. }
            | Self::MulticastSparseDelegate { .. }
            | Self::Name { .. }
            | Self::Object { .. }
            | Self::SoftObject { .. }
            | Self::Str { .. }
            | Self::Text { .. }
            | Self::UInt16 { .. }
            | Self::UInt32 { .. }
            | Self::UInt64 { .. } => CollectionProperties::None,
            Self::Unknown { .. } => unimplemented!(),
        }
    }

    fn generate_complete_property_type(&self) -> Result<FPropertyTypeName, PropertyTagError> {
        Ok(match self {
            Self::Array(array_property) => {
                let x = match array_property {
                    FArrayProperty::Bools { .. } => FPropertyTypeName::Bool,
                    FArrayProperty::Bytes { .. } => FPropertyTypeName::Byte(None),
                    FArrayProperty::Enums { type_name, .. } => match type_name {
                        FPropertyTypeName::Enum { .. } => type_name.clone(),
                        _ => unimplemented!("{type_name:?}"),
                    },
                    FArrayProperty::Floats { .. } => FPropertyTypeName::Float,
                    FArrayProperty::Ints { .. } => FPropertyTypeName::Int,
                    FArrayProperty::Names { .. } => FPropertyTypeName::Name,
                    FArrayProperty::Objects { .. } => FPropertyTypeName::Object,
                    FArrayProperty::SoftObjects { .. } => FPropertyTypeName::SoftObject,
                    FArrayProperty::Strs { .. } => FPropertyTypeName::Str,
                    FArrayProperty::Structs {
                        type_name,
                        class_name,
                        struct_guid,
                        values: _,
                    } => FPropertyTypeName::Struct {
                        type_name: type_name.as_ref().into(),
                        class_name: class_name.as_ref().into(),
                        struct_guid: *struct_guid,
                    },
                    FArrayProperty::TaggedStructs { .. } => unimplemented!(),
                    FArrayProperty::Texts { .. } => FPropertyTypeName::Text,
                };
                FPropertyTypeName::Array(Box::new(x))
            }
            Self::Bool { .. } => FPropertyTypeName::Bool,
            Self::Delegate { .. } => FPropertyTypeName::Delegate,
            Self::Double { .. } => FPropertyTypeName::Double,
            Self::Enum {
                value: FEnumProperty(type_name, _),
            } => type_name.clone(),
            Self::Float { .. } => FPropertyTypeName::Float,
            Self::Int { .. } => FPropertyTypeName::Int,
            Self::Int16 { .. } => FPropertyTypeName::Int16,
            Self::Int64 { .. } => FPropertyTypeName::Int64,
            Self::Int8 { .. } => FPropertyTypeName::Int8,
            Self::Map(map_property) => FPropertyTypeName::Map {
                key: Box::new(map_property.key_type().clone()),
                value: Box::new(map_property.value_type().clone()),
            },
            Self::MulticastInlineDelegate { .. } => FPropertyTypeName::MulticastInlineDelegate,
            Self::MulticastSparseDelegate { .. } => FPropertyTypeName::MulticastSparseDelegate,
            Self::Name { .. } => FPropertyTypeName::Name,
            Self::Object { .. } => FPropertyTypeName::Object,
            Self::SoftObject { .. } => FPropertyTypeName::SoftObject,
            Self::Str { .. } => FPropertyTypeName::Str,
            Self::Struct(struct_property) => FPropertyTypeName::Struct {
                type_name: struct_property.struct_type(),
                class_name: struct_property.struct_class(),
                struct_guid: struct_property.struct_guid(),
            },
            Self::Text { .. } => FPropertyTypeName::Text,
            Self::UInt16 { .. } => FPropertyTypeName::UInt16,
            Self::UInt32 { .. } => FPropertyTypeName::UInt32,
            Self::UInt64 { .. } => FPropertyTypeName::UInt64,
            _ => todo!("{self:?}"),
        })
    }
}

impl BinRead for FProperty {
    type Args<'a> = (&'a SerializationFormat, &'a PropertyTag);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, t): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let size = t.size();
        let start = reader.stream_position()?;
        let err_convert = &binrw_custom(start);

        let property_type = t.property_type_str().map_err(err_convert)?;

        #[rustfmt::skip] // Disable wrapping on this block
        let result = match property_type {
            NAME_ARRAY_PROPERTY  => {
                let inner_type = t.array_inner_type().map_err(err_convert)?;
                let array_property = FArrayProperty::read_options(reader, endian, (format, t, inner_type))?;
                Self::from(array_property)
            },
            NAME_BOOL_PROPERTY   => Self::from(  FBoolProperty::read_options(reader, endian, (t,))?),
            NAME_BYTE_PROPERTY   => Self::from(  FByteProperty::read_options(reader, endian, (t,))?),
            NAME_DELEGATE_PROPERTY => Self::from(FDelegateProperty::read_options(reader, endian, ())?),
            NAME_DOUBLE_PROPERTY => Self::from(FDoubleProperty::read_options(reader, endian, ())?),
            NAME_ENUM_PROPERTY   => Self::from(  FEnumProperty::read_options(reader, endian, (t,))?),
            NAME_FIELD_PATH_PROPERTY => Self::from(FFieldPathProperty::read_options(reader, endian, ())?),
            NAME_FLOAT_PROPERTY  => Self::from( FFloatProperty::read_options(reader, endian, ())?),
            NAME_INT16_PROPERTY  => Self::from( FInt16Property::read_options(reader, endian, ())?),
            NAME_INT64_PROPERTY  => Self::from( FInt64Property::read_options(reader, endian, ())?),
            NAME_INT8_PROPERTY   => Self::from(  FInt8Property::read_options(reader, endian, ())?),
            NAME_INT_PROPERTY    => Self::from(   FIntProperty::read_options(reader, endian, ())?),
            NAME_MAP_PROPERTY    => Self::from(   FMapProperty::read_options(reader, endian, (format, t))?),
            NAME_MULTICAST_INLINE_DELGATE_PROPERTY => Self::from(FMulticastInlineDelegateProperty::read_options(reader, endian, ())?),
            NAME_MULTICAST_SPARSE_DELGATE_PROPERTY => Self::from(FMulticastSparseDelegateProperty::read_options(reader, endian, ())?),
            NAME_NAME_PROPERTY   => Self::from(  FNameProperty::read_options(reader, endian, ())?),
            NAME_OBJECT_PROPERTY => Self::from(FObjectProperty::read_options(reader, endian, ())?),
            // NAME_OPTIONAL_PROPERTY => Property::Optional(OptionalProperty::read_options(reader, endian, ())?),
            NAME_SET_PROPERTY    => Self::from(   FSetProperty::read_options(reader, endian, (format, t))?),
            NAME_SOFT_OBJECT_PROPERTY => Self::from(FSoftObjectProperty::read_options(reader, endian, (format,))?),
            NAME_STRUCT_PROPERTY => {
                let type_name = t.struct_type_name().unwrap_or_default();
                let class_name = t.struct_class_name();
                let guid = t.struct_guid().map_err(binrw_custom(start))?;
                                    Self::from(FStructProperty::read_options(reader, endian, (format, Some(t.size()), type_name, class_name, guid))?)
            },
            NAME_STR_PROPERTY    => Self::from(   FStrProperty::read_options(reader, endian, ())?),
            NAME_TEXT_PROPERTY   => Self::from(  FTextProperty::read_options(reader, endian, (format,))?),
            NAME_UINT16_PROPERTY => Self::from(FUInt16Property::read_options(reader, endian, ())?),
            NAME_UINT32_PROPERTY => Self::from(FUInt32Property::read_options(reader, endian, ())?),
            NAME_UINT64_PROPERTY => Self::from(FUInt64Property::read_options(reader, endian, ())?),
            _ => {
                println!("Warning: Unrecognized property type {property_type}");
                let err_convert = &binrw_custom(start);
                let size = usize::try_from(size).map_err(err_convert)?;
                let mut value = vec![0u8; size];
                reader.read_exact(&mut value)?;
                let result = Self::Unknown { tag: t.clone(), value };
                return Ok(result);
            }
        };

        // Check bytes read compared to size
        let size = i64::from(size);
        let err_convert = &binrw_custom(start);
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
            let result = Self::Unknown {
                tag: t.clone(),
                value: buf,
            };
            return Ok(result);
        }

        Ok(result)
    }
}

macro_rules! from {
    ($variant:ident, $field:ident : $inner_type:ty) => {
        impl From<$inner_type> for FProperty {
            fn from($field: $inner_type) -> Self {
                Self::$variant { $field }
            }
        }
    };
    ($variant:ident, $inner_type:ty) => {
        impl From<$inner_type> for FProperty {
            fn from(value: $inner_type) -> Self {
                Self::$variant(value)
            }
        }
    };
}

from!(Array, FArrayProperty);
from!(Bool, value: FBoolProperty);
from!(Byte, FByteProperty);
from!(Delegate, value: FDelegateProperty);
from!(Double, value: FDoubleProperty);
from!(Enum, value: FEnumProperty);
from!(FieldPath, value: FFieldPathProperty);
from!(Float, value: FFloatProperty);
from!(Int, value: FIntProperty);
from!(Int16, value: FInt16Property);
from!(Int64, value: FInt64Property);
from!(Int8, value: FInt8Property);
from!(Map, FMapProperty);
from!(MulticastInlineDelegate, value: FMulticastInlineDelegateProperty);
from!(MulticastSparseDelegate, value: FMulticastSparseDelegateProperty);
from!(Name, value: FNameProperty);
from!(Object, value: FObjectProperty);
from!(Set, FSetProperty);
from!(SoftObject, FSoftObjectProperty);
from!(Str, value: FStrProperty);
from!(Struct, FStructProperty);
from!(Text, value: FTextProperty);
from!(UInt16, value: FUInt16Property);
from!(UInt32, value: FUInt32Property);
from!(UInt64, value: FUInt64Property);

#[cfg(test)]
mod test {
    // TODO: Write tests
}
