use crate::{
    format::SerializationFormat,
    types::{EPropertyTagFlags, FProperty, PropertyType, TArray},
};
use binrw::binrw;

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyType))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug)]
pub enum FMapProperty {
    #[br(pre_assert(!t.flags().map(EPropertyTagFlags::has_binary_or_native_serialize).unwrap_or(false)))]
    Known {
        allocation_flags: u32,

        #[br(calc = t.map_key_type().expect("key_type"))]
        #[bw(ignore)]
        key_type: PropertyType,

        #[br(calc = t.map_value_type().expect("value_type"))]
        #[bw(ignore)]
        value_type: PropertyType,

        #[br(args(format, &key_type, &value_type))]
        #[bw(args(format))]
        properties: TArray<MapEntry>,
    },
    Unknown {
        #[br(calc = t.map_key_type().expect("key_type"))]
        #[bw(ignore)]
        key_type: PropertyType,

        #[br(calc = t.map_value_type().expect("value_type"))]
        #[bw(ignore)]
        value_type: PropertyType,

        #[br(count = t.size())]
        data: Vec<u8>,
    },
}

#[binrw]
#[br(import(format: SerializationFormat, key_type: &PropertyType, value_type: &PropertyType))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug)]
pub struct MapEntry {
    #[br(args(format, key_type))]
    #[bw(args(format))]
    pub key: FProperty,

    #[br(args(format, value_type))]
    #[bw(args(format))]
    pub value: FProperty,
}
