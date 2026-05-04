use binrw::binrw;
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

#[binrw]
#[br(little)]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FPropertyTag {
    #[br(pre_assert(!options.property_tag_complete_type_name))]
    Incomplete {
        name: FString,
        prop_type: FString,
        array_index: u32,

        #[br(args(prop_type.0.as_deref().expect("prop_type")))]
        extra: CollectionProperties,

        size: u32,

        #[br(assert(terminator == 0))]
        terminator: u8,
    },
    #[br(pre_assert(options.property_tag_complete_type_name))]
    Complete {
        flags: PropertyTagFlags,
        name: FString,
        prop_type: TypeTree,
        size: u32,
    },
}
