use binrw::binrw;

use crate::types::{FEnumProperty, PropertyTag};

#[binrw]
#[br(import(t: &PropertyTag))]
#[derive(Debug, PartialEq)]
pub enum FByteProperty {
    #[br(pre_assert(t.size() <= 1))]
    Byte(u8),
    #[br(pre_assert(t.size() > 1))]
    Enum(#[br(args(t))] FEnumProperty),
}
