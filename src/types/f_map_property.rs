use crate::{
    format::SerializationFormat,
    types::{EPropertyTagFlags, FProperty, FPropertyTypeName, PropertyTag, TArray},
};
use binrw::binrw;

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyTag))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug, PartialEq)]
pub enum FMapProperty {
    #[br(pre_assert(!t.flags().map(EPropertyTagFlags::has_binary_or_native_serialize).unwrap_or(false)))]
    Known {
        allocation_flags: u32,

        #[br(calc = t.map_key_type()?)]
        #[bw(ignore)]
        key_type: PropertyTag,

        #[br(calc = t.map_value_type()?)]
        #[bw(ignore)]
        value_type: PropertyTag,

        #[br(args(format, &key_type, &value_type))]
        #[bw(args(format))]
        properties: TArray<MapEntry>,
    },
    Unknown {
        #[br(calc = t.map_key_type()?)]
        #[bw(ignore)]
        key_type: PropertyTag,

        #[br(calc = t.map_value_type()?)]
        #[bw(ignore)]
        value_type: PropertyTag,

        #[br(count = t.size())]
        data: Vec<u8>,
    },
}

impl FMapProperty {
    pub fn key_type(&self) -> &FPropertyTypeName {
        let key_type = match self {
            Self::Known { key_type, .. } => key_type,
            Self::Unknown { key_type, .. } => key_type,
        };
        match key_type {
            PropertyTag::Complete { property_type, .. } => property_type,
            _ => todo!("{key_type:?}"),
        }
    }

    pub fn value_type(&self) -> &FPropertyTypeName {
        let value_type = match self {
            Self::Known { value_type, .. } => value_type,
            Self::Unknown { value_type, .. } => value_type,
        };
        match value_type {
            PropertyTag::Complete { property_type, .. } => property_type,
            _ => todo!("{value_type:?}"),
        }
    }
}

#[binrw]
#[br(import(format: SerializationFormat, key_type: &PropertyTag, value_type: &PropertyTag))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug, PartialEq)]
pub struct MapEntry {
    #[br(args(format, key_type))]
    #[bw(args(format))]
    pub key: FProperty,

    #[br(args(format, value_type))]
    #[bw(args(format))]
    pub value: FProperty,
}
