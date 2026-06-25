use binrw::binrw;

use crate::types::{FString, PropertyType};

#[binrw]
#[br(import(t: &PropertyType))]
#[derive(Debug)]
pub struct FEnumProperty(
    #[br(calc = t.enum_name().into())]
    #[bw(ignore)]
    pub FString,
    pub FString,
);
