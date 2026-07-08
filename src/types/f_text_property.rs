use binrw::binrw;

use crate::{format::SerializationFormat, types::FText};

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FTextProperty(#[brw(args(format))] pub FText);
