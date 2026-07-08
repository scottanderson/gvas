#![allow(clippy::expect_used)]

use binrw::binrw;
use modular_bitfield::{bitfield, prelude::B3};

#[bitfield]
#[binrw]
#[br(map = Self::from_bytes)]
#[bw(map = |&x| Self::into_bytes(x))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EPropertyTagFlags {
    pub has_array_index: bool,
    pub has_property_guid: bool,
    pub has_property_extensions: bool,
    pub has_binary_or_native_serialize: bool,
    pub bool_true: bool,
    #[skip]
    padding: B3,
}
