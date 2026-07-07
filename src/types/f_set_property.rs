use crate::{
    format::SerializationFormat,
    types::{FProperty, PropertyTag, TArray},
};
use binrw::binrw;

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyTag))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug, PartialEq)]
pub enum FSetProperty {
    Known {
        allocation_flags: u32,
        #[br(calc = t.set_element_type().expect("set_element_type"))]
        #[bw(ignore)]
        element_type: PropertyTag,

        #[br(args(format, &element_type))]
        #[bw(args(format))]
        properties: TArray<FProperty>,
    },
    Unknown(#[br(count = t.size())] Vec<u8>),
}
