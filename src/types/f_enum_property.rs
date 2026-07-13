use binrw::binrw;

use crate::types::{FPropertyTypeName, FString, PropertyTag};

#[binrw]
#[br(import(t: &PropertyTag))]
#[derive(Debug, PartialEq)]
pub struct FEnumProperty(
    #[br(try_calc = t.enum_type())]
    #[bw(ignore)]
    pub FPropertyTypeName,
    pub FString,
);
