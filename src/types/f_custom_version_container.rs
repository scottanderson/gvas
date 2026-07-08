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
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FCustomVersionContainer {
    #[br(temp, assert(custom_version_format == ECustomVersionSerializationFormat::Optimized))]
    #[bw(calc(ECustomVersionSerializationFormat::Optimized))]
    custom_version_format: ECustomVersionSerializationFormat,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::custom_version_map"))]
    pub custom_versions: FCustomVersionArray,
}

impl FCustomVersionContainer {
    pub fn get(&self, version: FGuid) -> u32 {
        self.custom_versions
            .iter()
            .find(|v| v.key == version)
            .map_or(0, |v| v.value)
    }

    #[inline]
    pub fn get_custom<V: CustomVersion>(&self) -> u32 {
        self.get(V::GUID)
    }
}
