use crate::{
    types::FSaveGameHeader,
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
        let package_file_version_ue5 = header.package_file_version.version_ue5();
        let release_version = header.custom_versions.get(GUID_UE5_RELEASE_STREAM);
        let editor_version = header.custom_versions.get(GUID_EDITOR);
        ParsingOptions {
            property_tag_complete_type_name: package_file_version_ue5
                >= EUnrealEngineObjectUE5Version::PropertyTagCompleteTypeName as u32,
            text_64bit_support: release_version
                >= EUE5ReleaseStreamObjectVersion::TextFormatArgumentData64bitSupport as u32,
            large_world_coordinates: release_version
                >= EUE5ReleaseStreamObjectVersion::LargeWorldCoordinates as u32,
            include_always_sign: editor_version
                >= EEditorObjectVersion::AddedAlwaysSignNumberFormattingOption as u32,
            culture_invariant_stability: editor_version
                >= EEditorObjectVersion::CultureInvariantTextSerializationKeyStability as u32,
        }
    }
}
