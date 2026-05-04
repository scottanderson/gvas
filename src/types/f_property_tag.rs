use binrw::binrw;
use modular_bitfield::{bitfield, prelude::B27};

use crate::types::{FString, ParsingOptions};

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = Self::read_bytes)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PropertyTagFlags {
    pub has_array_index: bool,
    pub has_property_guid: bool,
    pub has_property_extensions: bool,
    pub has_binary_or_native_serialize: bool,
    pub bool_true: bool,
    #[skip]
    padding: B27,
}

impl PropertyTagFlags {
    fn read_bytes(&self) -> [u8; 4] {
        self.into_bytes()
    }
}

#[binrw]
#[derive(Debug)]
pub struct TypeTree {
    pub name: FString,
    pub child_count: u32,
    #[br(count = child_count)]
    pub children: Vec<TypeTree>,
}

#[binrw]
#[br(little)]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FPropertyTag {
    #[br(pre_assert(!options.property_tag_complete_type_name))]
    Incomplete {
        name: FString,
        _type: FString,
    },
    #[br(pre_assert(options.property_tag_complete_type_name))]
    Complete {
        flags: PropertyTagFlags,
        name: FString,
        _type: TypeTree,
    },
}
