use binrw::binrw;

use crate::types::{FCustomVersion, TArray};

// pub type FCustomVersionArray = TArray<FCustomVersion>;

#[binrw]
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
