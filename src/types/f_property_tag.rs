use std::io::Cursor;

use binrw::{BinRead, BinWrite, binrw};

use crate::error::{ParseGuidError, PropertyTagError, binrw_custom};
use crate::format::SerializationFormat;
use crate::types::{
    EPropertyTagFlags, FGuid, FProperty, FPropertyTypeName, FString, NAME_ARRAY_PROPERTY,
    NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY, NAME_MAP_PROPERTY, NAME_NONE,
    NAME_OPTIONAL_PROPERTY, NAME_SET_PROPERTY, NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY,
};

#[cfg(feature = "serde")]
use crate::serde::is_default;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FPropertyTag {
    None,
    Some {
        name: FString,
        property_tag: PropertyTag,
    },
}

impl FPropertyTag {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Some { name, .. } => name.as_deref(),
            Self::None => None,
        }
    }

    pub const fn as_ref(&self) -> Option<&PropertyTag> {
        match self {
            Self::Some { property_tag, .. } => Some(property_tag),
            Self::None => None,
        }
    }
}

impl BinRead for FPropertyTag {
    type Args<'a> = (&'a SerializationFormat,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let name = FString::read_options(reader, endian, ())?;
        if name == NAME_NONE {
            return Ok(Self::None);
        }
        let property_tag = PropertyTag::read_options(reader, endian, (format,))?;
        Ok(Self::Some { name, property_tag })
    }
}

impl BinWrite for FPropertyTag {
    type Args<'a> = (&'a SerializationFormat,);

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match self {
            Self::None => Ok(writer.write_all(b"\x05\x00\x00\x00None\x00")?),
            Self::Some { name, property_tag } => {
                name.write_options(writer, endian, ())?;
                property_tag.write_options(writer, endian, args)
            }
        }
    }
}

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyTag {
    #[br(pre_assert(!format.property_tag_complete_type_name()))]
    Incomplete {
        property_type: FString,
        size: u32,
        array_index: u32,
        #[br(args(format, property_type.as_deref().unwrap_or("")))]
        extra: CollectionProperties,
        #[brw(args(format, property_type.as_deref().unwrap_or("")))]
        maybe_property_guid: PropertyTagIncompleteGuid,
    },

    #[br(pre_assert(format.property_tag_complete_type_name()))]
    #[bw(assert(flags.has_array_index() == (*array_index != 0)))]
    #[bw(assert(flags.has_property_guid() == property_guid.is_valid()))]
    Complete {
        property_type: FPropertyTypeName,
        size: u32,
        flags: EPropertyTagFlags,
        #[brw(if(flags.has_array_index()))]
        array_index: u32,
        #[brw(if(flags.has_property_guid()))]
        property_guid: FGuid,
    },
}

impl PropertyTag {
    #[inline]
    pub(crate) fn synthetic_incomplete(property_type: FString, size: u32) -> Self {
        Self::Incomplete {
            property_type,
            size,
            array_index: 0,
            extra: CollectionProperties::None,
            maybe_property_guid: PropertyTagIncompleteGuid::default(),
        }
    }

    #[inline]
    pub(crate) fn synthetic_complete(property_type: FPropertyTypeName, size: u32) -> Self {
        Self::Complete {
            property_type,
            size,
            flags: EPropertyTagFlags::new(),
            array_index: 0,
            property_guid: FGuid::default(),
        }
    }

    #[inline]
    pub const fn array_index(&self) -> u32 {
        match self {
            Self::Incomplete { array_index, .. } | Self::Complete { array_index, .. } => {
                *array_index
            }
        }
    }

    #[inline]
    pub fn enum_type(&self) -> Result<FPropertyTypeName, PropertyTagError> {
        let result: FPropertyTypeName = match self {
            Self::Incomplete {
                property_type,
                extra,
                ..
            } => FPropertyTypeName::Enum {
                enum_class: match extra {
                    CollectionProperties::Byte { enum_name }
                    | CollectionProperties::Enum { enum_name } => enum_name.clone(),
                    CollectionProperties::Array { .. } => FString::null(),
                    CollectionProperties::None if property_type == NAME_ENUM_PROPERTY => {
                        FString::null()
                    }
                    _ => Err(PropertyTagError::Unsupported(
                        "enum_type".into(),
                        format!("{self:?}"),
                    ))?,
                },
                class_path: None,
                inner_type: None,
            },
            Self::Complete { property_type, .. } => match property_type {
                FPropertyTypeName::Array(inner) => *inner.clone(),
                other => other.clone(),
            },
        };
        Ok(result)
    }

    #[inline]
    pub const fn flags(&self) -> Option<&EPropertyTagFlags> {
        match self {
            Self::Incomplete { .. } => None,
            Self::Complete { flags, .. } => Some(flags),
        }
    }

    #[inline]
    fn guid(&self) -> FGuid {
        match self {
            Self::Incomplete {
                maybe_property_guid,
                ..
            } => maybe_property_guid.into(),
            Self::Complete { property_guid, .. } => *property_guid,
        }
    }

    #[inline]
    pub fn struct_guid(&self) -> Result<FGuid, ParseGuidError> {
        let result = match self {
            Self::Complete { property_type, .. } => property_type.struct_guid().unwrap_or_default(),
            Self::Incomplete {
                extra: CollectionProperties::Struct { struct_guid, .. },
                ..
            } => *struct_guid,
            Self::Incomplete { .. } => FGuid::default(),
        };
        Ok(result)
    }

    #[inline]
    pub fn map_key_type(&self) -> Result<FPropertyTypeName, PropertyTagError> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Map { inner_type, .. },
                ..
            } => FPropertyTypeName::from_name(inner_type.as_deref()).ok_or_else(|| {
                PropertyTagError::Unsupported("map_key_type".into(), format!("{self:?}"))
            }),
            Self::Complete {
                property_type: FPropertyTypeName::Map { key, .. },
                ..
            } => Ok(*key.clone()),
            _ => Err(PropertyTagError::Unsupported(
                "map_key_type".into(),
                format!("{self:?}"),
            )),
        }
    }

    #[inline]
    pub fn map_value_type(&self) -> Result<FPropertyTypeName, PropertyTagError> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Map { value_type, .. },
                ..
            } => FPropertyTypeName::from_name(value_type.as_deref()).ok_or_else(|| {
                PropertyTagError::Unsupported("map_value_type".into(), format!("{self:?}"))
            }),
            Self::Complete {
                property_type: FPropertyTypeName::Map { value, .. },
                ..
            } => Ok(*value.clone()),
            _ => Err(PropertyTagError::Unsupported(
                "map_value_type".into(),
                format!("{self:?}"),
            )),
        }
    }

    #[inline]
    pub fn set_element_type(&self) -> Result<FPropertyTypeName, PropertyTagError> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Set { inner_type },
                ..
            } => FPropertyTypeName::from_name(inner_type.as_deref()).ok_or_else(|| {
                PropertyTagError::Unsupported("set_element_type".into(), format!("{self:?}"))
            }),
            Self::Complete {
                property_type: FPropertyTypeName::Set(e),
                ..
            } => Ok(*e.clone()),
            _ => Err(PropertyTagError::Unsupported(
                "set_element_tag".into(),
                format!("{self:?}"),
            )),
        }
    }

    #[inline]
    pub fn property_type_str(&self) -> Result<&str, PropertyTagError> {
        match self {
            Self::Incomplete { property_type, .. } => property_type.as_deref(),
            Self::Complete { property_type, .. } => Some(property_type.name()),
        }
        .ok_or_else(|| {
            PropertyTagError::Unsupported("property_type_str".into(), format!("{self:?}"))
        })
    }

    #[inline]
    pub fn property_type_name(&self) -> Result<FPropertyTypeName, PropertyTagError> {
        match self {
            Self::Complete { property_type, .. } => Some(property_type.clone()),
            Self::Incomplete {
                property_type,
                extra,
                ..
            } => FPropertyTypeName::from_incomplete(property_type, extra),
        }
        .ok_or_else(|| {
            PropertyTagError::Unsupported("property_type_name".into(), format!("{self:?}"))
        })
    }

    #[inline]
    pub const fn size(&self) -> u32 {
        match self {
            Self::Incomplete { size, .. } => *size,
            Self::Complete { size, .. } => *size,
        }
    }

    #[inline]
    pub fn array_inner_type(&self) -> Result<&str, PropertyTagError> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Array { inner_type, .. },
                ..
            } => inner_type.as_deref(),
            Self::Complete {
                property_type: FPropertyTypeName::Array(inner_type),
                ..
            } => Some(inner_type.name()),
            _ => None,
        }
        .ok_or_else(|| {
            PropertyTagError::Unsupported("array_inner_type".into(), format!("{self:?}"))
        })
    }

    #[inline]
    pub fn array_struct_type(&self) -> Result<(&str, &str, FGuid), PropertyTagError> {
        match &self {
            Self::Complete {
                property_type: FPropertyTypeName::Array(inner_type),
                ..
            } => Some(inner_type.as_ref()),
            _ => None,
        }
        .and_then(|inner_type| match inner_type {
            FPropertyTypeName::StructComplete {
                type_name: FString(Some(type_name)),
                class_name: FString(Some(class_name)),
                struct_guid,
            } => Some((type_name.as_str(), class_name.as_str(), *struct_guid)),
            _ => None,
        })
        .ok_or_else(|| {
            PropertyTagError::Unsupported("array_struct_type".into(), format!("{self:?}"))
        })
    }

    #[inline]
    pub fn struct_type_name(&self) -> Option<&str> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Struct { type_name, .. },
                ..
            } => type_name.as_deref(),
            Self::Complete {
                property_type: FPropertyTypeName::StructComplete { type_name, .. },
                ..
            } => type_name.as_deref(),
            _ => None,
        }
    }

    #[inline]
    pub fn struct_class_name(&self) -> Option<&str> {
        match self {
            Self::Complete {
                property_type: FPropertyTypeName::StructComplete { class_name, .. },
                ..
            } => class_name.as_deref(),
            _ => None,
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Eq, PartialEq)]
#[br(import(format: &SerializationFormat, property_type: &str))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollectionProperties {
    #[br(pre_assert(matches!(property_type, NAME_ARRAY_PROPERTY)))]
    Array {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_BOOL_PROPERTY)))]
    Bool {
        value: u8,
    },

    #[br(pre_assert(matches!(property_type, NAME_BYTE_PROPERTY)))]
    Byte {
        enum_name: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_ENUM_PROPERTY)))]
    Enum {
        enum_name: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_MAP_PROPERTY) && format.property_tag_set_map_support()))]
    Map {
        inner_type: FString,
        value_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_OPTIONAL_PROPERTY)))]
    Option {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_SET_PROPERTY) && format.property_tag_set_map_support()))]
    Set {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_STRUCT_PROPERTY)))]
    Struct {
        type_name: FString,
        struct_guid: FGuid,
    },

    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyTagIncompleteGuid(pub Option<FGuid>);

impl BinRead for PropertyTagIncompleteGuid {
    type Args<'a> = (&'a SerializationFormat, &'a str);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, property_type): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let has_property_guid = if format.property_guid_in_property_tag() {
            match u8::read_options(reader, endian, ())? {
                0 => false,
                1 => true,
                l => unimplemented!("unsupported length: {l}"),
            }
        } else {
            property_type == NAME_TEXT_PROPERTY
        };
        let maybe_guid = has_property_guid
            .then(|| FGuid::read_options(reader, endian, ()))
            .transpose()?;
        Ok(Self(maybe_guid))
    }
}

impl BinWrite for PropertyTagIncompleteGuid {
    type Args<'a> = (&'a SerializationFormat, &'a str);

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        (format, property_type): Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        if format.property_guid_in_property_tag() {
            match self.0 {
                None => {
                    let has_property_guid = 0u8;
                    has_property_guid.write_options(writer, endian, ())
                }
                Some(guid) => {
                    let has_property_guid = 1u8;
                    has_property_guid.write_options(writer, endian, ())?;
                    guid.write_options(writer, endian, ())
                }
            }
        } else {
            let guid = FGuid::from(self);
            if property_type == NAME_TEXT_PROPERTY {
                guid.write_options(writer, endian, ())
            } else {
                assert!(!guid.is_valid(), "illegal guid for this property");
                Ok(())
            }
        }
    }
}

impl From<FGuid> for PropertyTagIncompleteGuid {
    fn from(value: FGuid) -> Self {
        Self(value.is_valid().then_some(value))
    }
}

impl From<&PropertyTagIncompleteGuid> for FGuid {
    fn from(value: &PropertyTagIncompleteGuid) -> Self {
        value.0.unwrap_or_default()
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaggedProperty {
    pub property_name: FString,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
    pub array_index: u32,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
    pub has_binary_or_native_serialize: bool,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
    pub has_property_extensions: bool,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
    pub property_guid: FGuid,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub property: FProperty,
}

impl TaggedProperty {
    fn new(property_name: FString, property_tag: PropertyTag, property: FProperty) -> Self {
        let array_index = property_tag.array_index();
        let property_guid = property_tag.guid();
        let flags = property_tag.flags();
        let (has_binary_or_native_serialize, has_property_extensions) = (
            flags.is_some_and(EPropertyTagFlags::has_binary_or_native_serialize),
            flags.is_some_and(EPropertyTagFlags::has_property_extensions),
        );
        Self {
            property_name,
            array_index,
            has_binary_or_native_serialize,
            has_property_extensions,
            property,
            property_guid,
        }
    }
}

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaggedProperties(
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::tagged_properties_map"))]
    pub  Vec<TaggedProperty>,
);

impl BinRead for TaggedProperties {
    type Args<'a> = (&'a SerializationFormat,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut properties = Vec::new();
        loop {
            match FPropertyTag::read_options(reader, endian, (format,))? {
                FPropertyTag::None => break,
                FPropertyTag::Some { name, property_tag } => {
                    let property =
                        FProperty::read_options(reader, endian, (format, &property_tag))?;
                    // println!("Read {property:?}");
                    let property = TaggedProperty::new(name, property_tag, property);
                    properties.push(property);
                }
            }
        }
        Ok(Self(properties))
    }
}

impl BinWrite for TaggedProperties {
    type Args<'a> = (&'a SerializationFormat,);

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        for tagged_property in self.iter() {
            let TaggedProperty {
                property_name,
                array_index,
                has_binary_or_native_serialize,
                has_property_extensions,
                property,
                property_guid,
            } = tagged_property;
            // Write to temp buffer
            let mut buf = Cursor::new(Vec::new());
            property.write_options(&mut buf, endian, (format,))?;
            let property_buf = buf.into_inner();
            let pos = writer.stream_position()?;
            let len = property_buf.len();
            let len = u32::try_from(len).map_err(binrw_custom(pos))?;

            // Generate property tag
            let property_tag = property
                .generate_tag(
                    format,
                    len,
                    *array_index,
                    *has_binary_or_native_serialize,
                    *has_property_extensions,
                    *property_guid,
                )
                .map_err(binrw_custom(pos))?;

            // Write tagged property to writer
            property_name.write_options(writer, endian, ())?;
            property_tag.write_options(writer, endian, (format,))?;
            property_buf.write_options(writer, endian, ())?;
        }
        // Write the sentinel value "None" to terminate the list
        FPropertyTag::None.write_options(writer, endian, (format,))?;
        Ok(())
    }
}

impl std::fmt::Debug for TaggedProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TaggedProperties::from")
            .field(&self.0)
            .finish()
    }
}

impl std::ops::Deref for TaggedProperties {
    type Target = Vec<TaggedProperty>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TaggedProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<[TaggedProperty; N]> for TaggedProperties {
    fn from(value: [TaggedProperty; N]) -> Self {
        Self(Vec::from(value))
    }
}

impl From<Vec<TaggedProperty>> for TaggedProperties {
    fn from(value: Vec<TaggedProperty>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        error::Result,
        types::{
            EEditorObjectVersion, EUE5ReleaseStreamObjectVersion, EUnrealEngineObjectUE4Version,
            EUnrealEngineObjectUE5Version,
        },
    };

    use super::*;

    const FORMAT_INCOMPLETE: &SerializationFormat = &SerializationFormat::from_enums(
        EUnrealEngineObjectUE4Version::OldestLoadablePackage,
        None,
        EUE5ReleaseStreamObjectVersion::BeforeCustomVersionWasAdded,
        EEditorObjectVersion::BeforeCustomVersionWasAdded,
    );

    const FORMAT_COMPLETE: &SerializationFormat = &SerializationFormat::from_enums(
        EUnrealEngineObjectUE4Version::AutomaticVersionPlusOne,
        Some(EUnrealEngineObjectUE5Version::AutomaticVersionPlusOne),
        EUE5ReleaseStreamObjectVersion::AutomaticVersionPlusOne,
        EEditorObjectVersion::AutomaticVersionPlusOne,
    );

    fn test_fpropertytag(
        tag: FPropertyTag,
        expected: &[u8],
        format: &SerializationFormat,
    ) -> Result<()> {
        // Write
        let mut buf = Cursor::new(vec![]);
        tag.write_le_args(&mut buf, (format,))?;
        let buf = buf.into_inner();
        assert_eq!(buf, expected);

        // Read
        let mut cursor = Cursor::new(Vec::from(expected));
        let read = FPropertyTag::read_le_args(&mut cursor, (format,))?;
        assert_eq!(tag, read);

        Ok(())
    }

    #[test]
    fn structproperty_incomplete() -> Result<()> {
        test_fpropertytag(
            FPropertyTag::Some {
                name: FString::from("test"),
                property_tag: PropertyTag::Incomplete {
                    property_type: FString::from(NAME_STRUCT_PROPERTY),
                    size: 0,
                    array_index: 0,
                    extra: CollectionProperties::Struct {
                        type_name: FString::from("TestClass"),
                        struct_guid: FGuid::default(),
                    },
                    maybe_property_guid: PropertyTagIncompleteGuid::default(),
                },
            },
            &[
                5, 0, 0, 0, b't', b'e', b's', b't', 0, // name
                15, 0, 0, 0, b'S', b't', b'r', b'u', b'c', b't', b'P', b'r', b'o', b'p', b'e',
                b'r', b't', b'y', 0, // type
                0, 0, 0, 0, // size
                0, 0, 0, 0, // array_index
                10, 0, 0, 0, b'T', b'e', b's', b't', b'C', b'l', b'a', b's', b's',
                0, // type_name
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // struct_guid
            ],
            FORMAT_INCOMPLETE,
        )
    }

    #[test]
    fn structproperty_complete() -> Result<()> {
        test_fpropertytag(
            FPropertyTag::Some {
                name: FString::from("test"),
                property_tag: PropertyTag::Complete {
                    property_type: FPropertyTypeName::StructComplete {
                        type_name: "TestClass".into(),
                        class_name: "/path".into(),
                        struct_guid: FGuid::from_u32(
                            0xFF000088, 0xEE111199, 0xDD2222AA, 0xCC3333BB,
                        ),
                    },
                    size: 0,
                    flags: EPropertyTagFlags::new(),
                    array_index: 0,
                    property_guid: FGuid::default(),
                },
            },
            &[
                5, 0, 0, 0, b't', b'e', b's', b't', 0, // name
                15, 0, 0, 0, b'S', b't', b'r', b'u', b'c', b't', b'P', b'r', b'o', b'p', b'e',
                b'r', b't', b'y', 0, // type
                2, 0, 0, 0, // root child count
                10, 0, 0, 0, b'T', b'e', b's', b't', b'C', b'l', b'a', b's', b's',
                0, // child 1 name
                1, 0, 0, 0, // child 1 child count
                6, 0, 0, 0, b'/', b'p', b'a', b't', b'h', 0, // child 1.1 name
                0, 0, 0, 0, // child 1.1 child count
                37, 0, 0, 0, b'f', b'f', b'0', b'0', b'0', b'0', b'8', b'8', b'-', b'e', b'e',
                b'1', b'1', b'-', b'1', b'1', b'9', b'9', b'-', b'd', b'd', b'2', b'2', b'-', b'2',
                b'2', b'a', b'a', b'c', b'c', b'3', b'3', b'3', b'3', b'b', b'b',
                0, // child 2 name
                0, 0, 0, 0, // child 2 child count
                0, 0, 0, 0, // size
                0, // flags
            ],
            FORMAT_COMPLETE,
        )
    }
}
