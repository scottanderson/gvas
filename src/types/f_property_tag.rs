use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite, binrw};
use modular_bitfield::{bitfield, prelude::B3};

use crate::options::ParsingOptions;
use crate::properties::{
    NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY,
    NAME_MAP_PROPERTY, NAME_OPTION_PROPERTY, NAME_SET_PROPERTY, NAME_STRUCT_PROPERTY, Property,
};
use crate::types::FString;

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = |&x| Self::into_bytes(x))]
#[derive(Clone, Copy, Debug, Default)]
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
pub struct TypeTree {
    pub name: FString,
    pub child_count: u32,
    #[br(count = child_count)]
    pub children: Vec<TypeTree>,
}

impl std::fmt::Debug for TypeTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.0.as_deref().unwrap_or("None");
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
#[derive(Debug)]
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
#[br(import(options: ParsingOptions, flags: PropertyTagFlags))]
#[derive(Debug)]
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
        #[br(if(flags.has_array_index()))]
        array_index: u32,
        #[br(if(flags.has_property_guid()))]
        guid: u128,
        size: u32,
    },
}

#[derive(Debug)]
pub enum FPropertyTag {
    None,
    Some {
        name: String,
        property_type: PropertyType,
        property: Property,
    },
}

impl BinRead for FPropertyTag {
    type Args<'a> = ParsingOptions;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        options: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let flags = if options.property_tag_complete_type_name {
            PropertyTagFlags::read_options(reader, endian, ())?
        } else {
            PropertyTagFlags::default()
        };

        let name = FString::read_options(reader, endian, ())?;
        let name = match name.0 {
            None => return Ok(Self::None),
            Some(name) if name == "None" => return Ok(Self::None),
            Some(name) => name,
        };
        println!("Name = {:?}", name);

        let property_type = PropertyType::read_options(reader, endian, (options, flags))?;
        println!("Type = {:?}", property_type);

        let property = Property::read_options(reader, endian, (options, &property_type))?;
        println!("Property = {:?}", property);

        Ok(FPropertyTag::Some {
            name,
            property_type,
            property,
        })
    }
}

impl BinWrite for FPropertyTag {
    type Args<'a> = ParsingOptions;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TaggedProperties(pub Vec<FPropertyTag>);

impl BinRead for TaggedProperties {
    type Args<'a> = (ParsingOptions,);

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (options,): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut properties = Vec::new();

        loop {
            let property = FPropertyTag::read_options(reader, endian, options)?;
            match (property) {
                FPropertyTag::None => break,
                _ => properties.push(property),
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
        for tag in &self.0 {
            tag.write_options(writer, endian, options)?;
        }

        if options.property_tag_complete_type_name {
            let flags = PropertyTagFlags::default();
            flags.write_options(writer, endian, ())?;
        }
        FString(Some("None".to_string())).write_options(writer, endian, ())?;

        Ok(())
    }
}
