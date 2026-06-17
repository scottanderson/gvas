#![allow(unused)]

mod f_custom_version;
mod f_engine_version;
mod f_package_file_version;
mod f_property_tag;
mod f_property_type_name;
mod f_save_game_header;
mod f_string;
mod f_text;
mod t_array;
mod t_option;

use binrw::binrw;

use crate::options::ParsingOptions;
pub use crate::types::{
    f_custom_version::{FCustomVersion, FCustomVersionContainer},
    f_engine_version::FEngineVersion,
    f_package_file_version::FPackageFileVersion,
    f_property_tag::{
        CollectionProperties, FPropertyTag, PropertyTagFlags, PropertyType, TaggedProperties,
    },
    f_property_type_name::FPropertyTypeName,
    f_save_game_header::{FSaveGameHeader, SaveGameFileVersion},
    f_string::FString,
    f_text::FText,
    t_array::TArray,
    t_option::TOption,
};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct SaveGameFile {
    pub header: FSaveGameHeader,

    #[br(temp, calc(ParsingOptions::from(&header)))]
    #[bw(calc(ParsingOptions::from(header)))]
    options: ParsingOptions,

    #[brw(if(options.property_tag_complete_type_name))]
    #[br(temp, assert(spacer == 0))]
    #[bw(calc(0))]
    spacer: u8,

    #[brw(args(options))]
    pub properties: TaggedProperties,

    #[br(temp, assert(footer == 0))]
    #[bw(calc(0))]
    footer: u32,
}

#[cfg(test)]
mod test {
    use std::io::{self, Read, Seek};
    use std::{fs::File, io::Cursor, path::Path};

    use binrw::{BinRead, BinWrite};

    use crate::error::Result;
    use crate::types::SaveGameFile;

    fn test_save_game_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<()> {
        // Open
        let mut file = File::open(&path)?;

        // Read
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let result = SaveGameFile::read(&mut cursor)?;
        assert_eq!(len as u64, cursor.stream_position()?);

        // TODO: Write
        // let mut buf2 = vec![0u8; len];
        // let mut cursor2 = Cursor::new(buf2);
        // SaveGameFile::write(&result, &mut cursor2)?;

        // TODO: Compare
        // let buf = cursor.into_inner();
        // let buf2 = cursor2.into_inner();
        // assert!(buf == buf2);

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
