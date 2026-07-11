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
    package_file_version: u32,     // EUnrealEngineObjectUE4Version,
    package_file_version_ue5: u32, // Option<EUnrealEngineObjectUE5Version>,
    release_version: u32,          // EUE5ReleaseStreamObjectVersion,
    editor_version: u32,           // EEditorObjectVersion,
}

impl SerializationFormat {
    #[cfg_attr(not(test), allow(unused))]
    #[inline]
    pub(crate) const fn from_enums(
        package_file_version: EUnrealEngineObjectUE4Version,
        package_file_version_ue5: Option<EUnrealEngineObjectUE5Version>,
        release_version: EUE5ReleaseStreamObjectVersion,
        editor_version: EEditorObjectVersion,
    ) -> Self {
        Self::from_versions(
            package_file_version as u32,
            match package_file_version_ue5 {
                Some(e) => e as u32,
                None => 0,
            },
            release_version as u32,
            editor_version as u32,
        )
    }

    #[inline]
    const fn from_versions(
        package_file_version: u32,
        package_file_version_ue5: u32,
        release_version: u32,
        editor_version: u32,
    ) -> Self {
        Self {
            package_file_version,
            package_file_version_ue5,
            release_version,
            editor_version,
        }
    }

    #[inline]
    pub fn ftext_history_date_timezone(&self) -> bool {
        self.package_file_version >= EUnrealEngineObjectUE4Version::FtextHistoryDateTimezone as u32
    }

    #[inline]
    pub fn property_tag_set_map_support(&self) -> bool {
        self.package_file_version >= EUnrealEngineObjectUE4Version::PropertyTagSetMapSupport as u32
    }

    #[inline]
    pub fn property_guid_in_property_tag(&self) -> bool {
        self.package_file_version >= EUnrealEngineObjectUE4Version::PropertyGuidInPropertyTag as u32
    }

    #[inline]
    pub fn property_tag_complete_type_name(&self) -> bool {
        self.package_file_version_ue5
            >= EUnrealEngineObjectUE5Version::PropertyTagCompleteTypeName as u32
    }

    #[inline]
    pub fn fsoftobjectpath_remove_asset_path_fnames(&self) -> bool {
        self.package_file_version_ue5
            >= EUnrealEngineObjectUE5Version::FsoftobjectpathRemoveAssetPathFnames as u32
    }

    #[inline]
    pub fn text_64bit_support(&self) -> bool {
        self.release_version
            >= EUE5ReleaseStreamObjectVersion::TextFormatArgumentData64bitSupport as u32
    }

    #[inline]
    pub fn large_world_coordinates(&self) -> bool {
        self.release_version >= EUE5ReleaseStreamObjectVersion::LargeWorldCoordinates as u32
    }

    #[inline]
    pub fn include_always_sign(&self) -> bool {
        self.editor_version >= EEditorObjectVersion::AddedAlwaysSignNumberFormattingOption as u32
    }

    #[inline]
    pub fn culture_invariant_stability(&self) -> bool {
        self.editor_version
            >= EEditorObjectVersion::CultureInvariantTextSerializationKeyStability as u32
    }
}

impl FSaveGameHeader {
    #[inline]
    pub fn serialization_format(&self) -> SerializationFormat {
        fn get_custom<T: CustomVersion>(header: &FSaveGameHeader) -> u32 {
            match &header.custom_versions {
                Some(container) => container.get_custom::<T>(),
                None => 0,
            }
        }
        let package_file_version = self.package_file_version.version_ue4();
        let package_file_version_ue5 = self.package_file_version.version_ue5();
        let release_version = get_custom::<EUE5ReleaseStreamObjectVersion>(self);
        let editor_version = get_custom::<EEditorObjectVersion>(self);
        SerializationFormat::from_versions(
            package_file_version,
            package_file_version_ue5,
            release_version,
            editor_version,
        )
    }
}
