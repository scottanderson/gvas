use binrw::binrw;

use crate::types::{FCustomVersionContainer, FEngineVersion, FPackageFileVersion, FString};

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

    #[br(if(save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    #[bw(if(*save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    pub custom_versions: Option<FCustomVersionContainer>,

    pub save_game_class_name: FString,
}

#[cfg(test)]
mod test {
    use std::io::{Read, Seek};
    use std::{fs::File, io::Cursor, path::Path};

    use binrw::{BinRead, BinWrite};

    use crate::error::Result;
    use crate::types::FSaveGameHeader;

    fn test_save_game_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<()> {
        // Open
        let mut file = File::open(&path)?;

        // Read
        let mut buf = Vec::new();
        let _len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let result = FSaveGameHeader::read(&mut cursor)?;

        // Write
        let len = cursor.stream_position()? as usize;
        let buf2 = vec![0u8; len];
        let mut cursor2 = Cursor::new(buf2);
        FSaveGameHeader::write(&result, &mut cursor2)?;

        // Compare
        let buf = cursor.into_inner();
        let buf2 = cursor2.into_inner();
        assert_eq!(&buf[..buf2.len()], &buf2[..]);

        // Success
        Ok(())
    }

    #[test]
    fn test_regression_01_bin() -> Result<()> {
        test_save_game_file("tests/resources/regression_01.bin")
    }

    #[test]
    fn test_options_sav() -> Result<()> {
        test_save_game_file("tests/resources/Options.sav")
    }

    #[test]
    fn test_enum_array_sav() -> Result<()> {
        test_save_game_file("tests/resources/enum_array.sav")
    }

    #[test]
    fn test_component8_sav() -> Result<()> {
        test_save_game_file("tests/resources/component8.sav")
    }

    #[test]
    fn test_complete_property_tag_sav() -> Result<()> {
        test_save_game_file("tests/resources/complete_property_tag.sav")
    }

    #[test]
    fn test_slot2_sav() -> Result<()> {
        test_save_game_file("tests/resources/Slot2.sav")
    }

    #[test]
    fn test_features_01_bin() -> Result<()> {
        test_save_game_file("tests/resources/features_01.bin")
    }

    #[test]
    fn test_profile_0_sav() -> Result<()> {
        test_save_game_file("tests/resources/Profile_0.sav")
    }

    #[test]
    fn test_save_slot_03_sav() -> Result<()> {
        test_save_game_file("tests/resources/SaveSlot_03.sav")
    }

    #[test]
    fn test_package_version_524_sav() -> Result<()> {
        test_save_game_file("tests/resources/package_version_524.sav")
    }

    #[test]
    fn test_vector2d_sav() -> Result<()> {
        test_save_game_file("tests/resources/vector2d.sav")
    }

    #[test]
    fn test_delegate_sav() -> Result<()> {
        test_save_game_file("tests/resources/Delegate.sav")
    }

    #[test]
    fn test_slot3_sav() -> Result<()> {
        test_save_game_file("tests/resources/Slot3.sav")
    }

    #[test]
    fn test_assert_failed_sav() -> Result<()> {
        test_save_game_file("tests/resources/assert_failed.sav")
    }

    #[test]
    fn test_slot1_sav() -> Result<()> {
        test_save_game_file("tests/resources/Slot1.sav")
    }

    #[test]
    fn test_tagcontainer_sav() -> Result<()> {
        test_save_game_file("tests/resources/tagcontainer.sav")
    }

    #[test]
    fn test_string_table_entry_sav() -> Result<()> {
        test_save_game_file("tests/resources/string_table_entry.sav")
    }

    #[test]
    fn test_package_version_525_sav() -> Result<()> {
        test_save_game_file("tests/resources/package_version_525.sav")
    }

    #[test]
    fn test_ro_64bit_fav_sav() -> Result<()> {
        test_save_game_file("tests/resources/ro_64bit_fav.sav")
    }

    #[test]
    fn test_transform_sav() -> Result<()> {
        test_save_game_file("tests/resources/transform.sav")
    }

    #[test]
    fn test_text_property_noarray_bin() -> Result<()> {
        test_save_game_file("tests/resources/text_property_noarray.bin")
    }
}
