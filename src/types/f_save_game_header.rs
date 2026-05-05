use binrw::{BinRead, binrw};

use crate::{
    types::{FCustomVersion, FEngineVersion, FPropertyTag, FString, TaggedProperties},
    versions::{
        EEditorObjectVersion, EUE5ReleaseStreamObjectVersion, EUnrealEngineObjectUE5Version,
        GUID_EDITOR, GUID_UE5_RELEASE_STREAM,
    },
};

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaveGameFileVersion {
    InitialVersion = 1,
    // serializing custom versions into the savegame data to handle that type of versioning
    AddedCustomVersions = 2,
    // added a new UE5 version number to FPackageFileSummary
    PackageFileSummaryVersionChange = 3,
}

#[binrw]
#[derive(Debug, Default)]
pub struct CustomVersions {
    custom_version_format: i32,

    #[bw(try_calc(i32::try_from(custom_versions.len())))]
    custom_version_length: i32,

    #[br(count = custom_version_length)]
    custom_versions: Vec<FCustomVersion>,
}

impl CustomVersions {
    pub fn get(&self, version: u128) -> u32 {
        self.custom_versions
            .iter()
            .find(|v| v.key == version)
            .map(|v| v.value)
            .unwrap_or(0)
    }
}

#[binrw]
#[brw(little, magic = b"GVAS")]
#[derive(Debug)]
pub struct FSaveGameHeader {
    pub save_game_file_version: u32, //SaveGameFileVersion,

    pub package_file_version: u32, //EUnrealEngineObjectUE4Version,

    #[br(if(save_game_file_version >= SaveGameFileVersion::PackageFileSummaryVersionChange as u32))]
    #[bw(if(*save_game_file_version >= SaveGameFileVersion::PackageFileSummaryVersionChange as u32))]
    pub package_file_version_ue5: u32, //EUnrealEngineObjectUE5Version,

    pub engine_version: FEngineVersion,

    #[br(if(save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    #[bw(if(*save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    pub custom_versions: CustomVersions,

    pub save_game_class_name: FString,
}
