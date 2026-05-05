use binrw::{BinRead, binrw};

use crate::{
    enums::{
        EEditorObjectVersion, EUE5ReleaseStreamObjectVersion, EUnrealEngineObjectUE5Version,
        GUID_EDITOR, GUID_UE5_RELEASE_STREAM,
    },
    types::{FCustomVersion, FEngineVersion, FPropertyTag, FString, PropertyTagList},
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ParsingOptions {
    pub property_tag_complete_type_name: bool,
    pub text_64bit_support: bool,
    pub large_world_coordinates: bool,
    pub include_always_sign: bool,
    pub culture_invariant_stability: bool,
}

impl From<&FSaveGameHeader> for ParsingOptions {
    fn from(header: &FSaveGameHeader) -> Self {
        let property_tag_complete_type_name = header.save_game_file_version
            >= SaveGameFileVersion::PackageFileSummaryVersionChange as u32
            && header.package_file_version_ue5
                >= EUnrealEngineObjectUE5Version::PropertyTagCompleteTypeName as u32;

        let release_version = header.custom_versions.get(GUID_UE5_RELEASE_STREAM);
        let text_64bit_support = release_version
            >= EUE5ReleaseStreamObjectVersion::TextFormatArgumentData64bitSupport as u32;
        let large_world_coordinates =
            release_version >= EUE5ReleaseStreamObjectVersion::LargeWorldCoordinates as u32;

        let editor_version = header.custom_versions.get(GUID_EDITOR);
        let include_always_sign =
            editor_version >= EEditorObjectVersion::AddedAlwaysSignNumberFormattingOption as u32;
        let culture_invariant_stability = editor_version
            >= EEditorObjectVersion::CultureInvariantTextSerializationKeyStability as u32;

        ParsingOptions {
            property_tag_complete_type_name,
            text_64bit_support,
            large_world_coordinates,
            include_always_sign,
            culture_invariant_stability,
        }
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct SaveGameFile {
    pub header: FSaveGameHeader,

    #[br(calc(ParsingOptions::from(&header)))]
    #[bw(ignore)]
    options: ParsingOptions,

    #[br(args(options))]
    pub first: PropertyTagList,

    #[br(assert(footer == 0))]
    pub footer: u32,
}
