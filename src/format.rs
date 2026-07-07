//! Describes the serialization format used by a save file.
//!
//! Although the GVAS file format is broadly stable across Unreal Engine
//! releases, many individual types have version-dependent binary layouts.
//! Whether a particular field is present, omitted, widened, or serialized
//! differently depends on the package version and custom version GUIDs stored
//! in the [`FSaveGameHeader`].
//!
//! This module derives a [`SerializationFormat`] from an
//! [`FSaveGameHeader`]. The resulting flags describe the serialization
//! behavior expected throughout the rest of the crate, allowing individual
//! types to select the correct binary representation without repeatedly
//! performing version checks.

use crate::types::{
    CustomVersion, EEditorObjectVersion, EUE5ReleaseStreamObjectVersion,
    EUnrealEngineObjectUE4Version, EUnrealEngineObjectUE5Version, FSaveGameHeader,
};

#[derive(Clone, Copy, Debug)]
pub struct SerializationFormat {
    pub ftext_history_date_timezone: bool,
    pub property_tag_set_map_support: bool,
    pub property_guid_in_property_tag: bool,
    pub property_tag_complete_type_name: bool,
    pub fsoftobjectpath_remove_asset_path_fnames: bool,
    pub text_64bit_support: bool,
    pub large_world_coordinates: bool,
    pub include_always_sign: bool,
    pub culture_invariant_stability: bool,
}

impl FSaveGameHeader {
    #[inline]
    pub fn serialization_format(&self) -> SerializationFormat {
        let package_file_version = self.package_file_version.version_ue4();
        let package_file_version_ue5 = self.package_file_version.version_ue5();
        fn get_custom<T: CustomVersion>(header: &FSaveGameHeader) -> u32 {
            match &header.custom_versions {
                Some(container) => container.get_custom::<T>(),
                None => 0,
            }
        }
        let release_version = get_custom::<EUE5ReleaseStreamObjectVersion>(self);
        let editor_version = get_custom::<EEditorObjectVersion>(self);
        SerializationFormat {
            ftext_history_date_timezone: package_file_version
                >= EUnrealEngineObjectUE4Version::FtextHistoryDateTimezone as u32,
            property_tag_set_map_support: package_file_version
                >= EUnrealEngineObjectUE4Version::PropertyTagSetMapSupport as u32,
            property_guid_in_property_tag: package_file_version
                >= EUnrealEngineObjectUE4Version::PropertyGuidInPropertyTag as u32,
            property_tag_complete_type_name: package_file_version_ue5
                >= EUnrealEngineObjectUE5Version::PropertyTagCompleteTypeName as u32,
            fsoftobjectpath_remove_asset_path_fnames: package_file_version_ue5
                >= EUnrealEngineObjectUE5Version::FsoftobjectpathRemoveAssetPathFnames as u32,
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
