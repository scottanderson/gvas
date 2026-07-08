use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{FProperty, PropertyTag, TArray},
};

#[binrw]
#[br(import(format: &SerializationFormat, t: &PropertyTag))]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FSetProperty {
    allocation_flags: u32,
    #[br(try_calc = t.set_element_tag())]
    #[bw(ignore)]
    element_type: PropertyTag,
    #[br(args(format, &element_type))]
    #[bw(args(format))]
    properties: TArray<FProperty>,
}

impl FSetProperty {
    pub(crate) fn element_property_type_name(&self) -> &str {
        self.element_type
            .property_type_str()
            .unwrap_or_else(|_| todo!("{:?}", self.element_type))
    }
}
