use binrw::binrw;

use crate::types::{FString, PropertyType};

#[binrw]
#[br(import(t: &PropertyType))]
#[derive(Debug)]
pub enum FByteProperty {
    #[br(pre_assert(t.size() <= 1))]
    Byte(u8),
    #[br(pre_assert(t.size() > 1))]
    Enum(
        #[br(calc = t.enum_name().into())]
        #[bw(ignore)]
        FString,
        FString,
    ),
}
