use binrw::BinWrite;

use crate::{
    types::FSaveGameHeader,
    versions::{
        EEditorObjectVersion, EUE5ReleaseStreamObjectVersion, EUnrealEngineObjectUE5Version,
        GUID_EDITOR, GUID_UE5_RELEASE_STREAM,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct ParsingOptions {
    // pub ftext_history_date_timezone: bool,
    pub property_tag_complete_type_name: bool,
    pub fsoftobjectpath_remove_asset_path_fnames: bool,
    pub text_64bit_support: bool,
    pub large_world_coordinates: bool,
    pub include_always_sign: bool,
    pub culture_invariant_stability: bool,
}

impl BinWrite for ParsingOptions {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        _writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        // Required to allow ParsingOptions to be used with bw(calc)
        Ok(())
    }
}

impl From<&FSaveGameHeader> for ParsingOptions {
    fn from(header: &FSaveGameHeader) -> Self {
        // let package_file_version = header.package_file_version.version_ue4();
        let package_file_version_ue5 = header.package_file_version.version_ue5();
        fn get_custom_version(header: &FSaveGameHeader, version: u128) -> u32 {
            match &header.custom_versions {
                Some(container) => container.get(version),
                None => 0,
            }
        }
        let release_version = get_custom_version(header, GUID_UE5_RELEASE_STREAM);
        let editor_version = get_custom_version(header, GUID_EDITOR);
        ParsingOptions {
            // ftext_history_date_timezone: package_file_version
            //     >= EUnrealEngineObjectUE4Version::FtextHistoryDateTimezone as u32,
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
