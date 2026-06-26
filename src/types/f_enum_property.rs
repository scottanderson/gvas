use binrw::binrw;

use crate::types::{FPropertyTypeName, FString, PropertyTag};

#[binrw]
#[br(import(t: &PropertyTag))]
#[derive(Debug)]
pub struct FEnumProperty(
    #[br(calc = t.enum_type())]
    #[bw(ignore)]
    pub FPropertyTypeName,
    pub FString,
);
