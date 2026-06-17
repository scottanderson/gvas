use binrw::binrw;

use crate::{options::ParsingOptions, types::FText};

#[binrw]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub struct FTextProperty(#[brw(args(options))] pub FText);
