use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite, binrw};
use modular_bitfield::{bitfield, prelude::B3};

use crate::types::{FString, ParsingOptions};

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
#[derive(Debug)]
pub struct TypeTree {
    pub name: FString,
    pub child_count: u32,
    #[br(count = child_count)]
    pub children: Vec<TypeTree>,
}

// const NAME_ArrayProperty: &str = "ArrayProperty";
// const NAME_MapProperty: &str = "MapProperty";
// const NAME_SetProperty: &str = "SetProperty";
// const NAME_StructProperty: &str = "StructProperty";

#[binrw]
#[derive(Debug)]
#[br(import(prop_type: &str))]
pub enum CollectionProperties {
    #[br(pre_assert(matches!(prop_type, "ArrayProperty")))]
    Array {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(prop_type, "BoolProperty")))]
    Bool {
        value: u8,
    },

    #[br(pre_assert(matches!(prop_type, "ByteProperty")))]
    Byte {
        enum_name: FString,
    },

    #[br(pre_assert(matches!(prop_type, "EnumProperty")))]
    Enum {
        enum_name: FString,
    },

    // VER_UE4_PROPERTY_TAG_SET_MAP_SUPPORT
    #[br(pre_assert(matches!(prop_type, "MapProperty")))]
    Map {
        inner_type: FString,
        value_type: FString,
    },

    #[br(pre_assert(matches!(prop_type, "OptionProperty")))]
    Option {
        inner_type: FString,
    },

    // VER_UE4_PROPERTY_TAG_SET_MAP_SUPPORT
    #[br(pre_assert(matches!(prop_type, "SetProperty")))]
    Set {
        inner_type: FString,
    },

    #[br(pre_assert(matches!(prop_type, "StructProperty")))]
    Struct {
        type_name: FString,
        guid: u128,
    },

    None,
}

#[derive(Debug)]
pub enum FPropertyTag {
    None,

    Incomplete {
        name: FString,
        prop_type: FString,
        // size: u32,
        array_index: u32,
        extra: CollectionProperties,
        // terminator: u8,
    },

    Complete {
        // flags: PropertyTagFlags,
        name: FString,
        prop_type: TypeTree,
        // size: u32,
        array_index: u32,
        guid: u128,
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
            Some(PropertyTagFlags::read_options(reader, endian, ())?)
        } else {
            None
        };

        let name = FString::read_options(reader, endian, ())?;

        if name.0.as_deref() == Some("None") {
            return Ok(Self::None);
        }

        // ---- read rest depending on mode
        if options.property_tag_complete_type_name {
            let flags = flags.expect("flags required in complete mode");

            let prop_type = TypeTree::read_options(reader, endian, ())?;

            let mut array_index = 0;

            if flags.has_array_index() {
                array_index = u32::read_options(reader, endian, ())?;
            }

            let guid = if flags.has_property_guid() {
                u128::read_options(reader, endian, ())?
            } else {
                0u128
            };

            let size = u32::read_options(reader, endian, ())?;

            let mut property = vec![0u8; size as usize];
            reader.read_exact(&mut property)?;

            Ok(FPropertyTag::Complete {
                // flags,
                name,
                prop_type,
                // size,
                array_index,
                guid,
            })
        } else {
            let prop_type = FString::read_options(reader, endian, ())?;

            let size = u32::read_options(reader, endian, ())?;

            let array_index = u32::read_options(reader, endian, ())?;

            let options = (prop_type.0.as_deref().expect("prop_type"),);
            let extra = CollectionProperties::read_options(reader, endian, options)?;

            let terminator = u8::read_options(reader, endian, ())?;

            debug_assert_eq!(terminator, 0);

            Ok(FPropertyTag::Incomplete {
                name,
                prop_type,
                // size,
                array_index,
                extra,
                // terminator,
            })
        }
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

impl Deref for TaggedProperties {
    type Target = Vec<FPropertyTag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TaggedProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
