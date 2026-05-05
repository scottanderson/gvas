use crate::{
    types::{FSaveGameHeader, SaveGameFileVersion},
    versions::{
        EEditorObjectVersion, EUE5ReleaseStreamObjectVersion, EUnrealEngineObjectUE5Version,
        GUID_EDITOR, GUID_UE5_RELEASE_STREAM,
    },
};

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
