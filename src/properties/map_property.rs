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
        #[br(args(options, &t.map_key_type(), &t.map_value_type()))]
        #[bw(args(options))]
        properties: TArray<MapEntry>,
    },
    Unknown(#[br(count = t.size())] Vec<u8>),
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
