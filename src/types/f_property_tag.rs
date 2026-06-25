use std::io::Cursor;

use binrw::{BinRead, BinWrite, binrw};

use crate::format::SerializationFormat;
use crate::types::{
    EPropertyTagFlags, FGuid, FProperty, FPropertyTypeName, FString, NAME_ARRAY_PROPERTY,
    NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY, NAME_MAP_PROPERTY, NAME_NONE,
    NAME_OPTION_PROPERTY, NAME_SET_PROPERTY, NAME_STRUCT_PROPERTY,
};

#[binrw]
#[derive(Clone, Debug, Eq, PartialEq)]
#[br(import(format: SerializationFormat, property_type: &str))]
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

    #[br(pre_assert(matches!(property_type, NAME_MAP_PROPERTY) && format.property_tag_set_map_support))]
    Map {
        inner_type: FString,
        value_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_OPTION_PROPERTY)))]
    Option {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_SET_PROPERTY) && format.property_tag_set_map_support))]
    Set {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_STRUCT_PROPERTY)))]
    Struct {
        type_name: FString,
        guid: FGuid,
    },

    None,
}

#[binrw]
#[br(import(format: SerializationFormat))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertyType {
    #[br(pre_assert(!format.property_tag_complete_type_name))]
    Incomplete {
        property_type: FString,
        size: u32,
        array_index: u32,
        #[br(args(format, property_type.as_deref().unwrap_or("")))]
        extra: CollectionProperties,
        #[br(temp, assert(footer == 0))]
        #[bw(calc(0))]
        footer: u8,
    },

    #[br(pre_assert(format.property_tag_complete_type_name))]
    #[bw(assert(flags.has_array_index() == (*array_index != 0)))]
    #[bw(assert(flags.has_property_guid() == guid.is_valid()))]
    Complete {
        property_type: FPropertyTypeName,
        size: u32,
        flags: EPropertyTagFlags,
        #[brw(if(flags.has_array_index()))]
        array_index: u32,
        #[brw(if(flags.has_property_guid()))]
        guid: FGuid,
    },
}

impl PropertyType {
    #[inline]
    fn synthetic_incomplete(name: &str, size: u32) -> Self {
        Self::Incomplete {
            property_type: FString::from(name),
            size,
            array_index: 0,
            extra: CollectionProperties::None,
        }
    }

    #[inline]
    fn synthetic_complete(property_type: FPropertyTypeName, size: u32) -> Self {
        Self::Complete {
            property_type,
            size,
            flags: EPropertyTagFlags::new(),
            array_index: 0,
            guid: FGuid::invalid(),
        }
    }

    #[inline]
    pub fn array_index(&self) -> u32 {
        match self {
            Self::Incomplete { array_index, .. } | Self::Complete { array_index, .. } => {
                *array_index
            }
        }
    }

    #[inline]
    pub fn enum_type(&self) -> FPropertyTypeName {
        match self {
            Self::Incomplete { extra, .. } => FString::from(match extra {
                CollectionProperties::Byte { enum_name } => enum_name.as_deref(),
                CollectionProperties::Enum { enum_name } => enum_name.as_deref(),
                CollectionProperties::Array { .. } => None,
                _ => todo!("{extra:?}"),
            })
            .into(),
            Self::Complete { property_type, .. } => {
                match property_type.name.as_deref().unwrap_or_default() {
                    NAME_ARRAY_PROPERTY => {
                        assert_eq!(property_type.children.len(), 1);
                        let inner_type = property_type.children.first().unwrap();
                        inner_type.clone()
                    }
                    _ => todo!("{property_type}"),
                }
            }
        }
    }

    #[inline]
    pub fn flags(&self) -> Option<&EPropertyTagFlags> {
        match self {
            Self::Incomplete { .. } => None,
            Self::Complete { flags, .. } => Some(flags),
        }
    }

    #[inline]
    pub fn struct_guid(&self) -> FGuid {
        match self {
            Self::Complete {
                property_type,
                guid,
                ..
            } => {
                if guid.is_valid() {
                    todo!("valid guid in PropertyType")
                    // return *guid;
                }
                match property_type.name.as_deref().unwrap_or_default() {
                    NAME_STRUCT_PROPERTY => property_type.struct_guid().unwrap_or_default(),
                    _ => FGuid::invalid(),
                }
            }
            Self::Incomplete { extra, .. } => match extra {
                CollectionProperties::Struct { guid, .. } => *guid,
                _ => FGuid::invalid(),
            },
        }
    }

    #[inline]
    pub fn map_key_type(&self) -> Option<Self> {
        let size = 0;
        match self {
            Self::Incomplete {
                extra:
                    CollectionProperties::Map {
                        inner_type: FString(Some(name)),
                        ..
                    },
                ..
            } => Some(Self::synthetic_incomplete(name, size)),
            Self::Complete {
                property_type: FPropertyTypeName { children, .. },
                ..
            } => children
                .first()
                .cloned()
                .map(|t| Self::synthetic_complete(t, size)),
            _ => None,
        }
    }

    #[inline]
    pub fn map_value_type(&self) -> Option<Self> {
        let size = 0;
        match self {
            Self::Incomplete {
                extra:
                    CollectionProperties::Map {
                        value_type: FString(Some(name)),
                        ..
                    },
                ..
            } => Some(Self::synthetic_incomplete(name, size)),
            Self::Complete {
                property_type: FPropertyTypeName { children, .. },
                ..
            } => children
                .get(1)
                .cloned()
                .map(|t| Self::synthetic_complete(t, size)),
            _ => None,
        }
    }

    #[inline]
    pub fn set_element_type(&self) -> Option<Self> {
        let size = 0;
        match self {
            Self::Incomplete {
                extra:
                    CollectionProperties::Set {
                        inner_type: FString(Some(name)),
                    },
                ..
            } => Some(Self::synthetic_incomplete(name, size)),
            Self::Complete {
                property_type: FPropertyTypeName { children, .. },
                ..
            } => children
                .first()
                .cloned()
                .map(|t| Self::synthetic_complete(t, size)),
            _ => None,
        }
    }

    #[inline]
    pub fn property_type(&self) -> Option<&str> {
        match self {
            Self::Incomplete { property_type, .. } => property_type,
            Self::Complete { property_type, .. } => &property_type.name,
        }
        .as_deref()
    }

    #[inline]
    pub fn size(&self) -> u32 {
        match self {
            Self::Incomplete { size, .. } => *size,
            Self::Complete { size, .. } => *size,
        }
    }

    #[inline]
    pub fn array_inner_type(&self) -> Option<&FString> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Array { inner_type, .. },
                ..
            } => Some(inner_type),
            Self::Complete {
                property_type: FPropertyTypeName { children, .. },
                ..
            } => children.first().map(|head| &head.name),
            _ => None,
        }
    }

    #[inline]
    pub fn array_complete_type(&self) -> Option<&FPropertyTypeName> {
        let Self::Complete {
            property_type: FPropertyTypeName { name, children },
            ..
        } = self
        else {
            return None;
        };
        if name != NAME_ARRAY_PROPERTY {
            return None;
        }
        let [array_inner_type] = children.as_slice() else {
            panic!("children={children:?}");
            // return None;
        };
        Some(array_inner_type)
    }

    #[inline]
    pub fn array_struct_type(&self) -> Option<(&str, &str, FGuid)> {
        let Some(struct_property_type) = self.array_complete_type() else {
            panic!("self={self:?}");
            // return None;
        };
        let FPropertyTypeName {
            name: FString(Some(name)),
            children,
        } = struct_property_type
        else {
            panic!("struct_property_type={struct_property_type:?}");
            // return None;
        };
        if name != NAME_STRUCT_PROPERTY {
            panic!("name={name:?}");
            // return None;
        }
        let mut it = children.iter();
        let Some(inner) = it.next() else {
            panic!("Expected children len 1-2, got {children:?}");
            // return None;
        };
        let [class] = inner.children.as_slice() else {
            panic!("Expected inner children len 1, got {:?}", inner.children);
            // return None;
        };
        if !class.children.is_empty() {
            println!("Class children not empty: {:?}", class.children);
            return None;
        }
        // if class.name != PATH__SCRIPT__CORE_U_OBJECT {
        //     return None;
        // }
        let FString(Some(ref inner)) = inner.name else {
            return None;
        };
        let FString(Some(ref class)) = class.name else {
            return None;
        };
        let Some(guid) = it.next() else {
            return Some((inner, class, FGuid::invalid()));
        };
        if !guid.children.is_empty() {
            panic!("Guid children not empty: {:?}", guid.children);
            // return None;
        }
        let FString(Some(ref guid)) = guid.name else {
            panic!("Invalid guid name {:?}", guid.name);
            // return None;
        };
        let Ok(guid) = std::str::FromStr::from_str(guid) else {
            panic!("Invalid guid {guid}");
            // return None;
        };
        Some((inner, class, guid))
    }

    #[inline]
    pub fn struct_type_name(&self) -> Option<&str> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Struct { type_name, guid: _ },
                ..
            } => type_name.as_deref(),
            Self::Complete {
                property_type:
                    FPropertyTypeName {
                        name: FString(Some(name)),
                        children,
                    },
                ..
            } if name == NAME_STRUCT_PROPERTY => {
                let inner = children.first()?;
                inner.name.as_deref()
            }
            _ => None,
        }
    }

    #[inline]
    pub fn struct_class_name(&self) -> Option<&str> {
        match self {
            Self::Complete {
                property_type:
                    FPropertyTypeName {
                        name: FString(Some(property_type)),
                        children,
                    },
                ..
            } if property_type == NAME_STRUCT_PROPERTY => {
                let inner = children.first()?;
                let class = inner.children.first()?;
                class.name.as_deref()
            }
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FPropertyTag {
    None,
    Some {
        name: FString,
        property_type: PropertyType,
    },
}

impl BinRead for FPropertyTag {
    type Args<'a> = (SerializationFormat,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let name = FString::read_options(reader, endian, ())?;
        if name == NAME_NONE {
            return Ok(Self::None);
        };
        let property_type = PropertyType::read_options(reader, endian, (format,))?;
        Ok(Self::Some {
            name,
            property_type,
        })
    }
}

impl BinWrite for FPropertyTag {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match self {
            Self::None => Ok(writer.write_all(b"\x05\x00\x00\x00None\x00")?),
            Self::Some {
                name,
                property_type,
            } => {
                name.write_options(writer, endian, args)?;
                property_type.write_options(writer, endian, args)
            }
        }
    }
}

#[derive(Debug)]
pub struct TaggedProperty {
    pub array_index: u32,
    pub extensions: bool,
    pub guid: FGuid,
    pub native: bool,
    pub property: FProperty,
}

#[derive(Debug)]
pub struct TaggedProperties(pub Vec<(FString, TaggedProperty)>);

impl BinRead for TaggedProperties {
    type Args<'a> = (SerializationFormat,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut properties = Vec::new();
        loop {
            match FPropertyTag::read_options(reader, endian, (format,))? {
                FPropertyTag::None => break,
                FPropertyTag::Some {
                    name,
                    property_type,
                } => {
                    let property =
                        FProperty::read_options(reader, endian, (format, &property_type))?;
                    // println!("Read {property:?}");
                    let array_index = property_type.array_index();
                    let guid = property_type.struct_guid();
                    let (native, extensions) = match property_type.flags() {
                        Some(flags) => (
                            flags.has_binary_or_native_serialize(),
                            flags.has_property_extensions(),
                        ),
                        None => (false, false),
                    };
                    let property = TaggedProperty {
                        array_index,
                        extensions,
                        guid,
                        native,
                        property,
                    };
                    properties.push((name, property));
                }
            }
        }
        Ok(Self(properties))
    }
}

impl BinWrite for TaggedProperties {
    type Args<'a> = (SerializationFormat,);

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        for (name, tagged_property) in self.iter() {
            let TaggedProperty {
                array_index,
                extensions,
                guid,
                native,
                property,
            } = tagged_property;
            // Write to temp buffer
            let mut buf = Cursor::new(Vec::new());
            property.write_options(&mut buf, endian, (format,))?;
            let property_buf = buf.into_inner();
            let len = property_buf.len() as u32;

            // Generate property tag
            let property_type =
                property.property_type(format, len, *array_index, *guid, *native, *extensions);

            // Write tagged property to writer
            name.write_options(writer, endian, ())?;
            property_type.write_options(writer, endian, ())?;
            property_buf.write_options(writer, endian, ())?;
        }
        // Write the sentinel value "None" to terminate the list
        FPropertyTag::None.write_options(writer, endian, ())?;
        Ok(())
    }
}

impl std::ops::Deref for TaggedProperties {
    type Target = Vec<(FString, TaggedProperty)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TaggedProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {

    use crate::error::Result;

    use super::*;

    const OPTIONS_INCOMPLETE: SerializationFormat = SerializationFormat {
        ftext_history_date_timezone: false,
        property_tag_set_map_support: false,
        property_tag_complete_type_name: false,
        fsoftobjectpath_remove_asset_path_fnames: false,
        text_64bit_support: false,
        large_world_coordinates: false,
        include_always_sign: false,
        culture_invariant_stability: false,
    };

    const OPTIONS_COMPLETE: SerializationFormat = SerializationFormat {
        ftext_history_date_timezone: true,
        property_tag_set_map_support: true,
        property_tag_complete_type_name: true,
        fsoftobjectpath_remove_asset_path_fnames: true,
        text_64bit_support: true,
        large_world_coordinates: true,
        include_always_sign: true,
        culture_invariant_stability: true,
    };

    fn test_fpropertytag(
        tag: FPropertyTag,
        expected: &[u8],
        format: SerializationFormat,
    ) -> Result<()> {
        // Write
        let mut buf = Cursor::new(vec![]);
        tag.write_le(&mut buf)?;
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
                property_type: PropertyType::Incomplete {
                    property_type: FString::from(NAME_STRUCT_PROPERTY),
                    size: 0,
                    array_index: 0,
                    extra: CollectionProperties::Struct {
                        type_name: FString::from("TestClass"),
                        guid: FGuid::invalid(),
                    },
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
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // guid
                0, // footer
            ],
            OPTIONS_INCOMPLETE,
        )
    }

    #[test]
    fn structproperty_complete() -> Result<()> {
        test_fpropertytag(
            FPropertyTag::Some {
                name: FString::from("test"),
                property_type: PropertyType::Complete {
                    property_type: FPropertyTypeName::with_children(
                        NAME_STRUCT_PROPERTY,
                        [
                            FPropertyTypeName::with_children(
                                "TestClass",
                                [FPropertyTypeName::from("/path")],
                            ),
                            FPropertyTypeName::from("guid"),
                        ],
                    ),
                    size: 0,
                    flags: EPropertyTagFlags::new(),
                    array_index: 0,
                    guid: FGuid::invalid(),
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
                5, 0, 0, 0, b'g', b'u', b'i', b'd', 0, // child 2 name
                0, 0, 0, 0, // child 2 child count
                0, 0, 0, 0, // size
                0, // flags
            ],
            OPTIONS_COMPLETE,
        )
    }
}
