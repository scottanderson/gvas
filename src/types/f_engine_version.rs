use binrw::binrw;

use crate::types::FString;

#[binrw]
#[derive(Debug)]
pub struct FEngineVersion {
    major: u16,
    minor: u16,
    patch: u16,
    change_list: u32,
    branch: FString,
}
