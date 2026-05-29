use binrw::binrw;

use crate::types::{FSaveGameHeader, TArray};

#[binrw]
#[derive(Debug)]
pub struct FCustomVersion {
    pub key: u128,
    pub value: u32,
}

// pub type FCustomVersionArray = Vec<FCustomVersion>;

#[binrw]
#[br(import(custom_version_format: i32))]
#[derive(Debug)]
pub struct FCustomVersionContainer {
    custom_version_format: i32,
    custom_versions: TArray<FCustomVersion>,
}

impl FCustomVersionContainer {
    pub fn get(&self, version: u128) -> u32 {
        self.custom_versions
            .iter()
            .find(|v| v.key == version)
            .map(|v| v.value)
            .unwrap_or(0)
    }
}
