use crate::{
    options::ParsingOptions,
    types::{FProperty, PropertyType, TArray},
};
use binrw::binrw;

#[binrw]
#[br(import(options: ParsingOptions, t: &PropertyType))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FSetProperty {
    Known {
        allocation_flags: u32,
        #[br(calc = t.set_element_type().expect("set_element_type"))]
        #[bw(ignore)]
        element_type: PropertyType,

        #[br(args(options, &element_type))]
        #[bw(args(options))]
        properties: TArray<FProperty>,
    },
    Unknown(#[br(count = t.size())] Vec<u8>),
}
