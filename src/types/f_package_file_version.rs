use binrw::binrw;

use crate::types::SaveGameFileVersion;

const UE5_VERSION: u32 = SaveGameFileVersion::PackageFileSummaryVersionChange as u32;

#[binrw]
#[br(import(save_game_file_version: u32))]
#[derive(Debug)]
pub enum FPackageFileVersion {
    #[br(pre_assert(save_game_file_version < UE5_VERSION))]
    UE4 {
        package_file_version: u32, //EUnrealEngineObjectUE4Version,
    },
    #[br(pre_assert(save_game_file_version >= UE5_VERSION))]
    UE5 {
        package_file_version: u32,     //EUnrealEngineObjectUE4Version,
        package_file_version_ue5: u32, //EUnrealEngineObjectUE5Version,
    },
}

impl FPackageFileVersion {
    pub fn version_ue4(&self) -> u32 {
        match self {
            FPackageFileVersion::UE4 {
                package_file_version,
            } => *package_file_version,
            FPackageFileVersion::UE5 {
                package_file_version,
                ..
            } => *package_file_version,
        }
    }

    pub fn version_ue5(&self) -> u32 {
        match self {
            FPackageFileVersion::UE4 { .. } => 0,
            FPackageFileVersion::UE5 {
                package_file_version_ue5,
                ..
            } => *package_file_version_ue5,
        }
    }
}
