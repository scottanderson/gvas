use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{FProperty, FPropertyTypeName, PropertyTag, TArray},
};

#[binrw]
#[br(import(format: &SerializationFormat, t: &PropertyTag))]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FSetProperty {
    pub allocation_flags: u32,

    #[br(try_calc = t.set_element_type())]
    #[bw(ignore)]
    pub element_type: FPropertyTypeName,

    #[br(args(format, &element_type.as_tag(format, 0)))]
    #[bw(args(format))]
    pub properties: TArray<FProperty>,
}
