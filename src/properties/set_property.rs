use crate::{
    options::ParsingOptions,
    properties::Property,
    types::{PropertyType, StaticArray},
};
use binrw::binrw;

#[binrw]
#[br(import(options: ParsingOptions, t: &PropertyType))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum SetProperty {
    Known {
        allocation_flags: u32,
        #[br(args(options, &t.set_element_type()))]
        #[bw(args(options))]
        properties: StaticArray<Property>,
    },
    Unknown(#[br(count = t.size())] Vec<u8>),
}
