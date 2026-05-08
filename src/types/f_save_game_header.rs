use binrw::{BinRead, binrw};

use crate::{
    types::{
        FCustomVersion, FCustomVersionContainer, FEngineVersion, FPackageFileVersion, FPropertyTag,
        FString, TaggedProperties,
    },
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
#[brw(little, magic = b"GVAS")]
#[derive(Debug)]
pub struct FSaveGameHeader {
    pub save_game_file_version: u32, //SaveGameFileVersion,

    #[br(args(save_game_file_version))]
    pub package_file_version: FPackageFileVersion,

    pub engine_version: FEngineVersion,

    custom_version_format: i32,

    #[br(args(custom_version_format))]
    #[br(if(save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    #[bw(if(*save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    pub custom_versions: FCustomVersionContainer,

    pub save_game_class_name: FString,
}
