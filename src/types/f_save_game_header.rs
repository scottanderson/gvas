use binrw::binrw;

use crate::{
    object_verison::{
        EEditorObjectVersion, EUE5ReleaseStreamObjectVersion, GUID_EDITOR, GUID_UE5_RELEASE_STREAM,
    },
    types::{FCustomVersion, FEngineVersion, FString},
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

// #[binrw]
// #[brw(repr = u32)]
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub enum EUnrealEngineObjectUE4Version {
//     OldestLoadablePackage = 214,
// }

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EUnrealEngineObjectUE5Version {
    /// The original UE5 version, at the time this was added the UE4 version was 522, so UE5 will start from 1000 to show a clear difference
    #[default]
    InitialVersion = 1000,

    /// Support stripping names that are not referenced from export data
    NamesReferencedFromExportData,

    /// Added a payload table of contents to the package summary
    PayloadToc,

    /// Added data to identify references from and to optional package
    OptionalResources,

    /// Large world coordinates converts a number of core types to double components by default.
    LargeWorldCoordinates,

    /// Remove package GUID from FObjectExport
    RemoveObjectExportPackageGuid,

    /// Add IsInherited to the FObjectExport entry
    TrackObjectExportIsInherited,

    /// Replace FName asset path in FSoftObjectPath with (package name, asset name) pair FTopLevelAssetPath
    FsoftobjectpathRemoveAssetPathFnames,

    /// Add a soft object path list to the package summary for fast remap
    AddSoftobjectpathList,

    /// Added bulk/data resource table
    DataResources,

    /// Added script property serialization offset to export table entries for saved, versioned packages
    ScriptSerializationOffset,

    /// Adding property tag extension,
    /// Support for overridable serialization on UObject,
    /// Support for overridable logic in containers
    PropertyTagExtensionAndOverridableSerialization,

    /// Added property tag complete type name and serialization type
    PropertyTagCompleteTypeName,

    /// Changed UE::AssetRegistry::WritePackageData to include PackageBuildDependencies
    AssetRegistryPackageBuildDependencies,

    /// Added meta data serialization offset to for saved, versioned packages
    MetadataSerializationOffset,

    /// Added VCells to the object graph
    VerseCells,

    /// Changed PackageFileSummary to write FIoHash PackageSavedHash instead of FGuid Guid
    PackageSavedHash,

    /// OS shadow serialization of subobjects
    OsSubObjectShadowSerialization,
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
    pub fn get(&self, version: u128) -> Option<u32> {
        self.custom_versions
            .iter()
            .find(|v| v.key == version)
            .map(|v| v.value)
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
#[derive(Debug)]
pub struct ParsingOptions {
    pub property_tag_complete_type_name: bool,
    pub text_64bit_support: bool,
    pub large_world_coordinates: bool,
    pub include_always_sign: bool,
    pub culture_invariant_stability: bool,
}

impl From<FSaveGameHeader> for ParsingOptions {
    fn from(header: FSaveGameHeader) -> Self {
        let property_tag_complete_type_name = header.save_game_file_version
            >= SaveGameFileVersion::PackageFileSummaryVersionChange as u32
            && header.package_file_version_ue5
                >= EUnrealEngineObjectUE5Version::PropertyTagCompleteTypeName as u32;

        let release_version = header
            .custom_versions
            .get(GUID_UE5_RELEASE_STREAM)
            .unwrap_or(0);
        let text_64bit_support = release_version
            >= EUE5ReleaseStreamObjectVersion::TextFormatArgumentData64bitSupport as u32;
        let large_world_coordinates =
            release_version >= EUE5ReleaseStreamObjectVersion::LargeWorldCoordinates as u32;

        let editor_version = header.custom_versions.get(GUID_EDITOR).unwrap_or(0);
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
