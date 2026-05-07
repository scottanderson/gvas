use binrw::binrw;

use crate::types::FString;

#[binrw]
#[derive(Debug)]
pub struct StrProperty(pub FString);
