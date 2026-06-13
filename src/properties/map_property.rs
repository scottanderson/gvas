use crate::{
    options::ParsingOptions,
    properties::Property,
    types::{PropertyType, TArray},
};
use binrw::binrw;

#[binrw]
#[br(import(options: ParsingOptions, t: &PropertyType))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum MapProperty {
    Known {
        allocation_flags: u32,

        #[br(calc = t.map_key_type().expect("key_type"))]
        #[bw(ignore)]
        key_type: PropertyType,

        #[br(calc = t.map_value_type().expect("value_type"))]
        #[bw(ignore)]
        value_type: PropertyType,

        #[br(args(options, &key_type, &value_type))]
        #[bw(args(options))]
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
#[br(import(options: ParsingOptions, key_type: &PropertyType, value_type: &PropertyType))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub struct MapEntry {
    #[br(args(options, key_type))]
    #[bw(args(options))]
    pub key: Property,

    #[br(args(options, value_type))]
    #[bw(args(options))]
    pub value: Property,
}
