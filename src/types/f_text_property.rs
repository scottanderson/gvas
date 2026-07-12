use binrw::binrw;

use crate::{format::SerializationFormat, types::FText};

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
pub struct FTextProperty(#[brw(args(format))] pub FText);
