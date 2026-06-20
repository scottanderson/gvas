use binrw::binrw;

use crate::types::{FCustomVersion, FGuid, TArray};

#[binrw]
#[brw(repr = i32)]
#[derive(Debug, PartialEq, Eq)]
enum ECustomVersionSerializationFormat {
    Unknown,
    Guids,
    Enums,
    Optimized,
}

pub type FCustomVersionArray = TArray<FCustomVersion>;

pub trait CustomVersion {
    const GUID: FGuid;
}

#[binrw]
#[derive(Debug)]
pub struct FCustomVersionContainer {
    #[br(temp, assert(custom_version_format == ECustomVersionSerializationFormat::Optimized))]
    #[bw(calc(ECustomVersionSerializationFormat::Optimized))]
    custom_version_format: ECustomVersionSerializationFormat,
    custom_versions: FCustomVersionArray,
}

impl FCustomVersionContainer {
    pub fn get(&self, version: FGuid) -> u32 {
        self.custom_versions
            .iter()
            .find(|v| v.key == version)
            .map(|v| v.value)
            .unwrap_or(0)
    }

    pub fn get_custom<V: CustomVersion>(&self) -> u32 {
        self.get(V::GUID)
    }
}
