use std::io::{Cursor, Write};
use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite, binrw};
use modular_bitfield::{bitfield, prelude::B3};

use crate::options::ParsingOptions;
use crate::properties::{
    NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY,
    NAME_MAP_PROPERTY, NAME_NONE, NAME_OPTION_PROPERTY, NAME_SET_PROPERTY, NAME_STRUCT_PROPERTY,
    Property,
};
use crate::types::{FString, TArray};

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = |&x| Self::into_bytes(x))]
#[derive(Clone, Copy, Debug)]
pub struct PropertyTagFlags {
    pub has_array_index: bool,
    pub has_property_guid: bool,
    pub has_property_extensions: bool,
    pub has_binary_or_native_serialize: bool,
    pub bool_true: bool,
    #[skip]
    padding: B3,
}

#[binrw]
#[derive(Clone)]
pub struct TypeTree {
    pub name: FString,
    pub children: TArray<TypeTree>,
}

impl std::fmt::Debug for TypeTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.0.as_deref().unwrap_or(NAME_NONE);
        write!(f, "{}", name)?;
        if !self.children.is_empty() {
            write!(f, "<")?;
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", child)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

#[binrw]
#[derive(Clone, Debug)]
#[br(import(property_type: &str))]
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

    // VER_UE4_PROPERTY_TAG_SET_MAP_SUPPORT
    #[br(pre_assert(matches!(property_type, NAME_MAP_PROPERTY)))]
    Map {
        inner_type: FString,
        value_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_OPTION_PROPERTY)))]
    Option {
        inner_type: FString,
    },

    // VER_UE4_PROPERTY_TAG_SET_MAP_SUPPORT
    #[br(pre_assert(matches!(property_type, NAME_SET_PROPERTY)))]
    Set {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(property_type, NAME_STRUCT_PROPERTY)))]
    Struct {
        type_name: FString,
        guid: u128,
    },

    None,
}

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Clone, Debug)]
pub enum PropertyType {
    #[br(pre_assert(!options.property_tag_complete_type_name))]
    Incomplete {
        property_type: FString,
        size: u32,
        array_index: u32,
        #[br(args(property_type.0.as_deref().unwrap_or("")))]
        extra: CollectionProperties,
        #[br(temp, assert(footer == 0))]
        #[bw(calc(0))]
        footer: u8,
    },

    #[br(pre_assert(options.property_tag_complete_type_name))]
    Complete {
        property_type: TypeTree,
        size: u32,
        flags: PropertyTagFlags,
        #[br(if(flags.has_array_index()))]
        array_index: u32,
        #[br(if(flags.has_property_guid()))]
        guid: u128,
    },
}

impl PropertyType {
    #[inline]
    fn synthetic_incomplete(name: &str, size: u32) -> Self {
        PropertyType::Incomplete {
            property_type: FString(Some(name.to_string())),
            size,
            array_index: 0,
            extra: CollectionProperties::None,
        }
    }

    #[inline]
    fn synthetic_complete(property_type: TypeTree, size: u32) -> Self {
        PropertyType::Complete {
            property_type,
            size,
            flags: PropertyTagFlags::new(),
            array_index: 0,
            guid: 0,
        }
    }

    #[inline]
    pub fn map_key_type(&self) -> Self {
        let size = 0;
        match self {
            PropertyType::Incomplete {
                extra: CollectionProperties::Map { inner_type, .. },
                ..
            } => Self::synthetic_incomplete(inner_type.0.as_deref().unwrap_or(""), size),
            PropertyType::Complete {
                property_type: TypeTree { children, .. },
                ..
            } => children
                .first()
                .cloned()
                .map(|t| Self::synthetic_complete(t, size))
                .unwrap_or_else(|| Self::synthetic_incomplete("", size)),
            _ => Self::synthetic_incomplete("", size),
        }
    }

    #[inline]
    pub fn map_value_type(&self) -> Self {
        let size = 0;
        match self {
            PropertyType::Incomplete {
                extra:
                    CollectionProperties::Map {
                        value_type: FString(Some(name)),
                        ..
                    },
                ..
            } => Self::synthetic_incomplete(name, size),
            PropertyType::Complete {
                property_type: TypeTree { children, .. },
                ..
            } => children
                .get(1)
                .cloned()
                .map(|t| Self::synthetic_complete(t, size))
                .unwrap_or_else(|| Self::synthetic_incomplete("", size)),
            _ => Self::synthetic_incomplete("", size),
        }
    }

    #[inline]
    pub fn set_element_type(&self) -> Self {
        let size = 0;
        match self {
            PropertyType::Incomplete {
                extra:
                    CollectionProperties::Set {
                        inner_type: FString(Some(name)),
                    },
                ..
            } => Self::synthetic_incomplete(name, size),
            PropertyType::Complete {
                property_type: TypeTree { children, .. },
                ..
            } => children
                .first()
                .cloned()
                .map(|t| Self::synthetic_complete(t, size))
                .unwrap_or_else(|| Self::synthetic_incomplete("", size)),
            _ => Self::synthetic_incomplete("", size),
        }
    }

    #[inline]
    pub fn property_type(&self) -> Option<&str> {
        match self {
            PropertyType::Incomplete { property_type, .. } => property_type,
            PropertyType::Complete { property_type, .. } => &property_type.name,
        }
        .0
        .as_deref()
    }

    #[inline]
    pub fn size(&self) -> u32 {
        match self {
            PropertyType::Incomplete { size, .. } => *size,
            PropertyType::Complete { size, .. } => *size,
        }
    }

    #[inline]
    pub fn array_inner_type(&self) -> Option<&str> {
        match self {
            PropertyType::Incomplete {
                extra:
                    CollectionProperties::Array { inner_type, .. }
                    | CollectionProperties::Map { inner_type, .. }
                    | CollectionProperties::Option { inner_type, .. }
                    | CollectionProperties::Set { inner_type, .. },
                ..
            } => inner_type.0.as_deref(),
            PropertyType::Complete {
                property_type: TypeTree { children, .. },
                ..
            } => children.first().and_then(|head| head.name.0.as_deref()),
            _ => None,
        }
    }

    #[inline]
    pub fn array_complete_type(&self) -> Option<&TypeTree> {
        let PropertyType::Complete {
            property_type:
                TypeTree {
                    name: FString(Some(name)),
                    children,
                },
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
            return None;
        };
        Some(array_inner_type)
    }

    #[inline]
    pub fn array_struct_type(&self) -> Option<(&str, &str, &str)> {
        let Some(struct_property_type) = self.array_complete_type() else {
            panic!("self={self:?}");
            return None;
        };
        let TypeTree {
            name: FString(Some(name)),
            children,
        } = struct_property_type
        else {
            panic!("struct_property_type={struct_property_type:?}");
            return None;
        };
        if name != NAME_STRUCT_PROPERTY {
            panic!("name={name:?}");
            return None;
        }
        let [inner, guid] = children.as_slice() else {
            panic!("Expected children len 2, got {children:?}");
            return None;
        };
        let [class] = inner.children.as_slice() else {
            panic!("Expected inner children len 1, got {:?}", inner.children);
            return None;
        };
        if (!class.children.is_empty()) {
            println!("Class children not empty: {:?}", class.children);
            return None;
        }
        // if class.name.0.as_deref() != Some("/Script/CoreUObject") {
        //     return None;
        // }
        if (!guid.children.is_empty()) {
            println!("Guid children not empty: {:?}", guid.children);
            return None;
        }
        let FString(Some(ref inner)) = inner.name else {
            return None;
        };
        let FString(Some(ref class)) = class.name else {
            return None;
        };
        let FString(Some(ref guid)) = guid.name else {
            return None;
        };
        Some((inner, class, guid))
    }

    #[inline]
    pub fn struct_type_name(&self) -> Option<&str> {
        match self {
            PropertyType::Incomplete {
                extra: CollectionProperties::Struct { type_name, guid },
                ..
            } => type_name.0.as_deref(),
            PropertyType::Complete {
                property_type:
                    TypeTree {
                        name: FString(Some(name)),
                        children,
                    },
                ..
            } if name == NAME_STRUCT_PROPERTY => {
                let [inner] = children.as_slice() else {
                    return None;
                };
                let [class] = inner.children.as_slice() else {
                    return None;
                };
                if (!class.children.is_empty()) {
                    return None;
                }
                if class.name.0.as_deref() != Some("/Script/CoreUObject") {
                    return None;
                }
                inner.name.0.as_deref()
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum FPropertyTag {
    None,
    Some {
        name: FString,
        property_type: PropertyType,
    },
}

impl BinRead for FPropertyTag {
    type Args<'a> = (ParsingOptions,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (options,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let name = FString::read_options(reader, endian, ())?;
        if let Some(ref name) = name.0
            && name == NAME_NONE
        {
            return Ok(Self::None);
        };
        let property_type = PropertyType::read_options(reader, endian, (options,))?;
        Ok(FPropertyTag::Some {
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
            FPropertyTag::None => Ok(writer.write_all(b"\x05\x00\x00\x00None\x00")?),
            FPropertyTag::Some {
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
pub struct TaggedProperties(pub Vec<(FString, Property)>);

impl BinRead for TaggedProperties {
    type Args<'a> = (ParsingOptions,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (options,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut properties = Vec::new();
        loop {
            match FPropertyTag::read_options(reader, endian, (options,))? {
                FPropertyTag::None => break,
                FPropertyTag::Some {
                    name,
                    property_type,
                } => {
                    let property =
                        Property::read_options(reader, endian, (options, &property_type))?;
                    // println!("Read {property:?}");
                    properties.push((name, property));
                }
            }
        }
        Ok(TaggedProperties(properties))
    }
}

impl BinWrite for TaggedProperties {
    type Args<'a> = (ParsingOptions,);

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        (options,): Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        for (name, property) in self.0.iter() {
            // Write to temp buffer
            let mut buf = Cursor::new(Vec::new());
            property.write_options(&mut buf, endian, (options,))?;
            let property_buf = buf.into_inner();
            let len = property_buf.len() as u32;

            // Generate property tag
            let property_type = property.property_type(options, len);

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
